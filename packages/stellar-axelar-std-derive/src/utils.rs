use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, Pat, PatType, Type, TypePath};

pub fn parse_env_ident(args: &Punctuated<FnArg, Comma>) -> Option<&Ident> {
    args.iter().find_map(|arg| match arg {
        FnArg::Typed(PatType { ty, pat, .. }) if is_env(ty) => get_ident(pat),
        _ => None,
    })
}

const fn get_ident(pat: &Pat) -> Option<&Ident> {
    match pat {
        Pat::Ident(pat) => Some(&pat.ident),
        _ => None,
    }
}

fn is_env(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Env"),
        Type::Reference(reference) => is_env(&reference.elem),
        _ => false,
    }
}
