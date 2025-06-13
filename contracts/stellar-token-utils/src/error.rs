use stellar_axelar_std::{contracterror, soroban_sdk};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    MigrationNotAllowed = 1,
    MigrationInProgress = 2,
    InvalidAssetXdr = 3,
}
