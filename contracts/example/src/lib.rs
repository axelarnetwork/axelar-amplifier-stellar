#![no_std]
extern crate std;

mod contract;
pub mod event;
mod storage_types;

pub use contract::{Example, ExampleClient};
