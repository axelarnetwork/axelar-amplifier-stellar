// #![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod error;
mod interface;
mod types;

#[cfg(test)]
mod tests;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "library", not(feature = "testutils")))] {
        pub use interface::StellarGovernanceClient;
        pub use interface::StellarGovernanceInterface;
    } else {
        mod contract;
        mod event;
        mod storage;
        mod timelock;

        pub use crate::contract::{StellarGovernance, StellarGovernanceClient};
        pub use crate::interface::StellarGovernanceInterface;
    }
}
