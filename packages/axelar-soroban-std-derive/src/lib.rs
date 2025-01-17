use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Error, Ident, Token, Type};

/// Implements the Operatable interface for a Soroban contract.
///
/// # Example
/// ```rust
/// # mod test {
/// # use soroban_sdk::{contract, contractimpl, Address, Env};
/// use axelar_soroban_std_derive::Operatable;
///
/// #[contract]
/// #[derive(Operatable)]
/// pub struct Contract;
///
/// #[contractimpl]
/// impl Contract {
///     pub fn __constructor(env: &Env, owner: Address) {
///         axelar_soroban_std::interfaces::set_operator(env, &owner);
///     }
/// }
/// # }
/// ```
#[proc_macro_derive(Operatable)]
pub fn derive_operatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    operatable(name).into()
}

fn operatable(name: &Ident) -> TokenStream2 {
    quote! {
        use axelar_soroban_std::interfaces::OperatableInterface as _;

        #[soroban_sdk::contractimpl]
        impl axelar_soroban_std::interfaces::OperatableInterface for #name {
            fn operator(env: &Env) -> soroban_sdk::Address {
                axelar_soroban_std::interfaces::operator(env)
            }

            fn transfer_operatorship(env: &Env, new_operator: soroban_sdk::Address) {
                axelar_soroban_std::interfaces::transfer_operatorship::<Self>(env, new_operator);
            }
        }
    }
}

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

    ownable(name).into()
}

fn ownable(name: &Ident) -> TokenStream2 {
    quote! {
        use axelar_soroban_std::interfaces::OwnableInterface as _;

        #[soroban_sdk::contractimpl]
        impl axelar_soroban_std::interfaces::OwnableInterface for #name {
            fn owner(env: &Env) -> soroban_sdk::Address {
                axelar_soroban_std::interfaces::owner(env)
            }

            fn transfer_ownership(env: &Env, new_owner: soroban_sdk::Address) {
                axelar_soroban_std::interfaces::transfer_ownership::<Self>(env, new_owner);
            }
        }
    }
}

#[derive(Debug, Default)]
struct MigrationArgs {
    migration_data: Option<Type>,
}

impl Parse for MigrationArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let migration_data = Some(Self::parse_migration_data(input)?);

        if !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        Ok(Self { migration_data })
    }
}

impl MigrationArgs {
    fn parse_migration_data(input: ParseStream) -> syn::Result<Type> {
        let ident = input.parse::<Ident>()?;
        if ident != "with_type" {
            return Err(Error::new(ident.span(), "expected `with_type = ...`"));
        }

        input.parse::<Token![=]>()?;
        input.parse::<Type>()
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
/// #[migratable(with_type = Address)]
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
///     fn run_migration(env: &Env, new_owner: Address) {
///         Self::transfer_ownership(env, new_owner);
///     }
/// }
/// # }
/// ```
#[proc_macro_derive(Upgradable, attributes(migratable))]
pub fn derive_upgradable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let args = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("migratable"))
        .map(|attr| attr.parse_args::<MigrationArgs>())
        .transpose()
        .unwrap_or_else(|e| panic!("{}", e))
        .unwrap_or_else(MigrationArgs::default);

    upgradable(name, args).into()
}

fn upgradable(name: &Ident, args: MigrationArgs) -> TokenStream2 {
    syn::parse_str::<Type>("ContractError").unwrap_or_else(|_| {
        panic!(
            "{}",
            Error::new(
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
            .to_string()
        )
    });

    let migration_data = args
        .migration_data
        .as_ref()
        .map_or_else(|| quote! { () }, |ty| quote! { #ty });

    quote! {
        use axelar_soroban_std::interfaces::{UpgradableInterface as _, MigratableInterface as _};

        #[soroban_sdk::contractimpl]
        impl axelar_soroban_std::interfaces::UpgradableInterface for #name {
            fn version(env: &Env) -> soroban_sdk::String {
                soroban_sdk::String::from_str(env, env!("CARGO_PKG_VERSION"))
            }

            fn upgrade(env: &Env, new_wasm_hash: soroban_sdk::BytesN<32>) {
                axelar_soroban_std::interfaces::upgrade::<Self>(env, new_wasm_hash);
            }
        }

        #[soroban_sdk::contractimpl]
        impl axelar_soroban_std::interfaces::MigratableInterface for #name {
            type MigrationData = #migration_data;
            type Error = ContractError;

            fn migrate(env: &Env, migration_data: #migration_data) -> Result<(), ContractError> {
                axelar_soroban_std::interfaces::migrate::<Self>(env, || Self::run_migration(env, migration_data))
                    .map_err(|_| ContractError::MigrationNotAllowed)
            }
        }
    }
}

#[proc_macro_derive(Executable)]
pub fn derive_executable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    executable(name).into()
}

fn executable(name: &Ident) -> TokenStream2 {
    quote! {
        use interchain_token_service::executable::InterchainTokenExecutableInterface as _;

        impl interchain_token_service::executable::DeriveOnly for #name {}

        #[contractimpl]
        impl interchain_token_service::executable::InterchainTokenExecutableInterface for #name {
            fn execute_with_interchain_token(
                env: &Env,
                source_chain: String,
                message_id: String,
                source_address: Bytes,
                payload: Bytes,
                token_id: BytesN<32>,
                token_address: Address,
                amount: i128,
            ) -> Result<(), soroban_sdk::Error> {
                    <Self as interchain_token_service::executable::CustomExecutable>::interchain_token_service(env).require_auth();
                    <Self as interchain_token_service::executable::CustomExecutable>::execute(
                        env,
                        source_chain,
                        message_id,
                        source_address,
                        payload,
                        token_id,
                        token_address,
                        amount,
                    ).map_err(|error| error.into())
            }
        }
    }
}
