use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::ItemFn;

use crate::modifier::modifier_impl;
use crate::utils::parse_env_ident;

pub fn ownable(name: &Ident) -> TokenStream2 {
    quote! {
        use stellar_axelar_std::interfaces::OwnableInterface as _;

        #[stellar_axelar_std::contractimpl]
        impl stellar_axelar_std::interfaces::OwnableInterface for #name {
            fn owner(env: &Env) -> soroban_sdk::Address {
                stellar_axelar_std::interfaces::owner(env)
            }

            fn transfer_ownership(env: &Env, new_owner: soroban_sdk::Address) {
                stellar_axelar_std::interfaces::transfer_ownership::<Self>(env, new_owner);
            }
        }
    }
}

pub fn only_owner_impl(input_fn: ItemFn) -> TokenStream2 {
    let env_ident = parse_env_ident(&input_fn.sig.inputs)
        .expect("non-empty contract endpoints must have an Env argument");

    modifier_impl(
        &input_fn,
        quote! {
            Self::owner(&#env_ident).require_auth();
        },
    )
}
