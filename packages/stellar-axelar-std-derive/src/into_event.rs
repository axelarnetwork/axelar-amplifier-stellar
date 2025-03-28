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
    let has_datum = fields.has_datum;

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

            if #has_datum {
                env.events().publish(topics, data.get(0));
            } else {
                env.events().publish(topics, data);
            }
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
            let data = if #has_datum {
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
    has_datum: bool,
}

impl<'a> EventFields<'a> {
    fn add_field(&mut self, ident: &'a Ident, ty: &'a Type, field: &syn::Field) {
        match field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("data") || attr.path().is_ident("datum"))
        {
            /* datum */
            Some(attr) if attr.path().is_ident("datum") => {
                if self.has_datum {
                    panic!("Only one field can have the #[datum] attribute");
                }
                self.data.0.push(ident);
                self.data.1.push(ty);
                self.has_datum = true;
            }
            /* data */
            Some(_) => {
                self.data.0.push(ident);
                self.data.1.push(ty);
            }
            /* topic */
            None => {
                self.topics.0.push(ident);
                self.topics.1.push(ty);
            }
        }
    }
}

fn event_struct_fields(input: &DeriveInput) -> EventFields {
    let syn::Data::Struct(data_struct) = &input.data else {
        panic!("IntoEvent can only be derived for structs");
    };

    let mut fields = EventFields {
        topics: (Vec::new(), Vec::new()),
        data: (Vec::new(), Vec::new()),
        has_datum: false,
    };

    for field in data_struct.fields.iter() {
        if let Some(ident) = field.ident.as_ref() {
            fields.add_field(ident, &field.ty, field);
        }
    }

    fields
}
