#![cfg(test)]
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_contract_err, Address, Bytes, Env, String, Symbol};

use crate::contract::{AxelarGovernance, AxelarGovernanceClient};
use crate::error::ContractError;
use crate::tests::testutils::test_target::TestTarget;
use crate::tests::testutils::{setup_client, setup_payload, setup_token};
use crate::types::CommandType;

#[test]
fn schedule_same_proposal_twice_fails() {
    let (env, client, _contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let payload = setup_payload(
        &env,
        CommandType::ScheduleTimeLockProposal as u32,
        target,
        call_data,
        function,
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &payload);

    assert_contract_err!(
        client.try_execute(&governance_chain, &governance_address, &payload),
        ContractError::TimeLockAlreadyScheduled
    );
}

#[test]
fn execute_proposal_with_zero_eta_fails() {
    let env = Env::default();
    let gateway = Address::generate(&env);
    let owner = Address::generate(&env);
    let operator = Address::generate(&env);
    let governance_chain = String::from_str(&env, "test-chain");
    let governance_address = String::from_str(&env, "test-address");
    let minimum_time_delay = 0u64;

    let contract_id = env.register(
        AxelarGovernance,
        (
            &gateway,
            &owner,
            &operator,
            governance_chain.clone(),
            governance_address.clone(),
            &minimum_time_delay,
        ),
    );
    let client = AxelarGovernanceClient::new(&env, &contract_id);

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = 0u64;
    let token_address = setup_token(&env, contract_id, native_value);

    let payload = setup_payload(
        &env,
        CommandType::ScheduleTimeLockProposal as u32,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &payload);

    assert_contract_err!(
        client.try_execute_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address,
        ),
        ContractError::InvalidTimeLockHash
    );
}
