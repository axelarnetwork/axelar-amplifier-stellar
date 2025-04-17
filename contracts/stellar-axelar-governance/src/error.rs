use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    MigrationNotAllowed = 1,
    MigrationInProgress = 2,
    InvalidCommandType = 3,
    OperatorProposalNotApproved = 4,
    NotGovernance = 5,
    ContractPaused = 6,
    InvalidAddress = 7,
    InvalidTimeLockHash = 8,
    TimeLockAlreadyScheduled = 9,
    TimeLockNotReady = 10,
    TimeLockNotScheduled = 11,
    InsufficientBalance = 12,
    InvalidParameter = 13,
    InvalidParameterType = 14,
}
