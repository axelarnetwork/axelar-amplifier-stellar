use soroban_sdk::Address;
use stellar_axelar_std::IntoEvent;

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterAddedEvent {
    pub minter: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterRemovedEvent {
    pub minter: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MintedEvent {
    pub to: Address,
    pub amount: i128,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ApprovedEvent {
    pub from: Address,
    pub spender: Address,
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct TransferredEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct BurnedEvent {
    pub from: Address,
    pub amount: i128,
}
