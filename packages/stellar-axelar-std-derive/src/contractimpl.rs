use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_quote, AngleBracketedGenericArguments, GenericArgument, ImplItem, ImplItemFn, ItemImpl,
    PathArguments, PathSegment, ReturnType, Stmt, Type, TypePath, Visibility,
};

use crate::utils::{parse_env_identifier, PrependStatement};

pub fn contractimpl(mut impl_block: ItemImpl) -> Result<proc_macro2::TokenStream, syn::Error> {
    // if the block implements a trait, all of its functions are implicitly public, otherwise we only need to match public functions
    let match_contract_endpoint = if impl_block.trait_.is_some() {
        match_any_fn
    } else {
        match_pub_fn
    };

    impl_block
        .items
        .iter_mut()
        .filter_map(match_only_fn)
        .filter_map(match_contract_endpoint)
        // if a function doesn't have any arguments it cannot modify the environment, so it's safe to be called during migration
        .filter(has_args)
        .filter(should_be_blocked_during_migration)
        .try_for_each::<_, Result<_, syn::Error>>(|method| {
            let env_ident = parse_env_identifier(&method.sig.inputs)?;
            let error_handling = if can_return_contract_error(&method.sig.output) {
                return_migration_in_progress()
            } else {
                panic_on_failure()
            };

            method.prepend_statement(migration_in_progess_check(env_ident, error_handling));
            Ok(())
        })?;

    Ok(quote! {
        #[soroban_sdk::contractimpl]
        #impl_block
    })
}

fn match_only_fn(item: &mut ImplItem) -> Option<&mut ImplItemFn> {
    match item {
        ImplItem::Fn(fn_) => Some(fn_),
        _ => None,
    }
}

fn match_any_fn(fn_: &mut ImplItemFn) -> Option<&mut ImplItemFn> {
    Some(fn_)
}

fn match_pub_fn(fn_: &mut ImplItemFn) -> Option<&mut ImplItemFn> {
    match fn_ {
        ImplItemFn {
            vis: Visibility::Public(_),
            ..
        } => Some(fn_),
        _ => None,
    }
}

fn has_args(fn_: &&mut ImplItemFn) -> bool {
    !fn_.sig.inputs.is_empty()
}

fn should_be_blocked_during_migration(fn_: &&mut ImplItemFn) -> bool {
    fn_.attrs
        .iter()
        .all(|attr| !attr.path().is_ident("allow_during_migration"))
}

fn can_return_contract_error(return_type: &ReturnType) -> bool {
    match_result(return_type)
        .and_then(match_error_arg)
        .and_then(match_contract_error_type)
        .is_some()
}

fn match_result(return_type: &ReturnType) -> Option<&PathSegment> {
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

fn match_error_arg(result_segment: &PathSegment) -> Option<&GenericArgument> {
    match &result_segment.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            (args.len() == 2).then(|| args.last()).flatten()
        }
        _ => None,
    }
}

fn match_contract_error_type(error: &GenericArgument) -> Option<&PathSegment> {
    match error {
        GenericArgument::Type(Type::Path(TypePath { path, .. })) => path
            .segments
            .last()
            .filter(|segment| segment.ident == "ContractError"),
        _ => None,
    }
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

fn migration_in_progess_check(env_ident: &Ident, error_handling: Stmt) -> Stmt {
    parse_quote! {
        if stellar_axelar_std::interfaces::is_migrating(&#env_ident){
            #error_handling
        }
    }
}
