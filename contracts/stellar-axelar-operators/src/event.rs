use stellar_axelar_std::{Address, IntoEvent};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorAddedEvent {
    pub operator: Address,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorRemovedEvent {
    pub operator: Address,
}
