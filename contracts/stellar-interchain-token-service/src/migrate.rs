use stellar_axelar_std::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
use stellar_axelar_std::interfaces::CustomMigratableInterface;
use stellar_axelar_std::{contracttype, soroban_sdk, vec, Address, BytesN, Env, String, Symbol};
use stellar_upgrader::UpgraderClient;

use crate::error::ContractError;
use crate::flow_limit::current_epoch;
use crate::storage::TokenIdConfigValue;
use crate::types::TokenManagerType;
use crate::{storage, InterchainTokenService};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomMigrationData {
    pub new_token_manager_wasm_hash: BytesN<32>,
    pub new_interchain_token_wasm_hash: BytesN<32>,
}

pub mod legacy_storage {
    use stellar_axelar_std::{contractstorage, contracttype, soroban_sdk, BytesN};

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
        } = migration_data;

        storage::set_token_manager_wasm_hash(env, &new_token_manager_wasm_hash);
        storage::set_interchain_token_wasm_hash(env, &new_interchain_token_wasm_hash);

        Ok(())
    }
}

pub fn migrate_token(
    env: &Env,
    token_id: BytesN<32>,
    upgrader: Address,
    new_version: String,
) -> Result<(), ContractError> {
    let upgrader_client = UpgraderClient::new(env, &upgrader);

    let TokenIdConfigValue {
        token_address: interchain_token,
        token_manager,
        token_manager_type,
    } = storage::try_token_id_config(env, token_id.clone()).ok_or(ContractError::InvalidTokenId)?;

    env.authorize_as_current_contract(vec![
        env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: upgrader.clone(),
                fn_name: Symbol::new(env, "upgrade"),
                args: vec![env, storage::token_manager_wasm_hash(env).into()],
            },
            sub_invocations: vec![env],
        }),
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_manager.clone(),
                fn_name: Symbol::new(env, "upgrade"),
                args: vec![env, storage::token_manager_wasm_hash(env).into()],
            },
            sub_invocations: vec![env],
        }),
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_manager.clone(),
                fn_name: Symbol::new(env, "migrate"),
                args: vec![env, ().into()],
            },
            sub_invocations: vec![env],
        }),
    ]);

    upgrader_client.upgrade(
        &token_manager,
        &new_version,
        &storage::token_manager_wasm_hash(env),
        &vec![env, ().into()],
    );

    /* Only tokens deployed via ITS may be upgraded. */
    if token_manager_type == TokenManagerType::NativeInterchainToken {
        env.authorize_as_current_contract(vec![
            env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: upgrader.clone(),
                    fn_name: Symbol::new(env, "upgrade"),
                    args: vec![env, storage::interchain_token_wasm_hash(env).into()],
                },
                sub_invocations: vec![env],
            }),
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: interchain_token.clone(),
                    fn_name: Symbol::new(env, "upgrade"),
                    args: vec![env, storage::interchain_token_wasm_hash(env).into()],
                },
                sub_invocations: vec![env],
            }),
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: interchain_token.clone(),
                    fn_name: Symbol::new(env, "migrate"),
                    args: vec![env, ().into()],
                },
                sub_invocations: vec![env],
            }),
        ]);

        upgrader_client.upgrade(
            &interchain_token,
            &new_version,
            &storage::interchain_token_wasm_hash(env),
            &vec![env, ().into()],
        );
    }

    let current_epoch = current_epoch(env);

    let flow_key = legacy_storage::FlowKey {
        token_id: token_id.clone(),
        epoch: current_epoch,
    };

    if let Some(flow_out) = legacy_storage::try_flow_out(env, flow_key.clone()) {
        storage::set_flow_out(env, token_id.clone(), current_epoch, &flow_out);
    }
    if let Some(flow_in) = legacy_storage::try_flow_in(env, flow_key) {
        storage::set_flow_in(env, token_id, current_epoch, &flow_in);
    }

    Ok(())
}
