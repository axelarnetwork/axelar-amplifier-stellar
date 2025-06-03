use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ContractError {
    MigrationNotAllowed = 1,
    MigrationInProgress = 2,
    InvalidAssetCode = 3,
}
