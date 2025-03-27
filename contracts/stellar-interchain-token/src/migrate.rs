use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{ensure, Env};

use crate::error::ContractError;
use crate::{storage, InterchainToken};

pub mod legacy_storage {}

impl CustomMigratableInterface for InterchainToken {
    type MigrationData = i128;
    type Error = ContractError;

    fn __migrate(env: &Env, migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        let total_supply = migration_data;

        ensure!(total_supply >= 0, ContractError::InvalidAmount);

        storage::set_total_supply(env, &total_supply);

        Ok(())
    }
}
