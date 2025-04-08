#![no_std]

#[cfg(test)]
extern crate std;

pub mod interface;
pub mod types;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(test)))] {
        pub use interface::{MulticallInterface};
    } else {
        mod contract;

        pub use contract::{Multicall, MulticallClient};
    }
}
