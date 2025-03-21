use soroban_sdk::{contracttype, Address, BytesN, Env, String, Vec};
use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_upgrader::interface::UpgraderClient;

use crate::error::ContractError;
use crate::flow_limit::current_epoch;
use crate::storage::TokenIdConfigValue;
use crate::types::TokenManagerType;
use crate::{storage, InterchainTokenService};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomMigrationData {
    pub upgrader_client: Address,
    pub new_version: String,
    pub token_ids: Vec<BytesN<32>>,
    pub new_token_manager_wasm_hash: BytesN<32>,
    pub new_interchain_token_wasm_hash: BytesN<32>,
}

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
            upgrader_client,
            new_version,
            token_ids,
            new_token_manager_wasm_hash,
            new_interchain_token_wasm_hash,
        } = migration_data;

        let current_epoch = current_epoch(env);
        let upgrader_client = UpgraderClient::new(env, &upgrader_client);

        for token_id in token_ids.into_iter() {
            let TokenIdConfigValue {
                token_address: interchain_token,
                token_manager,
                token_manager_type,
            } = storage::try_token_id_config(env, token_id.clone())
                .ok_or(ContractError::InvalidTokenId)?;

            upgrader_client.upgrade(
                // FIXME: Err(Abort) immediately on internal call to upgrade()
                &token_manager,
                &new_version,
                &new_token_manager_wasm_hash,
                &soroban_sdk::Vec::new(env),
            );

            if token_manager_type == TokenManagerType::LockUnlock {
                continue;
            }

            upgrader_client.upgrade(
                &interchain_token,
                &new_version,
                &new_interchain_token_wasm_hash,
                &soroban_sdk::Vec::new(env),
            );

            let flow_key = legacy_storage::FlowKey {
                token_id: token_id.clone(),
                epoch: current_epoch,
            };

            if let Some(flow_out) = legacy_storage::try_flow_out(env, flow_key.clone()) {
                storage::set_flow_out(env, token_id.clone(), current_epoch, &flow_out);
            }
            if let Some(flow_in) = legacy_storage::try_flow_in(env, flow_key.clone()) {
                storage::set_flow_in(env, token_id.clone(), current_epoch, &flow_in);
            }
        }

        storage::set_token_manager_wasm_hash(env, &new_token_manager_wasm_hash);
        storage::set_interchain_token_wasm_hash(env, &new_interchain_token_wasm_hash);

        Ok(())
    }
}
