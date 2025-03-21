use proc_macro2::{Ident, TokenStream as TokenStream2, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemFn};

use crate::utils::{parse_env_identifier, PrependStatement};

pub fn operatable(name: &Ident) -> TokenStream2 {
    quote! {
        use stellar_axelar_std::interfaces::OperatableInterface as _;

        #[stellar_axelar_std::contractimpl]
        impl stellar_axelar_std::interfaces::OperatableInterface for #name {
            #[allow_during_migration]
            fn operator(env: &Env) -> stellar_axelar_std::Address {
                stellar_axelar_std::interfaces::operator(env)
            }

            fn transfer_operatorship(env: &Env, new_operator: stellar_axelar_std::Address) {
                stellar_axelar_std::interfaces::transfer_operatorship::<Self>(env, new_operator);
            }
        }
    }
}
pub fn only_operator_impl(mut input_fn: ItemFn) -> Result<TokenStream, syn::Error> {
    let env_ident = parse_env_identifier(&input_fn.sig.inputs)?;
    let auth_stmt = parse_quote!(Self::operator(&#env_ident).require_auth(););

    Ok(input_fn.prepend_statement(auth_stmt).into_token_stream())
}
