#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod contract;

pub use contract::CustomToken;
