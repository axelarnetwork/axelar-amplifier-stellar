use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::Env;

use crate::error::ContractError;
use crate::InterchainToken;

impl CustomMigratableInterface for InterchainToken {
    type MigrationData = ();
    type Error = ContractError;

    fn __migrate(_env: &Env, _migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        Ok(())
    }
}
