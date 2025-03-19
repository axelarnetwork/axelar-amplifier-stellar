#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::{Address as _, BytesN as _};
use soroban_sdk::{
    bytes, contract, contractimpl, vec, Address, Bytes, BytesN, Env, IntoVal, String, Val, Vec,
};
use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_gas_service::event::GasPaidEvent;
use stellar_axelar_gas_service::testutils::setup_gas_token;
use stellar_axelar_gas_service::{AxelarGasService, AxelarGasServiceClient};
use stellar_axelar_operators::{AxelarOperators, AxelarOperatorsClient};
use stellar_axelar_std::events::{fmt_last_emitted_event, Event};
use stellar_axelar_std::interfaces::OwnableInterface;
use stellar_axelar_std::{assert_contract_err, interfaces, mock_auth, IntoEvent, Ownable};
use stellar_interchain_token::{InterchainToken, InterchainTokenClient};

use crate::error::ContractError;
use crate::types::FunctionCall;
use crate::{Multicall, MulticallClient};

#[macro_export]
macro_rules! construct_function_call {
    ($env:expr, $contract:expr, $approver:expr, $function:ident ( $($arg:expr),* $(,)? )) => {{
        FunctionCall {
            contract: $contract.clone(),
            approver: $approver.clone(),
            function: soroban_sdk::Symbol::new(&$env, stringify!($function)),
            args: vec![&$env, $($arg),*],
        }
    }};
}

#[contract]
#[derive(Ownable)]
pub struct TestTarget;

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ExecutedEvent {
    pub value: u32,
}

#[contractimpl]
impl TestTarget {
    pub fn __constructor(env: Env, owner: Address) {
        interfaces::set_owner(&env, &owner);
    }

    pub fn method(env: &Env, value: u32) {
        ExecutedEvent { value }.emit(env);
    }

    pub fn failing(_env: &Env) {
        panic!("This method should fail");
    }

    pub const fn failing_with_error(_env: &Env) -> Result<Val, ContractError> {
        Err(ContractError::FunctionCallFailed)
    }
}

pub struct TestConfig<'a> {
    pub env: Env,
    pub client: MulticallClient<'a>,
    pub target_id: Address,
    pub owner: Address,
}

fn setup<'a>() -> TestConfig<'a> {
    let env = Env::default();
    let owner = Address::generate(&env);
    let contract_id = env.register(Multicall, ());
    let client = MulticallClient::new(&env, &contract_id);

    let target_id = env.register(TestTarget, (owner.clone(),));

    TestConfig {
        env,
        client,
        target_id,
        owner,
    }
}

fn setup_token_metadata(env: &Env, name: &str, symbol: &str, decimal: u32) -> TokenMetadata {
    TokenMetadata {
        decimal,
        name: name.into_val(env),
        symbol: symbol.into_val(env),
    }
}

fn setup_token<'a>(env: &Env) -> (InterchainTokenClient<'a>, Address, Address) {
    let owner = Address::generate(env);
    let minter = Address::generate(env);
    let token_id: BytesN<32> = BytesN::<32>::random(env);
    let token_metadata = setup_token_metadata(env, "name", "symbol", 6);

    let token_contract_id = env.register(
        InterchainToken,
        (owner, minter.clone(), &token_id, token_metadata),
    );

    let token_client = InterchainTokenClient::new(env, &token_contract_id);
    (token_client, token_contract_id, minter)
}

#[test]
fn multicall_succeeds() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let operator_id = env.register(AxelarOperators, (&owner,));
    let operator_client = AxelarOperatorsClient::new(&env, &operator_id);

    let amount = IntoVal::<_, Val>::into_val(&1000i128, &env);
    let user = Address::generate(&env);
    let (token_client, token_contract_id, _) = setup_token(&env);

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            token_contract_id,
            token_client.owner(),
            mint(user.to_val(), amount)
        ),
        construct_function_call!(
            env,
            token_contract_id,
            token_client.owner(),
            balance(user.to_val())
        ),
        construct_function_call!(env, operator_id, owner, is_operator(owner.to_val())),
        construct_function_call!(env, operator_id, owner, add_operator(owner.to_val())),
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&42u32, &env))
        ),
        construct_function_call!(env, target_id, owner, owner()),
    ];

    let token_auth = mock_auth!(token_client.owner(), token_client.mint(user, &amount));
    let multicall_token_auth = mock_auth!(
        token_client.owner(),
        client.multicall(&function_calls),
        &[(token_auth.invoke).clone()]
    );

    let operators_auth = mock_auth!(owner, operator_client.add_operator(&owner));
    let multicall_operators_auth = mock_auth!(
        owner,
        client.multicall(&function_calls),
        &[(operators_auth.invoke).clone()]
    );

    client
        .mock_auths(&[
            multicall_token_auth.clone(),
            multicall_token_auth,
            multicall_operators_auth.clone(),
            multicall_operators_auth.clone(),
            multicall_operators_auth.clone(),
            multicall_operators_auth,
        ])
        .multicall(&function_calls);

    goldie::assert!(fmt_last_emitted_event::<ExecutedEvent>(&env));
}

