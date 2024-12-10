use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, Error, Ident, Token, Type,
};

/// Implements the Ownable interface for a Soroban contract.
///
/// # Example
/// ```rust
/// # mod test {
/// # use soroban_sdk::{contract, contractimpl, Address, Env};
/// use axelar_soroban_std_derive::Ownable;
///
/// #[contract]
/// #[derive(Ownable)]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     pub fn __constructor(env: &Env, owner: Address) {
///         axelar_soroban_std::interfaces::set_owner(env, &owner);
///     }
/// }
/// # }
/// ```
#[proc_macro_derive(Ownable)]
pub fn derive_ownable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        use ::axelar_soroban_std::interfaces::OwnableInterface;

        #[::soroban_sdk::contractimpl]
        impl ::axelar_soroban_std::interfaces::OwnableInterface for #name {
            fn owner(env: &Env) -> ::soroban_sdk::Address {
                ::axelar_soroban_std::interfaces::owner(env)
            }

            fn transfer_ownership(env: &Env, new_owner: ::soroban_sdk::Address) {
                ::axelar_soroban_std::interfaces::transfer_ownership::<Self>(env, new_owner);
            }
        }
    };
    TokenStream::from(expanded)
}

struct UpgradableArgs {
    migration_data: Option<Type>,
}

impl Parse for UpgradableArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                migration_data: None,
            });
        }

        let ident: Ident = input.parse()?;
        if ident != "migration_data" {
            return Err(Error::new(ident.span(), "expected `migration_data=..`"));
        }

        input.parse::<Token![=]>()?;
        let migration_data = Some(input.parse::<Type>()?);

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self { migration_data })
    }
}

/// Implements the Upgradable and Migratable interfaces for a Soroban contract.
///
/// A `ContractError` error type must be defined in scope, and have a `MigrationNotAllowed` variant.
///
/// # Example
/// ```rust
/// # mod test {
/// # use soroban_sdk::{contract, contractimpl, contracterror, Address, Env};
/// use axelar_soroban_std_derive::{Ownable, Upgradable};
/// # #[contracterror]
/// # #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// # #[repr(u32)]
/// # pub enum ContractError {
/// #     MigrationNotAllowed = 1,
/// # }
///
/// #[contract]
/// #[derive(Ownable, Upgradable)]
/// #[upgradable(migration_data = ())]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     pub fn __constructor(env: &Env, owner: Address) {
///         axelar_soroban_std::interfaces::set_owner(env, &owner);
///     }
/// }
///
/// impl Contract {
///     fn run_migration(env: &Env, _migration_data: ()) {}
/// }
/// # }
/// ```
#[proc_macro_derive(Upgradable, attributes(upgradable))]
pub fn derive_upgradable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let args = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("upgradable"))
        .map(|attr| attr.parse_args::<UpgradableArgs>())
        .transpose()
        .unwrap_or_else(|e| panic!("{}", e))
        .unwrap_or_else(|| UpgradableArgs {
            migration_data: None,
        });

    syn::parse_str::<syn::Type>("ContractError")
        .map_err(|_| {
            syn::Error::new(
                name.span(),
                "ContractError must be defined in scope.\n\
                 Hint: Add this to your code:\n\
                 #[contracterror]\n\
                 #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]\n\
                 #[repr(u32)]\n\
                 pub enum ContractError {\n    \
                     MigrationNotAllowed = 1,\n\
                     ...\n
                 }",
            )
        })
        .unwrap_or_else(|e| panic!("{}", e));

    let migration_data = args
        .migration_data
        .as_ref()
        .map_or_else(|| quote! { () }, |ty| quote! { #ty });

    let expanded = quote! {
        use ::axelar_soroban_std::interfaces::{UpgradableInterface, MigratableInterface};

        #[::soroban_sdk::contractimpl]
        impl ::axelar_soroban_std::interfaces::UpgradableInterface for #name {
            fn version(env: &Env) -> ::soroban_sdk::String {
                ::soroban_sdk::String::from_str(env, env!("CARGO_PKG_VERSION"))
            }

            fn upgrade(env: &Env, new_wasm_hash: ::soroban_sdk::BytesN<32>) {
                ::axelar_soroban_std::interfaces::upgrade::<Self>(env, new_wasm_hash);
            }
        }

        #[::soroban_sdk::contractimpl]
        impl ::axelar_soroban_std::interfaces::MigratableInterface for #name {
            type MigrationData = #migration_data;
            type Error = ContractError;

            fn migrate(env: &Env, migration_data: #migration_data) -> Result<(), ContractError> {
                ::axelar_soroban_std::interfaces::migrate::<Self>(env, || Self::run_migration(env, migration_data))
                    .map_err(|_| ContractError::MigrationNotAllowed)
            }
        }
    };
    TokenStream::from(expanded)
}
