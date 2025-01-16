use core::fmt::Debug;

#[cfg(any(test, feature = "testutils"))]
use stellar_axelar_std::events::Event;
use stellar_axelar_std::IntoEvent;
use soroban_sdk::{Address, Bytes, BytesN, String};

use crate::types::Message;

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ContractCalledEvent {
    pub caller: Address,
    pub destination_chain: String,
    pub destination_address: String,
    pub payload_hash: BytesN<32>,
    #[data]
    pub payload: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MessageApprovedEvent {
    pub message: Message,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MessageExecutedEvent {
    pub message: Message,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct SignersRotatedEvent {
    pub epoch: u64,
    pub signers_hash: BytesN<32>,
}
