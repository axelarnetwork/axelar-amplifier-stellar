#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{contract, contractimpl, vec, Address, Env, IntoVal, Val, Vec};
use stellar_axelar_std::events::{fmt_last_emitted_event, Event};
use stellar_axelar_std::interfaces::OwnableInterface;
use stellar_axelar_std::{assert_contract_err, interfaces, mock_auth, IntoEvent, Ownable};

use crate::error::ContractError;
use crate::tests::bank_contract::{TestBankContract, TestBankContractClient};
use crate::tests::subscriber_contract::{TestSubscriptionContract, TestSubscriptionContractClient};
use crate::types::FunctionCall;
use crate::{Multicall, MulticallClient};

#[macro_export]
macro_rules! construct_function_call {
    ($contract:expr, $approver:expr, $client:ident . $function:ident ( $($arg:expr),* $(,)? )) => {{
        FunctionCall {
            contract: $contract.clone(),
            approver: $approver.clone(),
            function: soroban_sdk::Symbol::new(&$client.env, stringify!($function)),
            args: vec![&$client.env, $($arg.into_val(&$client.env)),*],
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

#[test]
fn multicall_succeeds() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let bank_id = env.register(TestBankContract, (owner.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let subscriber_id = env.register(TestSubscriptionContract, (owner.clone(),));
    let subscriber_client = TestSubscriptionContractClient::new(&env, &subscriber_id);

    let function_calls = vec![
        &env,
        construct_function_call!(bank_id, owner, bank_client.deposit(42u32)),
        construct_function_call!(bank_id, owner, bank_client.withdraw(10u32)),
        construct_function_call!(target_id, owner, client.method(0u32)),
        construct_function_call!(target_id, owner, client.owner()),
        construct_function_call!(subscriber_id, owner, subscriber_client.subscribe(owner)),
        construct_function_call!(subscriber_id, owner, subscriber_client.is_subscribed(owner)),
    ];

    let bank_deposit_auth = mock_auth!(owner, bank_client.deposit(42u32));
    let bank_withdraw_auth = mock_auth!(owner, bank_client.withdraw(10u32));
    let multicall_deposit_auth = mock_auth!(
        owner,
        client.multicall(&function_calls),
        &[(bank_deposit_auth.invoke).clone()]
    );
    let multicall_withdraw_auth = mock_auth!(
        owner,
        client.multicall(&function_calls),
        &[(bank_withdraw_auth.invoke).clone()]
    );

    let subscriber_subscribe_auth = mock_auth!(owner, subscriber_client.subscribe(&owner));
    let multicall_subscribe_auth = mock_auth!(
        owner,
        client.multicall(&function_calls),
        &[(subscriber_subscribe_auth.invoke).clone()]
    );

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    client
        .mock_auths(&[
            multicall_deposit_auth,
            multicall_withdraw_auth,
            multicall_auth.clone(),
            multicall_auth.clone(),
            multicall_subscribe_auth,
            multicall_auth,
        ])
        .multicall(&function_calls);

    goldie::assert!(fmt_last_emitted_event::<ExecutedEvent>(&env));
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
        construct_function_call!(target_id, owner, client.method(42u32)),
        construct_function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        construct_function_call!(target_id, owner, client.method(0u32)),
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
        construct_function_call!(target_id, owner, client.method(42u32)),
        construct_function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        construct_function_call!(target_id, owner, client.method(0u32)),
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
        construct_function_call!(target_id, owner, client.method(42u32)),
        construct_function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        construct_function_call!(target_id, owner, client.method(0u32)),
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
        construct_function_call!(target_id, owner, client.failing()),
    ];

    client.mock_all_auths().multicall(&function_calls);
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
        construct_function_call!(target_id, owner, client.failing_with_error()),
    ];

    assert_contract_err!(
        client.mock_all_auths().try_multicall(&function_calls),
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
        construct_function_call!(target_id, owner, client.method(42u32)),
        construct_function_call!(target_id, owner, client.failing_with_error()),
        construct_function_call!(target_id, owner, client.method(0u32)),
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
