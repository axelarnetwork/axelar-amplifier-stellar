#![no_std]

#[cfg(any(test, feature = "testutils"))]
#[macro_use]
extern crate std;

mod error;
mod r#macro;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(test)))] {
        mod interface;
        pub use interface::StellarTokenUtilsInterface;
    } else {
        mod interface;
        mod contract;

        pub use contract::{StellarTokenUtils, StellarTokenUtilsClient};
        pub use interface::StellarTokenUtilsInterface;
    }
}

pub use error::ContractError;
