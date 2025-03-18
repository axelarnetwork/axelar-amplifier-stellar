use soroban_sdk::Env;
use stellar_axelar_std::interfaces::{CustomMigratableInterface, UpgradableClient};

use crate::error::ContractError;
use crate::storage::TokenIdConfigValue;
use crate::types::CustomMigrationData;
use crate::{storage, InterchainTokenService};

pub mod legacy_storage {
    use soroban_sdk::{contracttype, BytesN};
    use stellar_axelar_std::contractstorage;

    #[contractstorage]
    enum LegacyDataKey {
        #[temporary]
        #[value(i128)]
        FlowOut { flow_key: FlowKey },

        #[temporary]
        #[value(i128)]
        FlowIn { flow_key: FlowKey },
    }

    #[contracttype]
    #[derive(Clone, Debug)]
    pub struct FlowKey {
        pub token_id: BytesN<32>,
        pub epoch: u64,
    }
}

impl CustomMigratableInterface for InterchainTokenService {
    type MigrationData = CustomMigrationData;
    type Error = ContractError;

    fn __migrate(env: &Env, migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        let CustomMigrationData {
            new_token_manager_wasm_hash,
            new_interchain_token_wasm_hash,
            token_ids,
            current_epoch,
        } = migration_data;

        for token_id in token_ids.into_iter() {
            let TokenIdConfigValue {
                token_address: interchain_token,
                token_manager,
                ..
            } = storage::try_token_id_config(env, token_id.clone())
                .ok_or(ContractError::InvalidTokenId)?;

            UpgradableClient::new(env, &token_manager).upgrade(&new_token_manager_wasm_hash);
            UpgradableClient::new(env, &interchain_token).upgrade(&new_interchain_token_wasm_hash);

            let flow_key = legacy_storage::FlowKey {
                token_id: token_id.clone(),
                epoch: current_epoch,
            };

            let flow_out = legacy_storage::try_flow_out(env, flow_key.clone())
                .ok_or(ContractError::InvalidFlowKey)?;
            let flow_in = legacy_storage::try_flow_in(env, flow_key.clone())
                .ok_or(ContractError::InvalidFlowKey)?;

            storage::set_flow_in(env, token_id.clone(), current_epoch, &flow_in);
            storage::set_flow_out(env, token_id.clone(), current_epoch, &flow_out);
        }

        storage::set_token_manager_wasm_hash(env, &new_token_manager_wasm_hash);
        storage::set_interchain_token_wasm_hash(env, &new_interchain_token_wasm_hash);

        Ok(())
    }
}
