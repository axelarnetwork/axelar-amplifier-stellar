use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    SameVersion = 1,
    UnexpectedNewVersion = 2,
    MigrationInProgress = 3,
}
