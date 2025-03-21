#![no_std]

// required by goldie
#[cfg(any(test, feature = "testutils"))]
extern crate std;

#[cfg(any(test, feature = "testutils"))]
pub mod testutils;

#[cfg(test)]
mod tests;

pub mod traits;

pub mod string;

pub mod types;

pub mod error;

pub mod ttl;

pub mod events;

#[cfg(any(test, feature = "derive"))]
pub mod interfaces;

pub mod address;

// This is needed to make the soroban_sdk macros work
pub use soroban_sdk;
pub use soroban_sdk::*;
// override specific soroban_sdk macro
pub use stellar_axelar_std_derive::contractimpl;
#[cfg(any(test, feature = "derive"))]
pub use stellar_axelar_std_derive::*;
