use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::ItemFn;

use crate::modifier::modifier_impl;
use crate::utils::parse_env_ident;

pub fn operatable(name: &Ident) -> TokenStream2 {
    quote! {
        use stellar_axelar_std::interfaces::OperatableInterface as _;

        #[stellar_axelar_std::contractimpl]
        impl stellar_axelar_std::interfaces::OperatableInterface for #name {
            fn operator(env: &Env) -> soroban_sdk::Address {
                stellar_axelar_std::interfaces::operator(env)
            }

            fn transfer_operatorship(env: &Env, new_operator: soroban_sdk::Address) {
                stellar_axelar_std::interfaces::transfer_operatorship::<Self>(env, new_operator);
            }
        }
    }
}
pub fn only_operator_impl(input_fn: ItemFn) -> TokenStream2 {
    let env_ident = parse_env_ident(&input_fn.sig.inputs)
        .expect("non-empty contract endpoints must have an Env argument");

    modifier_impl(
        &input_fn,
        quote! {
            Self::operator(&#env_ident).require_auth();
        },
    )
}
