use stellar_axelar_std::{Address, IntoEvent};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterAddedEvent {
    pub minter: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct MinterRemovedEvent {
    pub minter: Address,
}
