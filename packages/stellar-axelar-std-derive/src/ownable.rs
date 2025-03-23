use proc_macro2::{Ident, TokenStream as TokenStream2, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemFn};

use crate::utils::{parse_env_identifier, PrependStatement};

pub fn ownable(name: &Ident) -> TokenStream2 {
    quote! {
        use stellar_axelar_std::interfaces::OwnableInterface as _;

        #[stellar_axelar_std::contractimpl]
        impl stellar_axelar_std::interfaces::OwnableInterface for #name {
            #[allow_during_migration]
            fn owner(env: &Env) -> stellar_axelar_std::Address {
                stellar_axelar_std::interfaces::owner(env)
            }

            fn transfer_ownership(env: &Env, new_owner: stellar_axelar_std::Address) {
                stellar_axelar_std::interfaces::transfer_ownership::<Self>(env, new_owner);
            }
        }
    }
}

pub fn only_owner_impl(mut input_fn: ItemFn) -> Result<TokenStream, syn::Error> {
    let env_ident = parse_env_identifier(&input_fn.sig.inputs)?;
    let auth_stmt = parse_quote!(Self::owner(&#env_ident).require_auth(););

    Ok(input_fn.prepend_statement(auth_stmt).into_token_stream())
}
