#![cfg(any(test, feature = "testutils"))]
extern crate std;

use std::env;
use std::path::{Path, PathBuf};

pub use soroban_sdk::testutils::*;

/// Helper macro for building and verifying authorization chains in Soroban contract tests.
///
/// Used to verify that contract calls require the correct sequence of authorizations.
/// See the example package for usage in gas payment and cross-chain message verification scenarios.
///
/// # Example
/// ```rust,ignore
/// // Create authorization
/// let transfer_auth = auth_invocation!(
///     user,
///     asset_client.transfer(
///         &user,
///         source_gas_service_id,
///         gas_token.amount
///     )
/// );
///
/// // Create nested authorization chain for gas payment
/// let pay_gas_auth = auth_invocation!(
///     user,
///     source_gas_service_client.pay_gas(
///         source_app.address,
///         destination_chain,
///         destination_address,
///         payload,
///         &user,
///         gas_token,
///         &Bytes::new(&env)
///     ),
///     transfer_auth
/// );
///
/// // Verify authorizations
/// assert_eq!(env.auths(), pay_gas_auth);
/// ```
#[macro_export]
macro_rules! auth_invocation {
    // Basic case without sub-invocations
    ($caller:expr, $client:ident.$method:ident($($arg:expr),* $(,)?)) => {{
        std::vec![(
            $caller.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    $client.address.clone(),
                    Symbol::new(&$client.env, stringify!($method)),
                    ($($arg),*).into_val(&$client.env),
                )),
                sub_invocations: std::vec![],
            }
        )]
    }};

    // Case with sub-invocations (handles both regular and user auth cases)
    ($caller:expr, $client:ident.$method:ident($($arg:expr),* $(,)?), $subs:expr $(, $user:ident)?) => {{
        std::vec![(
            $caller.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    $client.address.clone(),
                    Symbol::new(&$client.env, stringify!($method)),
                    ($($arg),*).into_val(&$client.env),
                )),
                sub_invocations: $subs.into_iter().map(|(_, inv)| inv).collect(),
            }
        )]
    }};
}

#[macro_export]
macro_rules! assert_matches_golden_file {
    ($actual:expr) => {{
        const fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }

        // because f() will be defined inside the parent function, we can strip away the suffic to get the parent function name
        let mut function_path = type_name_of(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = function_path.strip_suffix("::{{closure}}") {
            function_path = rest;
        }

        let source_file = $crate::testutils::__source_file(file!());
        $crate::testutils::__assert_matches_golden_file($actual, source_file, function_path);
    }};
}

#[doc(hidden)]
pub fn __assert_matches_golden_file(
    actual: impl AsRef<str>,
    source_file: impl AsRef<Path>,
    function_path: impl AsRef<str>,
) {
    if let Err(err) = goldie::Goldie::new(source_file, function_path).assert(actual) {
        ::std::panic!("{}", err);
    }
}

#[doc(hidden)]
pub fn __source_file(file: &str) -> PathBuf {
    goldie::cargo_workspace_dir(env!("CARGO_MANIFEST_DIR")).join(file)
}

#[cfg(test)]
mod tests {
    use std::borrow::ToOwned;
    use std::path::PathBuf;
    use std::{env, fs, println};

    use crate::testutils::__source_file;

    #[test]
    #[should_panic]
    fn panics_when_golden_file_does_not_exist() {
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let function_path = type_name_of(panics_when_golden_file_does_not_exist);

        let golden_file = golden_file_name(function_path);

        assert_matches_golden_file!("something");

        if matches!(
            env::var("GOLDIE_UPDATE").ok().as_deref(),
            Some("1" | "true")
        ) {
            let _ = fs::remove_file(golden_file);
            panic!("goldie set to regenerate golden file");
        }
    }

    fn golden_file_name(function_path: &str) -> PathBuf {
        let (_, fn_name) = function_path.rsplit_once("::").unwrap();
        let source_file = __source_file(file!());

        let golden_file = {
            let mut p = source_file.parent().unwrap().to_owned();
            p.push("testdata");
            p.push(fn_name);
            p.set_extension("golden");
            p
        };
        golden_file
    }
}
