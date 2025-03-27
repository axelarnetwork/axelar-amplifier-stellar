use stellar_axelar_std::{Address, IntoEvent};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterAddedEvent {
    pub minter: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterRemovedEvent {
    pub minter: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    #[singleton_data]
    pub amount: i128,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MintEvent {
    pub owner: Address,
    pub to: Address,
    #[singleton_data]
    pub amount: i128,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ApproveEvent {
    pub owner: Address,
    pub spender: Address,
    #[singleton_data]
    pub amount: i128,
    #[singleton_data]
    pub expiration_ledger: u32,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct BurnEvent {
    pub from: Address,
    #[singleton_data]
    pub amount: i128,
}
