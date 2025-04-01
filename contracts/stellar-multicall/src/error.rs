use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    FunctionCallFailed = 1,
    MigrationInProgress = 2,
}
