use std::string::{String, ToString};
use std::{format, vec};

use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_gas_service::testutils::setup_gas_service;
use stellar_axelar_gas_service::AxelarGasServiceClient;
use stellar_axelar_gateway::testutils::{setup_gateway, TestSignerSet};
use stellar_axelar_gateway::AxelarGatewayClient;
use stellar_axelar_std::testutils::{AuthorizedFunction, AuthorizedInvocation};
use stellar_axelar_std::{Address, Env, IntoVal};

use crate::testutils::setup_its;
use crate::InterchainTokenServiceClient;

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

pub fn format_auths(
    auths: std::vec::Vec<(Address, AuthorizedInvocation)>,
    description: &str,
) -> String {
    let mut formatted_auths = format!("{:?}:\n", description.to_string());

    for (caller, invocation) in auths {
        let (client, method, args) = match invocation.function {
            AuthorizedFunction::Contract((client, method, args)) => (client, method, args),
            _ => panic!("Expected a contract function"),
        };
        formatted_auths.push_str(&format!("    caller: {:?}\n", caller));
        formatted_auths.push_str(&format!(
            "        invocation: {:?}.{:?}({:?})\n",
            client.to_string(),
            method.to_string(),
            args
        ));

        for sub_invocation in invocation.sub_invocations {
            formatted_auths.push_str(&format_auths(
                vec![(caller.clone(), sub_invocation)],
                "sub_invocation",
            ));
        }
    }

    formatted_auths
}