#[test]
fn multicall_long_auth_chain_succeeds() {
    let TestConfig {
        env, client, owner, ..
    } = setup();

    let spender: Address = Address::generate(&env);
    let sender: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    let payload = bytes!(&env, 0x1234);

    let gas_service_id = env.register(AxelarGasService, (&owner, &operator));
    let gas_service_client = AxelarGasServiceClient::new(&env, &gas_service_id);
    let token = setup_gas_token(&env, &spender);
    let token_client = token.client(&env);

    let destination_chain: String = String::from_str(&env, "ethereum");
    let destination_address = Address::generate(&env).to_string();

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            gas_service_id,
            owner,
            pay_gas(
                sender.to_val(),
                destination_chain.to_val(),
                destination_address.to_val(),
                payload.to_val(),
                spender.to_val(),
                IntoVal::<_, Val>::into_val(&token, &env),
                Bytes::new(&env).to_val()
            )
        ),
    ];

    let transfer_token_auth = mock_auth!(
        spender,
        token_client.transfer(spender, gas_service_client.address, token.amount)
    );

    let pay_gas_auth = mock_auth!(
        spender,
        gas_service_client.pay_gas(
            sender,
            destination_chain,
            destination_address,
            payload,
            spender,
            token,
            Bytes::new(&env)
        ),
        &[(transfer_token_auth.invoke).clone()]
    );

    let multicall_auth = mock_auth!(
        owner,
        client.multicall(&function_calls),
        &[(pay_gas_auth.invoke).clone()]
    );

    client
        .mock_auths(&[multicall_auth, pay_gas_auth])
        .multicall(&function_calls);

    goldie::assert!(fmt_last_emitted_event::<GasPaidEvent>(&env));
}

#[test]
fn multicall_zero_call_succeeds() {
    let TestConfig { env, client, .. } = setup();

    let function_calls = Vec::new(&env);

    client.multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_no_auth_fails() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&42u32, &env))
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            transfer_ownership(new_owner.to_val())
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&0u32, &env))
        ),
    ];

    client.multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_incorrect_approver_auth_fails() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&42u32, &env))
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            transfer_ownership(new_owner.to_val())
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&0u32, &env))
        ),
    ];

    let test_client = TestTargetClient::new(&env, &target_id);

    let transfer_ownership_auth = mock_auth!(new_owner, test_client.transfer_ownership(&new_owner));
    let multicall_auth_chained = mock_auth!(
        new_owner,
        client.multicall(&function_calls),
        &[(transfer_ownership_auth.invoke).clone()]
    );
    let multicall_auth = mock_auth!(new_owner, client.multicall(&function_calls));

    client
        .mock_auths(&[
            multicall_auth.clone(),
            multicall_auth_chained,
            multicall_auth,
        ])
        .multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_incomplete_auth_fails() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&42u32, &env))
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            transfer_ownership(new_owner.to_val())
        ),
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&0u32, &env))
        ),
    ];

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    client
        .mock_auths(&[
            multicall_auth.clone(),
            multicall_auth.clone(),
            multicall_auth,
        ])
        .multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(WasmVm, InvalidAction)")]
fn multicall_fails_when_target_panics() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let function_calls = vec![
        &env,
        construct_function_call!(env, target_id, owner, failing()),
    ];

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    client
        .mock_auths(&[multicall_auth])
        .multicall(&function_calls);
}

#[test]
fn multicall_fails_when_target_returns_error() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let function_calls = vec![
        &env,
        construct_function_call!(env, target_id, owner, failing_with_error()),
    ];

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    assert_contract_err!(
        client
            .mock_auths(&[multicall_auth])
            .try_multicall(&function_calls),
        ContractError::FunctionCallFailed
    );
}

#[test]
fn multicall_fails_when_some_calls_returns_error() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let function_calls = vec![
        &env,
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&42u32, &env))
        ),
        construct_function_call!(env, target_id, owner, failing_with_error()),
        construct_function_call!(
            env,
            target_id,
            owner,
            method(IntoVal::<_, Val>::into_val(&0u32, &env))
        ),
    ];

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    assert_contract_err!(
        client
            .mock_auths(&[
                multicall_auth.clone(),
                multicall_auth.clone(),
                multicall_auth
            ])
            .try_multicall(&function_calls),
        ContractError::FunctionCallFailed
    );
}
