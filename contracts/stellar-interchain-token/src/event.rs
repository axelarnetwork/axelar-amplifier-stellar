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
#[event_name("transfer")]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    #[data]
    pub amount: i128,
}
