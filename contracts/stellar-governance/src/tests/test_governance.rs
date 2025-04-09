#![cfg(test)]
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::xdr::ToXdr;
use stellar_axelar_std::{
    assert_contract_err, vec, Address, Bytes, Env, IntoVal, String, Symbol, Val, Vec,
};
use test_target::TestTarget;

use crate::contract::{StellarGovernance, StellarGovernanceClient};
use crate::error::ContractError;

mod test_target {
    use stellar_axelar_std::{contract, contractimpl, soroban_sdk};

    #[contract]
    pub struct TestTarget;

    #[contractimpl]
    impl TestTarget {
        pub const fn call_target() -> bool {
            true
        }
    }
}

fn setup_client<'a>() -> (Env, StellarGovernanceClient<'a>, String, String, u64) {
    let env = Env::default();
    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);
    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 10u64;

    let contract_id = env.register(
        StellarGovernance,
        (
            &gateway,
            &owner,
            &operator,
            governance_chain.clone(),
            governance_address.clone(),
            &minimum_time_delay,
        ),
    );
    let client = StellarGovernanceClient::new(&env, &contract_id);

    (
        env,
        client,
        governance_chain,
        governance_address,
        minimum_time_delay,
    )
}

fn setup<'a>() -> (
    Env,
    StellarGovernanceClient<'a>,
    String,
    String,
    Bytes,
    Address,
    Bytes,
    Symbol,
    i128,
    u64,
) {
    let (env, client, governance_chain, governance_address, minimum_time_delay) = setup_client();

    let command_id = 0u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 0i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let params: Vec<Val> = vec![
        &env,
        command_id.into_val(&env),
        target.into_val(&env),
        call_data.into_val(&env),
        function.into_val(&env),
        native_value.into_val(&env),
        eta.into_val(&env),
    ];
    let payload = params.to_xdr(&env);

    (
        env,
        client,
        governance_chain,
        governance_address,
        payload,
        target,
        call_data,
        function,
        native_value,
        eta,
    )
}

#[test]
fn schedule_proposal_and_get_eta_succeeds() {
    let (
        ..,
        client,
        governance_chain,
        governance_address,
        payload,
        target,
        call_data,
        function,
        native_value,
        eta,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    let retrieved_eta = client.get_proposal_eta(&target, &call_data, &function, &native_value);

    assert_eq!(retrieved_eta, eta);
}

#[test]
fn execute_existing_proposal_succeeds() {
    let (
        ..,
        client,
        governance_chain,
        governance_address,
        payload,
        target,
        call_data,
        function,
        native_value,
        _eta,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    client.execute_proposal(&target, &call_data, &function, &native_value);
}

#[test]
fn cancel_existing_proposal_succeeds() {
    let (
        env,
        client,
        governance_chain,
        governance_address,
        payload,
        target,
        call_data,
        function,
        native_value,
        eta,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    let cancel_params: Vec<Val> = vec![
        &env,
        1u32.into_val(&env),
        target.into_val(&env),
        call_data.into_val(&env),
        function.into_val(&env),
        native_value.into_val(&env),
        eta.into_val(&env),
    ];
    let cancel_payload = cancel_params.to_xdr(&env);

    client.execute(&governance_chain, &governance_address, &cancel_payload);

    let retrieved_eta = client.get_proposal_eta(&target, &call_data, &function, &native_value);
    assert_eq!(retrieved_eta, 0);
}

#[test]
fn execute_with_invalid_command_id_fails() {
    let (env, client, governance_chain, governance_address, minimum_time_delay) = setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[1, 2, 3]);
    let function = Symbol::new(&env, "call_target");
    let native_value = 0i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let params: Vec<Val> = vec![
        &env,
        4u32.into_val(&env),
        target.into_val(&env),
        call_data.into_val(&env),
        function.into_val(&env),
        native_value.into_val(&env),
        eta.into_val(&env),
    ];
    let payload = params.to_xdr(&env);

    assert_contract_err!(
        client.try_execute(&governance_chain, &governance_address, &payload),
        ContractError::InvalidCommandType
    );
}

#[test]
fn execute_with_wrong_source_chain_fails() {
    let (
        env,
        client,
        _governance_chain,
        governance_address,
        payload,
        _target,
        _call_data,
        _function,
        _native_value,
        _eta,
    ) = setup();

    let wrong_source_chain = String::from_str(&env, "wrong-chain");
    assert_contract_err!(
        client.try_execute(&wrong_source_chain, &governance_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
fn execute_with_wrong_source_address_fails() {
    let (
        env,
        client,
        governance_chain,
        _governance_address,
        payload,
        _target,
        _call_data,
        _function,
        _native_value,
        _eta,
    ) = setup();
    let wrong_source_address = String::from_str(&env, "wrong-address");
    assert_contract_err!(
        client.try_execute(&governance_chain, &wrong_source_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Context, MissingValue)")]
fn execute_proposal_with_invalid_function_fails() {
    let (env, client, governance_chain, governance_address, minimum_time_delay) = setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[1, 2, 3]);
    let function = Symbol::new(&env, "invalid_function");
    let native_value = 0i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let params: Vec<Val> = vec![
        &env,
        0u32.into_val(&env),
        target.into_val(&env),
        call_data.into_val(&env),
        function.into_val(&env),
        native_value.into_val(&env),
        eta.into_val(&env),
    ];
    let payload = params.to_xdr(&env);

    client.execute(&governance_chain, &governance_address, &payload);

    client.execute_proposal(&target, &call_data, &function, &native_value);
}
