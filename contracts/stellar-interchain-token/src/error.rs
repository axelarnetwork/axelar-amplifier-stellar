use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    MigrationNotAllowed = 1,
    NotMinter = 2,
    InvalidAmount = 3,
    InvalidExpirationLedger = 4,
    InsufficientAllowance = 5,
    InsufficientBalance = 6,
    MigrationInProgress = 7,
}
