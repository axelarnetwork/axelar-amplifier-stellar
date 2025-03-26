use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_gas_service::testutils::setup_gas_service;
use stellar_axelar_gas_service::AxelarGasServiceClient;
use stellar_axelar_gateway::testutils::{setup_gateway, TestSignerSet};
use stellar_axelar_gateway::AxelarGatewayClient;
use stellar_axelar_std::{Address, BytesN, Env, IntoVal};
use stellar_upgrader::testutils::setup_upgrader;
use stellar_upgrader::UpgraderClient;

use crate::flow_limit::current_epoch;
use crate::migrate::{legacy_storage, CustomMigrationData};
use crate::storage::TokenIdConfigValue;
use crate::testutils::{setup_its, setup_its_token};
use crate::{storage, InterchainTokenServiceClient};

const NEW_INTERCHAIN_TOKEN_SERVICE_WASM: &[u8] =
    include_bytes!("../testdata/stellar_interchain_token_service.optimized.wasm");
const TOKEN_MANAGER_WASM_V110: &[u8] =
    include_bytes!("../testdata/stellar_token_manager_v1_1_0.optimized.wasm");
const INTERCHAIN_TOKEN_WASM_V110: &[u8] =
    include_bytes!("../testdata/stellar_interchain_token_v1_1_0.optimized.wasm");

pub const INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX: i32 = -4;
pub const INTERCHAIN_TOKEN_DEPLOYED_WITHOUT_GAS_TOKEN_EVENT_IDX: i32 = -2;
pub const INTERCHAIN_TOKEN_DEPLOYED_NO_SUPPLY_EVENT_IDX: i32 =
    INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX + 1;
pub const TOKEN_MANAGER_DEPLOYED_EVENT_IDX: i32 = INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX + 1;

pub trait TokenMetadataExt {
    fn new(env: &Env, name: &str, symbol: &str, decimal: u32) -> Self;
}

impl TokenMetadataExt for TokenMetadata {
    fn new(env: &Env, name: &str, symbol: &str, decimal: u32) -> Self {
        Self {
            decimal,
            name: name.into_val(env),
            symbol: symbol.into_val(env),
        }
    }
}

pub struct MigrateTestConfig<'a> {
    pub env: Env,
    pub owner: Address,
    pub its_client: InterchainTokenServiceClient<'a>,
    pub upgrader_client: UpgraderClient<'a>,
    pub token_id: BytesN<32>,
    pub current_epoch: u64,
    pub its_wasm_hash: BytesN<32>,
    pub token_manager: Address,
    pub interchain_token: Address,
    pub migration_data: CustomMigrationData,
}

pub fn setup_env<'a>() -> (
    Env,
    InterchainTokenServiceClient<'a>,
    AxelarGatewayClient<'a>,
    AxelarGasServiceClient<'a>,
    TestSignerSet,
) {
    let env = Env::default();

    let (signers, gateway_client) = setup_gateway(&env, 0, 5);
    let gas_service_client: AxelarGasServiceClient<'_> = setup_gas_service(&env);

    let client = setup_its(&env, &gateway_client, &gas_service_client, None);

    (env, client, gateway_client, gas_service_client, signers)
}

pub fn setup_migrate_env<'a>() -> MigrateTestConfig<'a> {
    let (env, its_client, ..) = setup_env();
    let upgrader_client = setup_upgrader(&env);
    let owner: Address = its_client.owner();
    let (token_id, _) = setup_its_token(&env, &its_client, &owner, 100);

    let its_wasm_hash = env
        .deployer()
        .upload_contract_wasm(NEW_INTERCHAIN_TOKEN_SERVICE_WASM);
    let new_token_manager_wasm_hash = env.deployer().upload_contract_wasm(TOKEN_MANAGER_WASM_V110);
    let new_interchain_token_wasm_hash = env
        .deployer()
        .upload_contract_wasm(INTERCHAIN_TOKEN_WASM_V110);

    let current_epoch = current_epoch(&env);

    let token_manager = its_client.token_manager_address(&token_id);
    let interchain_token = its_client.interchain_token_address(&token_id);

    MigrateTestConfig {
        env,
        owner,
        its_client,
        upgrader_client,
        token_id,
        current_epoch,
        its_wasm_hash,
        token_manager,
        interchain_token,
        migration_data: CustomMigrationData {
            new_token_manager_wasm_hash,
            new_interchain_token_wasm_hash,
        },
    }
}

pub fn setup_migrate_storage<'a>(
    env: &Env,
    token_config: TokenIdConfigValue,
    its_client: &InterchainTokenServiceClient<'a>,
    token_id: BytesN<32>,
    current_epoch: u64,
    flow_in_amount: i128,
    flow_out_amount: i128,
) {
    env.as_contract(&its_client.address, || {
        let flow_key = legacy_storage::FlowKey {
            token_id: token_id.clone(),
            epoch: current_epoch,
        };

        legacy_storage::set_flow_in(&env, flow_key.clone(), &flow_in_amount);
        legacy_storage::set_flow_out(&env, flow_key, &flow_out_amount);

        storage::set_token_id_config(&env, token_id.clone(), &token_config);
    });
}

pub fn assert_migrate_storage<'a>(
    env: &Env,
    its_client: &InterchainTokenServiceClient<'a>,
    migration_data: CustomMigrationData,
    token_id: BytesN<32>,
    current_epoch: u64,
    flow_in_amount: i128,
    flow_out_amount: i128,
) {
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::token_manager_wasm_hash(&env)
        }),
        migration_data.new_token_manager_wasm_hash,
        "token manager WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::interchain_token_wasm_hash(&env)
        }),
        migration_data.new_interchain_token_wasm_hash,
        "interchain token WASM hash should be updated"
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_in(&env, token_id.clone(), current_epoch)
        }),
        flow_in_amount
    );
    assert_eq!(
        env.as_contract(&its_client.address, || {
            storage::flow_out(&env, token_id, current_epoch)
        }),
        flow_out_amount
    );
}
