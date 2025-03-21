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

// This is needed to make the soroban_sdk macros work, 
// because they generate code containing soroban_sdk::{...} 
pub use soroban_sdk;
// re-export the soroban_sdk so all types are available at the top level,
// and can be overwritten if necessary
pub use soroban_sdk::*;
// override specific soroban_sdk macro
#[cfg(any(test, feature = "derive"))]
pub use stellar_axelar_std_derive::contractimpl;
#[cfg(any(test, feature = "derive"))]
pub use stellar_axelar_std_derive::*;
