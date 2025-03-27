use heck::ToSnakeCase;
use proc_macro2::Ident;
use quote::quote;
use syn::{DeriveInput, LitStr, Type};

pub fn into_event(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let event_name = event_name_snake_case(input);
    let fields = event_struct_fields(input);
    let (topic_field_idents, topic_types) = fields.topics;
    let (data_field_idents, data_types) = fields.data;
    let datum = fields.is_datum;

    let topic_type_tokens = topic_types.iter().map(|ty| quote!(#ty));
    let data_type_tokens = data_types.iter().map(|ty| quote!(#ty));

    let emit_impl = quote! {
        fn emit(self, env: &stellar_axelar_std::Env) {
            use stellar_axelar_std::{IntoVal, Symbol, Env, Val, Vec, vec};

            let topics = (Symbol::new(env, #event_name),
                #(IntoVal::<Env, Val>::into_val(&self.#topic_field_idents, env),)*
            );

            let data: Vec<Val> = vec![
                env
                #(, IntoVal::<_, Val>::into_val(&self.#data_field_idents, env))*
            ];

            env.events().publish(topics, data);
        }
    };

    let from_event_impl = quote! {
        fn from_event(env: &stellar_axelar_std::Env,
            topics: stellar_axelar_std::Vec<stellar_axelar_std::Val>, data: stellar_axelar_std::Val) -> Self {
            use stellar_axelar_std::{TryFromVal, Symbol, Val, Vec};

            // Verify the event name matches
            let event_name = Symbol::try_from_val(env, &topics.get(0)
                .expect("missing event name in topics"))
                .expect("invalid event name type");
            assert_eq!(event_name, Symbol::new(env, #event_name), "event name mismatch");

            // Parse topics from Val to the corresponding type,
            // and assign them to a variable with the same name as the struct field
            // E.g. let destination_chain = String::try_from_val(env, &topics.get(1));
            // Start from index 1 because the first topic is the event name
            let mut topic_idx = 1;
            #(
                let #topic_field_idents = <#topic_type_tokens>::try_from_val(env, &topics.get(topic_idx)
                    .expect("the number of topics does not match this function's definition"))
                    .expect("given topic value does not match the expected type");
                topic_idx += 1;
            )*

            // Parse data from Val to the corresponding types,
            // and assign them to a variable with the same name as the struct field
            // E.g. let message = Message::try_from_val(env, &data.get(0));
            let data = if #datum {
                let mut vec = Vec::<Val>::new(env);
                vec.push_back(data);
                vec
            } else {
                Vec::<Val>::try_from_val(env, &data)
                    .expect("invalid data format")
            };

            let mut data_idx = 0;
            #(
                let #data_field_idents = <#data_type_tokens>::try_from_val(env, &data.get(data_idx)
                    .expect("the number of data entries does not match this function's definition"))
                    .expect("given data value does not match the expected type");
                data_idx += 1;
            )*

            // Construct the struct from the parsed topics and data.
            // Since the variables created above have the same name as the struct fields,
            // the compiler will automatically assign the values to the struct fields.
            Self {
                #(#topic_field_idents,)*
                #(#data_field_idents,)*
            }
        }
    };

    let schema_impl = quote! {
        fn schema(env: &stellar_axelar_std::Env) -> &'static str {
            concat!(
                #event_name, " {\n",
                #(
                    "    #[topic] ",
                    stringify!(#topic_field_idents),
                    ": ",
                    stringify!(#topic_types),
                    ",\n",
                )*
                #(
                    "    #[data]  ",
                    stringify!(#data_field_idents),
                    ": ",
                    stringify!(#data_types),
                    ",\n",
                )*
                "}"
            )
        }
    };

    quote! {
        impl stellar_axelar_std::events::Event for #name {
            #emit_impl

            #from_event_impl

            #schema_impl
        }
    }
}

fn event_name_snake_case(input: &DeriveInput) -> String {
    input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("event_name"))
        .map(|attr| attr.parse_args::<LitStr>().unwrap().value())
        .unwrap_or_else(|| {
            input
                .ident
                .to_string()
                .strip_suffix("Event")
                .unwrap()
                .to_snake_case()
        })
}

type EventIdent<'a> = Vec<&'a Ident>;
type EventType<'a> = Vec<&'a Type>;
type EventStructFields<'a> = (EventIdent<'a>, EventType<'a>);

struct EventFields<'a> {
    topics: EventStructFields<'a>,
    data: EventStructFields<'a>,
    is_datum: bool,
}

fn event_struct_fields(input: &DeriveInput) -> EventFields {
    let syn::Data::Struct(data_struct) = &input.data else {
        panic!("IntoEvent can only be derived for structs");
    };

    let mut topic_idents = Vec::new();
    let mut topic_types = Vec::new();
    let mut data_idents = Vec::new();
    let mut data_types = Vec::new();
    let mut datum_count = 0;

    for field in data_struct.fields.iter() {
        if let Some(ident) = field.ident.as_ref() {
            if field.attrs.iter().any(|attr| attr.path().is_ident("data")) {
                data_idents.push(ident);
                data_types.push(&field.ty);
            } else if field.attrs.iter().any(|attr| attr.path().is_ident("datum")) {
                if datum_count == 0 {
                    datum_count = 1;
                } else {
                    panic!("Only one field can have the #[datum] attribute");
                }
                data_idents.push(ident);
                data_types.push(&field.ty);
            } else {
                topic_idents.push(ident);
                topic_types.push(&field.ty);
            }
        }
    }

    EventFields {
        topics: (topic_idents, topic_types),
        data: (data_idents, data_types),
        is_datum: datum_count == 1,
    }
}
