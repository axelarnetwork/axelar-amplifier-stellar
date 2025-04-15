#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

pub mod error;
mod event;
mod interface;
mod macros;
mod types;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(feature = "testutils")))] {
        pub use interface::AxelarGovernanceClient;
        pub use interface::AxelarGovernanceInterface;
    } else {
        mod contract;
        mod storage;
        mod timelock;

        pub use crate::contract::{AxelarGovernance, AxelarGovernanceClient};
        pub use crate::interface::AxelarGovernanceInterface;
    }
}
