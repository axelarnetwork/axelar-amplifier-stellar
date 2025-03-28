#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, IntoVal, Vec};
use stellar_axelar_std::events::fmt_last_emitted_event;
use stellar_axelar_std::{assert_contract_err, mock_auth};

use crate::error::ContractError;
use crate::types::FunctionCall;
use crate::{Multicall, MulticallClient};

#[macro_export]
macro_rules! function_call {
    ($contract:expr, $approver:expr, $client:ident . $function:ident ( $($arg:expr),* $(,)? )) => {{
        FunctionCall {
            contract: $contract.clone(),
            approver: $approver.clone(),
            function: soroban_sdk::Symbol::new(&$client.env, stringify!($function)),
            args: vec![&$client.env, $($arg.into_val(&$client.env)),*],
        }
    }};
}

mod test_bank {
    use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};
    use stellar_axelar_std::{interfaces, Ownable};

    use crate::error::ContractError;

    #[contract]
    #[derive(Ownable)]
    pub struct TestBankContract;

    #[contractimpl]
    impl TestBankContract {
        const BALANCE_KEY: Symbol = symbol_short!("balance");

        pub fn __constructor(env: Env, owner: Address) {
            interfaces::set_owner(&env, &owner);
            env.storage().instance().set(&Self::BALANCE_KEY, &0u32);
        }

        pub fn balance(env: &Env) -> u32 {
            env.storage()
                .instance()
                .get(&Self::BALANCE_KEY)
                .unwrap_or(0u32)
        }

        pub fn deposit(env: &Env, amount: u32) {
            let owner = Self::owner(env);
            owner.require_auth();

            let current_balance: u32 = env
                .storage()
                .instance()
                .get(&Self::BALANCE_KEY)
                .unwrap_or(0u32);
            let new_balance = current_balance + amount;
            env.storage()
                .instance()
                .set(&Self::BALANCE_KEY, &new_balance);
        }

        pub fn withdraw(env: &Env, amount: u32) -> Result<(), ContractError> {
            let owner = Self::owner(env);
            owner.require_auth();

            let current_balance: u32 = env
                .storage()
                .instance()
                .get(&Self::BALANCE_KEY)
                .unwrap_or(0u32);
            if current_balance < amount {
                return Err(ContractError::FunctionCallFailed);
            }
            let new_balance = current_balance - amount;
            env.storage()
                .instance()
                .set(&Self::BALANCE_KEY, &new_balance);
            Ok(())
        }
    }
}

mod test_target {
    use soroban_sdk::{contract, contractimpl, Address, Env, Val};
    use stellar_axelar_std::events::Event;
    use stellar_axelar_std::{interfaces, IntoEvent, Ownable};

    use crate::error::ContractError;

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
}

use test_bank::{TestBankContract, TestBankContractClient};
use test_target::{ExecutedEvent, TestTarget};

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

    let function_calls = vec![
        &env,
        function_call!(bank_id, owner, bank_client.deposit(42u32)),
        function_call!(bank_id, owner, bank_client.withdraw(10u32)),
        function_call!(target_id, owner, client.method(0u32)),
        function_call!(target_id, owner, client.owner()),
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

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    client
        .mock_auths(&[
            multicall_deposit_auth,
            multicall_withdraw_auth,
            multicall_auth.clone(),
            multicall_auth.clone(),
            multicall_auth,
        ])
        .multicall(&function_calls);

    goldie::assert!(fmt_last_emitted_event::<ExecutedEvent>(&env));
}

