use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotGovernance = 1,
    ProposalNotReady = 2,
    ProposalNotScheduled = 3,
    ProposalExecutionFailed = 4,
    InvalidProposalParams = 5,
    InvalidTimeDelay = 6,
    InvalidCommandType = 7,
    WithdrawalFailed = 8,
    OperatorProposalNotApproved = 9,
    MigrationInProgress = 10,
    MigrationNotAllowed = 11,
    ContractPaused = 12,
    InvalidAddress = 13,
    InvalidTimeLockHash = 14,
    TimeLockAlreadyScheduled = 15,
    TimeLockNotReady = 16,
    TimeLockNotScheduled = 17,
    InsufficientBalance = 18,
    ExecutionFailed = 19,
    InvalidContract = 20,
    InvalidParameter = 21,
    InvalidParameterType = 22,
}
