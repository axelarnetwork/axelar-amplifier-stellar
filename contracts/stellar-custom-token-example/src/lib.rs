#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod contract;

#[cfg(test)]
mod tests;

pub use contract::CustomToken;