#[test]
fn multicall_succeeds_different_approvers() {
    let TestConfig {
        env,
        client,
        target_id,
        owner,
    } = setup();

    let bank_owner1 = Address::generate(&env);
    let bank_owner2 = Address::generate(&env);
    let bank_id = env.register(TestBankContract, (bank_owner1.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let function_calls = vec![
        &env,
        function_call!(bank_id, bank_owner1, bank_client.deposit(20u32)),
        function_call!(bank_id, bank_owner1, bank_client.withdraw(10u32)),
        function_call!(
            bank_id,
            bank_owner1,
            bank_client.transfer_ownership(bank_owner2)
        ),
        function_call!(bank_id, bank_owner2, bank_client.deposit(10u32)),
        function_call!(bank_id, bank_owner2, bank_client.withdraw(20u32)),
        function_call!(target_id, owner, client.method(0u32)),
        function_call!(target_id, owner, client.owner()),
    ];

    let bank_old_owner_deposit_auth = mock_auth!(bank_owner1, bank_client.deposit(20u32));
    let bank_old_owner_withdraw_auth = mock_auth!(bank_owner1, bank_client.withdraw(10u32));
    let bank_transfer_ownership_auth =
        mock_auth!(bank_owner1, bank_client.transfer_ownership(bank_owner2));
    let bank_new_owner_deposit_auth = mock_auth!(bank_owner2, bank_client.deposit(10u32));
    let bank_new_owner_withdraw_auth = mock_auth!(bank_owner2, bank_client.withdraw(20u32));

    let multicall_old_owner_deposit_auth = mock_auth!(
        bank_owner1,
        client.multicall(&function_calls),
        &[(bank_old_owner_deposit_auth.invoke).clone()]
    );
    let multicall_old_owner_withdraw_auth = mock_auth!(
        bank_owner1,
        client.multicall(&function_calls),
        &[(bank_old_owner_withdraw_auth.invoke).clone()]
    );

    let multicall_transfer_ownership_auth = mock_auth!(
        bank_owner1,
        client.multicall(&function_calls),
        &[(bank_transfer_ownership_auth.invoke).clone()]
    );

    let multicall_new_owner_deposit_auth = mock_auth!(
        bank_owner2,
        client.multicall(&function_calls),
        &[(bank_new_owner_deposit_auth.invoke).clone()]
    );
    let multicall_new_owner_withdraw_auth = mock_auth!(
        bank_owner2,
        client.multicall(&function_calls),
        &[(bank_new_owner_withdraw_auth.invoke).clone()]
    );

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    client
        .mock_auths(&[
            multicall_old_owner_deposit_auth,
            multicall_old_owner_withdraw_auth,
            multicall_transfer_ownership_auth,
            multicall_new_owner_deposit_auth,
            multicall_new_owner_withdraw_auth,
            multicall_auth.clone(),
            multicall_auth.clone(),
            // multicall_auth,
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
fn multicall_fails_withdraw_more_than_balance() {
    let TestConfig {
        env, client, owner, ..
    } = setup();

    let bank_id = env.register(TestBankContract, (owner.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let function_calls = vec![
        &env,
        function_call!(bank_id, owner, bank_client.deposit(10u32)),
        function_call!(bank_id, owner, bank_client.withdraw(20u32)),
    ];

    let bank_deposit_auth = mock_auth!(owner, bank_client.deposit(10u32));
    let bank_withdraw_auth = mock_auth!(owner, bank_client.withdraw(20u32));
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

    let multicall_auth = mock_auth!(owner, client.multicall(&function_calls));

    assert_contract_err!(
        client
            .mock_auths(&[
                multicall_deposit_auth,
                multicall_withdraw_auth,
                multicall_auth,
            ])
            .try_multicall(&function_calls),
        ContractError::FunctionCallFailed
    );
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
        function_call!(target_id, owner, client.method(42u32)),
        function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        function_call!(target_id, owner, client.method(0u32)),
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
        function_call!(target_id, owner, client.method(42u32)),
        function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        function_call!(target_id, owner, client.method(0u32)),
    ];

    // Skip the TestTargetClient usage and just set up the multicall auth directly
    let multicall_auth = mock_auth!(new_owner, client.multicall(&function_calls));

    // This should fail because new_owner doesn't have the right to call these functions
    client
        .mock_auths(&[
            multicall_auth.clone(),
            multicall_auth.clone(),
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
        function_call!(target_id, owner, client.method(42u32)),
        function_call!(target_id, owner, client.transfer_ownership(new_owner)),
        function_call!(target_id, owner, client.method(0u32)),
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

    let function_calls = vec![&env, function_call!(target_id, owner, client.failing())];

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
        function_call!(target_id, owner, client.failing_with_error()),
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
        function_call!(target_id, owner, client.method(42u32)),
        function_call!(target_id, owner, client.failing_with_error()),
        function_call!(target_id, owner, client.method(0u32)),
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
