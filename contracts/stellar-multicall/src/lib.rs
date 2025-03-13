#![no_std]

#[cfg(test)]
extern crate std;

pub mod error;

pub mod interface;

pub mod types;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(test)))] {
        pub use interface::{MulticallClientClient, MulticallInterface};
    } else {
        mod contract;

        pub use contract::{Multicall, MulticallClient};
    }
}
