#![cfg(test)]
use stellar_axelar_std::events::fmt_last_emitted_event;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{assert_contract_err, Address, Bytes, String, Symbol};

use crate::error::ContractError;
use crate::event::{
    OperatorProposalCancelledEvent, ProposalCancelledEvent, ProposalScheduledEvent,
};
use crate::tests::testutils::test_target::TestTarget;
use crate::tests::testutils::{setup, setup_client, setup_payload};
use crate::types::CommandType;

#[test]
fn schedule_proposal_and_get_eta_succeeds() {
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
        ..,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    goldie::assert!(fmt_last_emitted_event::<ProposalScheduledEvent>(&env));

    let retrieved_eta = client.get_proposal_eta(&target, &call_data, &function, &native_value);

    assert_eq!(retrieved_eta, eta);
}

#[test]
fn cancel_proposal_succeeds() {
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
        ..,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    let cancel_payload = setup_payload(
        &env,
        CommandType::CancelTimeLockProposal as u32,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &cancel_payload);

    goldie::assert!(fmt_last_emitted_event::<ProposalCancelledEvent>(&env));

    let retrieved_eta = client.get_proposal_eta(&target, &call_data, &function, &native_value);
    assert_eq!(retrieved_eta, 0);
}

#[test]
fn schedule_proposal_with_invalid_command_id_fails() {
    let (env, client, _contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[1, 2, 3]);
    let function = Symbol::new(&env, "call_target");
    let native_value = 0i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let payload = setup_payload(&env, 4u32, target, call_data, function, native_value, eta);

    assert_contract_err!(
        client.try_execute(&governance_chain, &governance_address, &payload),
        ContractError::InvalidCommandType
    );
}

#[test]
fn schedule_proposal_with_wrong_source_chain_fails() {
    let (env, client, _governance_chain, governance_address, payload, ..) = setup();

    let wrong_source_chain = String::from_str(&env, "wrong-chain");
    assert_contract_err!(
        client.try_execute(&wrong_source_chain, &governance_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
fn schedule_proposal_with_wrong_source_address_fails() {
    let (env, client, governance_chain, _governance_address, payload, ..) = setup();
    let wrong_source_address = String::from_str(&env, "wrong-address");
    assert_contract_err!(
        client.try_execute(&governance_chain, &wrong_source_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
fn cancel_unscheduled_proposal_fails() {
    let (env, client, _contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let payload = setup_payload(
        &env,
        CommandType::CancelTimeLockProposal as u32,
        target,
        call_data,
        function,
        native_value,
        eta,
    );

    assert_contract_err!(
        client.try_execute(&governance_chain, &governance_address, &payload),
        ContractError::TimeLockNotScheduled
    );
}

#[test]
fn toggle_operator_proposal_approval_succeeds() {
    let (env, client, _contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();
    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let approve_payload = setup_payload(
        &env,
        CommandType::ApproveOperatorProposal as u32,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &approve_payload);

    assert!(client.is_operator_proposal_approved(&target, &call_data, &function, &native_value));

    let cancel_payload = setup_payload(
        &env,
        CommandType::CancelOperatorApproval as u32,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &cancel_payload);

    goldie::assert!(fmt_last_emitted_event::<OperatorProposalCancelledEvent>(
        &env
    ));

    assert!(!client.is_operator_proposal_approved(&target, &call_data, &function, &native_value));
}

#[test]
fn transfer_operatorship_succeeds() {
    let (env, client, ..) = setup_client();
    let new_operator = Address::generate(&env);

    client
        .mock_all_auths()
        .transfer_operatorship_wrapper(&new_operator);

    assert_eq!(client.operator(), new_operator);
}
