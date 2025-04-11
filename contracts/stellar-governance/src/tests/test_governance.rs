#![cfg(test)]
use stellar_axelar_std::events::fmt_last_emitted_event;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::StellarAssetClient;
use stellar_axelar_std::xdr::ToXdr;
use stellar_axelar_std::{
    assert_auth_err, assert_contract_err, vec, Address, Bytes, Env, IntoVal, String, Symbol, Val,
    Vec,
};
use test_target::TestTarget;

use crate::contract::{StellarGovernance, StellarGovernanceClient};
use crate::error::ContractError;
use crate::event::{
    OperatorProposalApprovedEvent, OperatorProposalCancelledEvent, OperatorProposalExecutedEvent,
    ProposalCancelledEvent, ProposalExecutedEvent, ProposalScheduledEvent,
};
use crate::types::CommandType;

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

fn setup_client<'a>() -> (
    Env,
    StellarGovernanceClient<'a>,
    Address,
    String,
    String,
    u64,
) {
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
        contract_id,
        governance_chain,
        governance_address,
        minimum_time_delay,
    )
}

fn setup_payload(
    env: &Env,
    command_id: u32,
    target: Address,
    call_data: Bytes,
    function: Symbol,
    native_value: i128,
    eta: u64,
) -> Bytes {
    let params: Vec<Val> = vec![
        &env,
        command_id.into_val(env),
        target.into_val(env),
        call_data.into_val(env),
        function.into_val(env),
        native_value.into_val(env),
        eta.into_val(env),
    ];
    params.to_xdr(env)
}

fn setup_token(env: &Env, contract_id: Address, native_value: i128) -> Address {
    let token = env.register_stellar_asset_contract_v2(contract_id.clone());
    StellarAssetClient::new(env, &token.address())
        .mock_all_auths()
        .mint(&contract_id, &native_value);
    token.address()
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
    Address,
) {
    let (env, client, contract_id, governance_chain, governance_address, minimum_time_delay) =
        setup_client();

    let command_id = CommandType::ScheduleTimeLockProposal as u32;
    let target = env.register(TestTarget, ());
    let call_data = Bytes::new(&env);
    let function = Symbol::new(&env, "call_target");
    let native_value = 1000i128;
    let eta = env.ledger().timestamp() + minimum_time_delay;

    let token_address = setup_token(&env, contract_id, native_value);

    let payload = setup_payload(
        &env,
        command_id,
        target.clone(),
        call_data.clone(),
        function.clone(),
        native_value,
        eta,
    );

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
        token_address,
    )
}

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
fn execute_existing_proposal_succeeds() {
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
        _eta,
        token_address,
    ) = setup();

    client.execute(&governance_chain, &governance_address, &payload);

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
fn execute_with_invalid_command_id_fails() {
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
fn execute_with_wrong_source_chain_fails() {
    let (env, client, _governance_chain, governance_address, payload, ..) = setup();

    let wrong_source_chain = String::from_str(&env, "wrong-chain");
    assert_contract_err!(
        client.try_execute(&wrong_source_chain, &governance_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
fn execute_with_wrong_source_address_fails() {
    let (env, client, governance_chain, _governance_address, payload, ..) = setup();
    let wrong_source_address = String::from_str(&env, "wrong-address");
    assert_contract_err!(
        client.try_execute(&governance_chain, &wrong_source_address, &payload),
        ContractError::NotGovernance
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Storage, MissingValue)")]
fn execute_with_invalid_target_address_fails() {
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
fn operator_proposal_approval_status_changes() {
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
fn execute_unapproved_operator_proposal_fails() {
    let (env, client, contract_id, _governance_chain, _governance_address, _minimum_time_delay) =
        setup_client();

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
fn transfer_operatorship_succeeds() {
    let (env, client, _contract_id, _governance_chain, _governance_address, _minimum_time_delay) =
        setup_client();
    let new_operator = Address::generate(&env);

    client
        .mock_all_auths()
        .transfer_operatorship_wrapper(&new_operator);

    assert_eq!(client.operator(), new_operator);
}
