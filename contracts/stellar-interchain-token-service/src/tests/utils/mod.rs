use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_gas_service::testutils::setup_gas_service;
use stellar_axelar_gas_service::AxelarGasServiceClient;
use stellar_axelar_gateway::testutils::{setup_gateway, TestSignerSet};
use stellar_axelar_gateway::AxelarGatewayClient;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{Address, Bytes, BytesN, Env, IntoVal, String};

use crate::testutils::setup_its;
use crate::InterchainTokenServiceClient;

pub const INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX: i32 = -4;
pub const INTERCHAIN_TOKEN_DEPLOYED_WITHOUT_GAS_TOKEN_EVENT_IDX: i32 = -2;
pub const INTERCHAIN_TOKEN_DEPLOYED_NO_SUPPLY_EVENT_IDX: i32 =
    INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX + 1;
pub const TOKEN_MANAGER_DEPLOYED_EVENT_IDX: i32 = INTERCHAIN_TOKEN_DEPLOYED_EVENT_IDX + 1;

pub const TEST_SALT: [u8; 32] = [1; 32];
pub const TEST_DESTINATION_CHAIN: &str = "ethereum";
pub const TEST_DESTINATION_TOKEN_ADDRESS: [u8; 32] = [2; 32];
pub const TEST_TRANSFER_AMOUNT: i128 = 1000;
pub const TEST_TRANSFER_DESTINATION_ADDRESS: [u8; 32] = [3; 32];
pub const TEST_TRANSFER_DESTINATION_CHAIN: &str = "avalanche";

pub struct TestData {
    pub deployer: Address,
    pub token: stellar_axelar_std::testutils::StellarAssetContract,
    pub salt: BytesN<32>,
    pub destination_chain: String,
    pub destination_token_address: Bytes,
}

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

pub fn setup_test_data(env: &stellar_axelar_std::Env) -> TestData {
    let deployer = Address::generate(env);
    let owner = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(owner);
    let salt = BytesN::<32>::from_array(env, &TEST_SALT);
    let destination_chain = String::from_str(env, TEST_DESTINATION_CHAIN);
    let destination_token_address = Bytes::from_array(env, &TEST_DESTINATION_TOKEN_ADDRESS);

    TestData {
        deployer,
        token,
        salt,
        destination_chain,
        destination_token_address,
    }
}
