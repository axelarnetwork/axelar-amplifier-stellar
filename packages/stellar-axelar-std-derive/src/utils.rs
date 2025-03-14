use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ImplItemFn, ItemFn, Pat, PatType, Stmt, Type, TypePath};

pub trait PrependStatement {
    fn prepend_statement(&mut self, stmt: Stmt) -> &Self;
}

impl PrependStatement for ItemFn {
    fn prepend_statement(&mut self, stmt: Stmt) -> &Self {
        self.block.stmts.insert(0, stmt);
        self
    }
}

impl PrependStatement for ImplItemFn {
    fn prepend_statement(&mut self, stmt: Stmt) -> &Self {
        self.block.stmts.insert(0, stmt);
        self
    }
}

pub fn parse_env_identifier(args: &Punctuated<FnArg, Comma>) -> Result<&Ident, syn::Error> {
    args.iter()
        .find_map(match_arg_type_pattern)
        .filter(|type_pattern| is_env_type(&type_pattern.ty))
        .and_then(|pat_type| match_env_arg_identifier(&pat_type.pat))
        .ok_or_else(|| {
            syn::Error::new_spanned(
                args,
                "non-empty contract endpoints must have an Env argument",
            )
        })
}

const fn match_arg_type_pattern(arg: &FnArg) -> Option<&PatType> {
    match arg {
        FnArg::Typed(pat_type) => Some(pat_type),
        _ => None,
    }
}

fn is_env_type(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Env"),
        Type::Reference(reference) => is_env_type(&reference.elem),
        _ => false,
    }
}

const fn match_env_arg_identifier(pat: &Pat) -> Option<&Ident> {
    match pat {
        Pat::Ident(pat) => Some(&pat.ident),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_quote;

    use crate::utils::PrependStatement;

    #[test]
    fn prepend_statement_empty_body_generation_succeeds() {
        let mut input_fn: syn::ItemFn = syn::parse_quote! {
            fn test_fn(env: &Env, other: i32) {}
        };

        let generated_function = input_fn
            .prepend_statement(parse_quote! {
                Self::operator(&env).require_auth();
            })
            .to_token_stream();
        let generated_function_file: syn::File = syn::parse2(generated_function).unwrap();
        let formatted_generated_function = prettyplease::unparse(&generated_function_file);
        goldie::assert!(formatted_generated_function);
    }

    #[test]
    fn prepend_statement_generation_succeeds() {
        let mut input_fn: syn::ItemFn = syn::parse_quote! {
            fn test_fn(env: &Env, other: i32) {
            let x = 42;
            let y = vec![1, 2, 3];
            let z = x + y[2];
            }
        };

        let generated_function = input_fn
            .prepend_statement(parse_quote! {
                Self::operator(&env).require_auth();
            })
            .to_token_stream();

        let generated_function_file: syn::File = syn::parse2(generated_function).unwrap();
        let formatted_generated_function = prettyplease::unparse(&generated_function_file);

        goldie::assert!(formatted_generated_function);
    }
}
