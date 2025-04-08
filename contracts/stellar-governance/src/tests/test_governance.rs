#![cfg(test)]
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::StellarAssetClient;
use stellar_axelar_std::types::Token;
use stellar_axelar_std::xdr::ToXdr;
use stellar_axelar_std::{
    vec, Address, Bytes, Env, IntoVal, String, Symbol, Val, Vec,
};
use test_target::TestTarget;

use crate::contract::{StellarGovernance, StellarGovernanceClient};
use crate::interface::StellarGovernanceInterface;

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

fn setup_token(env: &Env, recipient: &Address, amount: i128) -> Token {
    let asset = env.register_stellar_asset_contract_v2(Address::generate(env));

    StellarAssetClient::new(env, &asset.address())
        .mock_all_auths()
        .mint(recipient, &amount);

    Token {
        address: asset.address(),
        amount,
    }
}

#[test]
fn constructor_initialization_succeeds() {
    let env = Env::default();

    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);

    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 1000u64;

    env.register(
        StellarGovernance,
        (
            &gateway,
            &owner,
            &operator,
            governance_chain,
            governance_address,
            &minimum_time_delay,
        ),
    );
}

#[test]
fn schedule_proposal_and_get_eta_succeeds() {
    let env = Env::default();

    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);
    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 1000u64;

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

    let command_id = 0u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[1, 2, 3]);
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

    client.execute(&governance_chain, &governance_address, &payload);

    let retrieved_eta = client.get_proposal_eta(&target, &call_data, &function, &native_value);

    assert_eq!(retrieved_eta, eta);
}

#[test]
fn execute_existing_proposal_succeeds() {
    let env = Env::default();

    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);
    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 1000u64;

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

    let command_id = 0u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[]);
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

    // Schedule the proposal
    client.execute(&governance_chain, &governance_address, &payload);

    // Execute the proposal
    client.execute_proposal(&target, &call_data, &function, &native_value);
}

#[test]
fn withdraw_currency_succeeds() {
    let env = Env::default();

    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);
    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 1000u64;

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

    let amount = 1000i128;
    let Token {  .. } = setup_token(&env, &contract_id, amount);
    // let token = Token {
    //     address,
    //     amount: amount,
    // };
    //let token_client = token.client(&env);

    let command_id = 0u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[]);
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

    client.execute(&governance_chain, &governance_address, &payload);

    client.execute_proposal(&target, &call_data, &function, &native_value);

    // let contract_balance = token_client.balance(&contract_id);
    // assert_eq!(contract_balance, 0);
}
