#![cfg(test)]
use stellar_axelar_std::events::fmt_last_emitted_event;
use stellar_axelar_std::testutils::{Address as _, Ledger as _};
use stellar_axelar_std::{assert_auth_err, assert_contract_err, Address, Bytes, Symbol};

use crate::error::ContractError;
use crate::event::{
    OperatorProposalApprovedEvent, OperatorProposalExecutedEvent, ProposalExecutedEvent,
    ProposalScheduledEvent,
};
use crate::tests::testutils::test_target::TestTarget;
use crate::tests::testutils::{setup, setup_client, setup_payload, setup_token};
use crate::types::CommandType;

#[test]
fn execute_proposal_succeeds() {
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
        token_address,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    env.ledger().set_timestamp(eta);

    client.execute_proposal(
        &target,
        &call_data,
        &function,
        &native_value,
        &token_address,
    );

    goldie::assert!(fmt_last_emitted_event::<ProposalExecutedEvent>(&env));
}

#[test]
#[should_panic(expected = "HostError: Error(Storage, MissingValue)")]
fn execute_proposal_with_invalid_target_address_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let invalid_target = Address::generate(&env);
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;
    let token_address = setup_token(&env, contract_id, native_value);

    let payload = setup_payload(
        &env,
        CommandType::ScheduleTimeLockProposal as u32,
        invalid_target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &payload);

    env.ledger().set_timestamp(eta);

    goldie::assert!(fmt_last_emitted_event::<ProposalScheduledEvent>(&env));

    client.execute_proposal(
        &invalid_target,
        &call_data,
        &function,
        &native_value,
        &token_address,
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Context, MissingValue)")]
fn execute_proposal_with_invalid_function_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::from_slice(&env, &[1, 2, 3]);
    let function = Symbol::new(&env, "invalid_function");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;
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

    env.ledger().set_timestamp(eta);

    goldie::assert!(fmt_last_emitted_event::<ProposalScheduledEvent>(&env));

    client.execute_proposal(
        &target,
        &call_data,
        &function,
        &native_value,
        &token_address,
    );
}

#[test]
fn execute_proposal_time_lock_not_ready_fails() {
    let (
        _env,
        client,
        governance_chain,
        governance_address,
        payload,
        target,
        call_data,
        function,
        native_value,
        _eta,
        token_address,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

    assert_contract_err!(
        client.try_execute_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address,
        ),
        ContractError::TimeLockNotReady
    );
}

#[test]
fn execute_proposal_insufficient_balance_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let command_id = CommandType::ScheduleTimeLockProposal as u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let token_address = setup_token(&env, contract_id, 0i128);

    let payload = setup_payload(
        &env,
        command_id,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &payload);

    env.ledger().set_timestamp(eta);

    goldie::assert!(fmt_last_emitted_event::<ProposalScheduledEvent>(&env));

    assert_contract_err!(
        client.try_execute_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address,
        ),
        ContractError::InsufficientBalance
    );
}

#[test]
fn approve_and_execute_operator_proposal_succeeds() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;
    let token_address = setup_token(&env, contract_id, native_value);
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

    client.mock_all_auths().execute_operator_proposal(
        &target,
        &call_data,
        &function,
        &native_value,
        &token_address,
    );

    goldie::assert!(fmt_last_emitted_event::<OperatorProposalExecutedEvent>(
        &env
    ));
}

#[test]
fn execute_unapproved_operator_proposal_fails() {
    let (env, client, contract_id, ..) = setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let token_address = setup_token(&env, contract_id, native_value);

    assert_contract_err!(
        client.mock_all_auths().try_execute_operator_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address,
        ),
        ContractError::OperatorProposalNotApproved
    );
}

#[test]
fn execute_operator_proposal_by_non_operator_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;
    let token_address = setup_token(&env, contract_id, native_value);

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

    goldie::assert!(fmt_last_emitted_event::<OperatorProposalApprovedEvent>(
        &env
    ));

    let random_address = Address::generate(&env);
    assert_auth_err!(
        random_address,
        client.execute_operator_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address
        )
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Context, MissingValue)")]
fn execute_operator_proposal_with_invalid_function_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "invalid_function");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;
    let token_address = setup_token(&env, contract_id, native_value);

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

    goldie::assert!(fmt_last_emitted_event::<OperatorProposalApprovedEvent>(
        &env
    ));

    client.mock_all_auths().execute_operator_proposal(
        &target,
        &call_data,
        &function,
        &native_value,
        &token_address,
    );
}

#[test]
fn execute_operator_proposal_insufficient_balance_fails() {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let command_id = CommandType::ApproveOperatorProposal as u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let token_address = setup_token(&env, contract_id, 0i128);

    let payload = setup_payload(
        &env,
        command_id,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

    client.execute(&governance_chain, &governance_address, &payload);

    goldie::assert!(fmt_last_emitted_event::<OperatorProposalApprovedEvent>(
        &env
    ));

    assert_contract_err!(
        client.mock_all_auths().try_execute_operator_proposal(
            &target,
            &call_data,
            &function,
            &native_value,
            &token_address,
        ),
        ContractError::InsufficientBalance
    );
}
