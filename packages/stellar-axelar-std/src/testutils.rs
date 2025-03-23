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
macro_rules! assert_storage_layout_is_sound {
    ($actual:expr) => {{
        const fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }

        // because f() will be defined inside the parent function, we can strip away the suffic to get the parent function name
        let mut function_path = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = function_path.strip_suffix("::{{closure}}") {
            function_path = rest;
        }

        let source_file = $crate::testutils::__source_file(file!());
        $crate::testutils::__assert_storage_layout_is_sound($actual, source_file, function_path);
    }};
}

#[doc(hidden)]
pub fn __assert_storage_layout_is_sound(
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
