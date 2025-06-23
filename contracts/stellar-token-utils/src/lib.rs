#![no_std]

#[cfg(test)]
extern crate std;

pub mod interface;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(test)))] {
        pub use interface::TokenUtilsInterface;
    } else {
        mod contract;

        pub use contract::{TokenUtils, TokenUtilsClient};
    }
}
