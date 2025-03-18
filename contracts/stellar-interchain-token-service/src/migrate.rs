use soroban_sdk::{Address, BytesN, Env, Vec};
use stellar_axelar_std::interfaces::{CustomMigratableInterface, UpgradableClient};

use crate::error::ContractError;
use crate::{deployer, storage, InterchainTokenService};

pub mod legacy_storage {
    use soroban_sdk::{contracttype, Address, BytesN, String};
    use stellar_axelar_std::contractstorage;

    use crate::storage::TokenIdConfigValue;

    #[contractstorage]
    enum DataKey {
        #[instance]
        #[value(Address)]
        Gateway,

        #[instance]
        #[value(Address)]
        GasService,

        #[instance]
        #[value(String)]
        ChainName,

        #[instance]
        #[value(String)]
        ItsHubAddress,

        #[instance]
        #[value(Address)]
        NativeTokenAddress,

        #[instance]
        #[value(BytesN<32>)]
        InterchainTokenWasmHash,

        #[instance]
        #[value(BytesN<32>)]
        TokenManagerWasmHash,

        #[persistent]
        #[status]
        TrustedChain { chain: String },

        #[persistent]
        #[value(TokenIdConfigValue)]
        TokenIdConfig { token_id: BytesN<32> },

        #[persistent]
        #[value(i128)]
        FlowLimit { token_id: BytesN<32> },

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
    type MigrationData = (
        BytesN<32>, /* new_wasm_hash */
        Vec<(BytesN<32> /* token_id */, u64 /* epoch */)>,
    );
    type Error = ContractError;

    fn __migrate(env: &Env, migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        let (new_wasm_hash, token_ids_and_epochs) = migration_data;

        for (token_id, epoch) in token_ids_and_epochs {
            let token_id_config = storage::try_token_id_config(env, token_id.clone())
                .ok_or(ContractError::InvalidTokenId)?;

            Self::migrate_token_manager(env, token_id_config.token_manager, &new_wasm_hash)?;
            Self::migrate_interchain_token(env, &token_id, &new_wasm_hash)?;
            Self::migrate_flow_key(env, &token_id, epoch)?;
        }

        Ok(())
    }
}

impl InterchainTokenService {
    fn migrate_token_manager(
        env: &Env,
        token_manager_address: Address,
        new_wasm_hash: &BytesN<32>,
    ) -> Result<(), ContractError> {
        UpgradableClient::new(&env, &token_manager_address).upgrade(new_wasm_hash);

        storage::set_token_manager_wasm_hash(env, new_wasm_hash);

        Ok(())
    }

    fn migrate_interchain_token(
        env: &Env,
        token_id: &BytesN<32>,
        new_wasm_hash: &BytesN<32>,
    ) -> Result<(), ContractError> {
        let interchain_token_address = deployer::interchain_token_address(env, token_id.clone());

        UpgradableClient::new(&env, &interchain_token_address).upgrade(new_wasm_hash);

        storage::set_interchain_token_wasm_hash(env, new_wasm_hash);

        Ok(())
    }

    fn migrate_flow_key(env: &Env, token_id: &BytesN<32>, epoch: u64) -> Result<(), ContractError> {
        let flow_key = legacy_storage::FlowKey {
            token_id: token_id.clone(),
            epoch,
        };

        let flow_out = legacy_storage::try_flow_out(env, flow_key.clone())
            .ok_or(ContractError::InvalidFlowKey)?;
        let flow_in = legacy_storage::try_flow_in(env, flow_key.clone())
            .ok_or(ContractError::InvalidFlowKey)?;

        storage::set_flow_in(env, token_id.clone(), epoch, &flow_in);
        storage::set_flow_out(env, token_id.clone(), epoch, &flow_out);

        Ok(())
    }
}
