use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_quote, AngleBracketedGenericArguments, GenericArgument, ImplItem, ImplItemFn, ItemImpl,
    PathArguments, PathSegment, ReturnType, Stmt, Type, TypePath, Visibility,
};

use crate::utils::{parse_env_identifier, PrependStatement};

pub fn contractimpl(impl_block: &mut ItemImpl) -> Result<proc_macro2::TokenStream, syn::Error> {
    for method in impl_block.items.iter_mut() {
        if let ImplItem::Fn(method_fn) = method {
            let is_contract_endpoint =
                impl_block.trait_.is_some() || matches!(method_fn.vis, Visibility::Public(_));
            let is_stateful = has_args(&method_fn);

            if is_stateful && is_contract_endpoint {
                if is_allowed_during_migration(&method_fn) {
                    remove_allow_during_migration_attribute(method_fn);
                } else {
                    block_during_migration(method_fn)?;
                }
            }

            if is_contract_endpoint && has_env_param(&method_fn) {
                instance_ttl_extension(method_fn)?;
            }
        }
    }

    Ok(quote! {
        #[soroban_sdk::contractimpl]
        #impl_block
    })
}

fn instance_ttl_extension(method: &mut ImplItemFn) -> Result<(), syn::Error> {
    let env_ident = parse_env_identifier(&method.sig.inputs)?;

    let extend_ttl_stmt: Stmt = parse_quote! {
        stellar_axelar_std::ttl::extend_instance_ttl(&#env_ident);
    };

    method.prepend_statement(extend_ttl_stmt);
    Ok(())
}

fn has_env_param(fn_: &&mut ImplItemFn) -> bool {
    parse_env_identifier(&fn_.sig.inputs).is_ok()
}

fn block_during_migration(method: &mut ImplItemFn) -> Result<(), syn::Error> {
    let env_ident = parse_env_identifier(&method.sig.inputs)?;
    let error_handling = if can_return_contract_error(&method.sig.output) {
        return_migration_in_progress()
    } else {
        panic_on_failure()
    };

    method.prepend_statement(expect_migration_complete(env_ident, error_handling));
    Ok(())
}

fn remove_allow_during_migration_attribute(method: &mut ImplItemFn) {
    method
        .attrs
        .retain(|attr| !attr.path().is_ident("allow_during_migration"))
}

fn has_args(fn_: &&mut ImplItemFn) -> bool {
    !fn_.sig.inputs.is_empty()
}

fn is_allowed_during_migration(fn_: &&mut ImplItemFn) -> bool {
    fn_.attrs
        .iter()
        .any(|attr| attr.path().is_ident("allow_during_migration"))
}

fn can_return_contract_error(return_type: &ReturnType) -> bool {
    any_result(return_type)
        .and_then(extract_error_arg)
        .filter(is_contract_error_type)
        .is_some()
}

fn any_result(return_type: &ReturnType) -> Option<&PathSegment> {
    match return_type {
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(TypePath { path, .. }) => path
                .segments
                .last()
                .filter(|segment| segment.ident == "Result"),
            _ => None,
        },
        _ => None,
    }
}

fn extract_error_arg(result_segment: &PathSegment) -> Option<&GenericArgument> {
    match &result_segment.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            (args.len() == 2).then(|| args.last()).flatten()
        }
        _ => None,
    }
}

fn is_contract_error_type(error: &&GenericArgument) -> bool {
    matches!(error, GenericArgument::Type(Type::Path(TypePath { path, .. }))
        if path.segments
        .last()
        .filter(|segment| segment.ident == "ContractError")
        .is_some())
}

fn return_migration_in_progress() -> Stmt {
    parse_quote! {
        return Err(ContractError::MigrationInProgress);
    }
}

fn panic_on_failure() -> Stmt {
    parse_quote! {
        panic!("contract migration in progress");
    }
}

fn expect_migration_complete(env: &Ident, error_handling: Stmt) -> Stmt {
    parse_quote! {
        if stellar_axelar_std::interfaces::is_migrating(&#env){
            #error_handling
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn entrypoints_have_appropriate_checks_added() {
        let mut contract_input: syn::ItemImpl = syn::parse_quote! {
            #[contractimpl]
            impl Contract {
                pub fn should_return_contract_error(env: &Env, arg: String) -> Result<u32, ContractError> {
                    // entrypoint code

                    Ok(3)
                }

                pub fn should_panic(env: &Env, arg: String) {
                    // entrypoint code
                }

                pub fn should_panic_because_not_contract_error(env: &Env, arg: String) ->Result<u32, OtherError> {
                    // entrypoint code

                    Ok(5)
                }

                pub fn should_have_no_check_because_not_stateful(){
                    // entrypoint code
                }

                #[allow_during_migration]
                pub fn is_allowed_during_migration(env: &Env, arg: String) {
                    // entrypoint code
                }

                fn should_have_no_check_because_private(env: &Env, arg: String) {
                    // some logic
                }
            }
        };

        let contract_impl: proc_macro2::TokenStream =
            crate::contractimpl::contractimpl(&mut contract_input).unwrap();
        let contract_impl_file: syn::File = syn::parse2(contract_impl).unwrap();
        let formatted_contract_impl = prettyplease::unparse(&contract_impl_file);

        goldie::assert!(formatted_contract_impl);
    }

    #[test]
    fn trait_entrypoints_have_appropriate_checks_added() {
        let mut contract_input: syn::ItemImpl = syn::parse_quote! {
            #[contractimpl]
            impl SomeTrait for Contract {
                fn should_return_contract_error(env: &Env, arg: String) -> Result<u32, ContractError> {
                    // entrypoint code

                    Ok(3)
                }

                fn should_panic(env: &Env, arg: String) {
                    // entrypoint code
                }

                fn should_have_no_check_because_not_stateful(){
                    // entrypoint code
                }

                #[allow_during_migration]
                fn is_allowed_during_migration(env: &Env, arg: String) {
                    // entrypoint code
                }
            }
        };

        let contract_impl: proc_macro2::TokenStream =
            crate::contractimpl::contractimpl(&mut contract_input).unwrap();
        let contract_impl_file: syn::File = syn::parse2(contract_impl).unwrap();
        let formatted_contract_impl = prettyplease::unparse(&contract_impl_file);

        goldie::assert!(formatted_contract_impl);
    }
}
