use stellar_axelar_std::{Address, Bytes, IntoEvent};

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ProposalScheduledEvent {
    pub proposal_hash: Bytes,
    pub target: Address,
    pub call_data: Bytes,
    pub eta: u64,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ProposalCancelledEvent {
    pub proposal_hash: Bytes,
    pub target: Address,
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorProposalApprovedEvent {
    pub proposal_hash: Bytes,
    pub target: Address,
    pub call_data: Bytes,
}

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorProposalCancelledEvent {
    pub proposal_hash: Bytes,
    pub target: Address,
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
pub struct OperatorProposalExecutedEvent {
    pub target: Address,
    pub proposal_hash: Bytes,
    #[data]
    pub call_data: Bytes,
}
