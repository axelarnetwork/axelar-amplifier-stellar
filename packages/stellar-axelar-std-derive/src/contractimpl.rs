use crate::utils::parse_env_ident;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_quote, ImplItem, ImplItemFn, ItemImpl, Path, Stmt, Visibility};

pub fn contractimpl(mut impl_block: ItemImpl) -> proc_macro2::TokenStream {
    let match_contract_endpoint = match &impl_block.trait_ {
        Some((_, trait_path, _))
            if is_migratable_trait(trait_path) || is_upgradable_trait(trait_path) =>
        {
            do_not_match
        }
        Some(_) => match_non_trivial_trait_fn,
        None => match_non_trivial_pub_fn,
    };

    impl_block
        .items
        .iter_mut()
        .filter_map(match_contract_endpoint)
        .for_each(|method| {
            let env_ident = parse_env_ident(&method.sig.inputs)
                .expect("non-empty contract endpoints must have an Env argument");
            method
                .block
                .stmts
                .insert(0, migration_in_progess_check(env_ident));
        });

    quote! {
        #[soroban_sdk::contractimpl]
        #impl_block
    }
}

fn is_upgradable_trait(trait_path: &Path) -> bool {
    quote! {#trait_path}
        .to_string()
        .contains("UpgradableInterface")
}

fn is_migratable_trait(trait_path: &Path) -> bool {
    quote! {#trait_path}
        .to_string()
        .contains("MigratableInterface")
}

fn do_not_match(_item: &mut ImplItem) -> Option<&mut ImplItemFn> {
    None
}

fn match_non_trivial_trait_fn(item: &mut ImplItem) -> Option<&mut ImplItemFn> {
    match item {
        ImplItem::Fn(method) if !method.block.stmts.is_empty() => Some(method),
        _ => None,
    }
}

fn match_non_trivial_pub_fn(item: &mut ImplItem) -> Option<&mut ImplItemFn> {
    match_non_trivial_trait_fn(item).filter(|method| matches!(method.vis, Visibility::Public(_)))
}

fn migration_in_progess_check(env_ident: &Ident) -> Stmt {
    parse_quote! {
        if stellar_axelar_std::interfaces::is_migrating(&#env_ident){
            panic!()
        }
    }
}
