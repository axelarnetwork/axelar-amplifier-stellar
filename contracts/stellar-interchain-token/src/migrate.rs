use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{contracttype, ensure, soroban_sdk, Env};

use crate::error::ContractError;
use crate::{storage, InterchainToken};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomMigrationData {
    pub total_supply: i128,
}

impl CustomMigratableInterface for InterchainToken {
    type MigrationData = CustomMigrationData;
    type Error = ContractError;

    fn __migrate(env: &Env, migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        let CustomMigrationData { total_supply } = migration_data;

        ensure!(total_supply >= 0, ContractError::InvalidAmount);

        storage::set_total_supply(env, &total_supply);

        Ok(())
    }
}
