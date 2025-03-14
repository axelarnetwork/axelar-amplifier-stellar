#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{
    contract, contractimpl, symbol_short, vec, Address, Env, IntoVal, Symbol, Val, Vec,
};
use stellar_axelar_std::events::{fmt_last_emitted_event, Event};
use stellar_axelar_std::interfaces::OwnableInterface;
use stellar_axelar_std::{assert_contract_err, interfaces, mock_auth, IntoEvent};

use crate::error::ContractError;
use crate::types::FunctionCall;
use crate::{Multicall, MulticallClient};

#[contract]
pub struct TestTarget;

#[derive(Debug, PartialEq, Eq, IntoEvent)]
pub struct ExecutedEvent {
    pub value: u32,
}

#[contractimpl]
impl OwnableInterface for TestTarget {
    fn owner(env: &Env) -> Address {
        interfaces::owner(env)
    }

    fn transfer_ownership(env: &Env, new_owner: Address) {
        interfaces::transfer_ownership::<Self>(env, new_owner);
    }
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

fn setup<'a>() -> (Env, MulticallClient<'a>, Address, Address) {
    let env = Env::default();
    let owner = Address::generate(&env);
    let contract_id = env.register(Multicall, ());
    let client = MulticallClient::new(&env, &contract_id);

    let target_id = env.register(TestTarget, (owner.clone(),));

    (env, client, target_id, owner)
}

#[test]
fn multicall_succeeds() {
    let (env, client, target, _) = setup();

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&42u32, &env)],
        },
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("owner"),
            args: vec![&env],
        },
    ];

    client.multicall(&function_calls);
    goldie::assert!(fmt_last_emitted_event::<ExecutedEvent>(&env));
}

#[test]
fn transfer_ownership_auth() {
    let (env, _, target, current_owner) = setup();
    let new_owner = Address::generate(&env);

    let test_client = TestTargetClient::new(&env, &target);
    let transfer_ownership_auth =
        mock_auth!(current_owner, test_client.transfer_ownership(&new_owner));
    test_client
        .mock_auths(&[transfer_ownership_auth])
        .transfer_ownership(&new_owner);
}

#[test]
fn multicall_auth_succeeds() {
    let (env, client, target, current_owner) = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&42u32, &env)],
        },
        FunctionCall {
            contract: target.clone(),
            function: Symbol::new(&env, "transfer_ownership"),
            args: vec![&env, new_owner.to_val()],
        },
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&0u32, &env)],
        },
    ];

    let test_client = TestTargetClient::new(&env, &target);

    let transfer_ownership_auth =
        mock_auth!(current_owner, test_client.transfer_ownership(&new_owner));

    client
        .mock_auths(&[transfer_ownership_auth])
        .multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_auth_no_auth_fails() {
    let (env, client, target, _) = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target.clone(),
            function: Symbol::new(&env, "transfer_ownership"),
            args: vec![&env, new_owner.to_val()],
        },
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&42u32, &env)],
        },
    ];

    client.multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_auth_incorrect_auth_fails() {
    let (env, client, target, _) = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target.clone(),
            function: Symbol::new(&env, "transfer_ownership"),
            args: vec![&env, new_owner.to_val()],
        },
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&42u32, &env)],
        },
    ];

    let test_client = TestTargetClient::new(&env, &target);

    let transfer_ownership_auth = mock_auth!(new_owner, test_client.transfer_ownership(&new_owner));

    client
        .mock_auths(&[transfer_ownership_auth])
        .multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(WasmVm, InvalidAction)")]
fn multicall_fails_when_target_panics() {
    let (env, client, target, _) = setup();

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target,
            function: symbol_short!("failing"),
            args: Vec::<Val>::new(&env),
        },
    ];

    client.multicall(&function_calls);
}

#[test]
fn multicall_fails_when_target_returns_error() {
    let (env, client, target, _) = setup();

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target,
            function: Symbol::new(&env, "failing_with_error"),
            args: Vec::<Val>::new(&env),
        },
    ];

    assert_contract_err!(
        client.try_multicall(&function_calls),
        ContractError::FunctionCallFailed
    );
}

#[test]
fn multicall_fails_when_some_calls_returns_error() {
    let (env, client, target, _) = setup();

    let function_calls = vec![
        &env,
        FunctionCall {
            contract: target.clone(),
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&42u32, &env)],
        },
        FunctionCall {
            contract: target.clone(),
            function: Symbol::new(&env, "failing_with_error"),
            args: Vec::<Val>::new(&env),
        },
        FunctionCall {
            contract: target,
            function: symbol_short!("method"),
            args: vec![&env, IntoVal::<_, Val>::into_val(&0u32, &env)],
        },
    ];
    assert_contract_err!(
        client.try_multicall(&function_calls),
        ContractError::FunctionCallFailed
    );
}
