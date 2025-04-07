use core::fmt::Debug;

use stellar_axelar_std::{Address, Bytes, IntoEvent};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ProposalScheduledEvent {
    pub target: Address,
    pub eta: u64,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ProposalCancelledEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ProposalExecutedEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorProposalApprovedEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorProposalCancelledEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorProposalExecutedEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}
