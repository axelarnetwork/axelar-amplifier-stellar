#![cfg(test)]
extern crate std;

use crate::types::FunctionCall;
use crate::{Multicall, MulticallClient};
use stellar_axelar_std::events::fmt_last_emitted_event;
use stellar_axelar_std::mock_auth;
use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{soroban_sdk, vec, Address, Env, IntoVal, Vec};

#[macro_export]
macro_rules! function_call {
    ($approver:expr, $client:ident . $function:ident ( $($arg:expr),* $(,)? )) => {{
        FunctionCall {
            contract: $client.address.clone(),
            approver: $approver.clone(),
            function: soroban_sdk::Symbol::new(&$client.env, stringify!($function)),
            args: vec![&$client.env, $($arg.into_val(&$client.env)),*],
        }
    }};
}

mod test_bank {
    use stellar_axelar_std::{contract, contractimpl, Address, Env};
    use stellar_axelar_std::{contracterror, soroban_sdk};
    use stellar_axelar_std::{interfaces, Ownable};

    #[contracterror]
    #[derive(Debug, Eq, PartialEq)]
    #[repr(u32)]
    pub enum TestBankError {
        InsufficientBalance = 1,
    }

    #[contract]
    #[derive(Ownable)]
    pub struct TestBankContract;

    mod storage {
        use stellar_axelar_std::contractstorage;
        use stellar_axelar_std::soroban_sdk;
        #[contractstorage]
        enum DataKey {
            #[instance]
            #[value(u32)]
            Balance,
        }
    }

    #[contractimpl]
    impl TestBankContract {
        pub fn __constructor(env: Env, owner: Address) {
            interfaces::set_owner(&env, &owner);
            storage::set_balance(&env, &0u32);
        }

        pub fn balance(env: &Env) -> u32 {
            storage::balance(env)
        }

        pub fn deposit(env: &Env, amount: u32) {
            let owner = Self::owner(env);
            owner.require_auth();

            let current_balance = storage::balance(env);
            let new_balance = current_balance + amount;
            storage::set_balance(env, &new_balance);
        }

        pub fn withdraw(env: &Env, amount: u32) -> Result<(), TestBankError> {
            let owner = Self::owner(env);
            owner.require_auth();

            let current_balance = storage::balance(env);
            if current_balance < amount {
                return Err(TestBankError::InsufficientBalance);
            }
            let new_balance = current_balance - amount;
            storage::set_balance(env, &new_balance);
            Ok(())
        }
    }
}

mod test_target {
    use stellar_axelar_std::events::Event;
    use stellar_axelar_std::{contract, contractimpl, Address, Env, Val};
    use stellar_axelar_std::{contracterror, soroban_sdk};
    use stellar_axelar_std::{interfaces, IntoEvent, Ownable};

    #[contracterror]
    #[derive(Debug, Eq, PartialEq)]
    #[repr(u32)]
    pub enum TestTargetError {
        TestError = 1,
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

        pub fn failing_with_error(_env: &Env) -> Result<Val, TestTargetError> {
            Err(TestTargetError::TestError)
        }
    }
}

use test_bank::{TestBankContract, TestBankContractClient};
use test_target::{ExecutedEvent, TestTarget, TestTargetClient};

pub struct TestConfig<'a> {
    pub env: Env,
    pub client: MulticallClient<'a>,
    pub target_client: TestTargetClient<'a>,
    pub owner: Address,
}

fn setup<'a>() -> TestConfig<'a> {
    let env = Env::default();
    let owner = Address::generate(&env);
    let contract_id = env.register(Multicall, ());
    let client = MulticallClient::new(&env, &contract_id);

    let target_id = env.register(TestTarget, (owner.clone(),));
    let target_client = TestTargetClient::new(&env, &target_id);

    TestConfig {
        env,
        client,
        target_client,
        owner,
    }
}

#[test]
fn multicall_succeeds() {
    let TestConfig {
        env,
        client,
        target_client,
        owner,
        ..
    } = setup();

    let bank_id = env.register(TestBankContract, (owner.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let function_calls = vec![
        &env,
        function_call!(owner, bank_client.deposit(42u32)),
        function_call!(owner, bank_client.withdraw(10u32)),
        function_call!(owner, target_client.method(0u32)),
        function_call!(owner, target_client.owner()),
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
        target_client,
        owner,
        ..
    } = setup();

    let bank_owner1 = Address::generate(&env);
    let bank_owner2 = Address::generate(&env);
    let bank_id = env.register(TestBankContract, (bank_owner1.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let function_calls = vec![
        &env,
        function_call!(bank_owner1, bank_client.deposit(20u32)),
        function_call!(bank_owner1, bank_client.withdraw(10u32)),
        function_call!(bank_owner1, bank_client.transfer_ownership(bank_owner2)),
        function_call!(bank_owner2, bank_client.deposit(10u32)),
        function_call!(bank_owner2, bank_client.withdraw(20u32)),
        function_call!(owner, target_client.method(0u32)),
        function_call!(owner, target_client.owner()),
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
#[should_panic(expected = "HostError: Error(Contract, #1)")] // TestBankError::InsufficientBalance
fn multicall_fails_withdraw_more_than_balance() {
    let TestConfig {
        env, client, owner, ..
    } = setup();

    let bank_id = env.register(TestBankContract, (owner.clone(),));
    let bank_client = TestBankContractClient::new(&env, &bank_id);

    let function_calls = vec![
        &env,
        function_call!(owner, bank_client.deposit(10u32)),
        function_call!(owner, bank_client.withdraw(20u32)),
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

    client
        .mock_auths(&[
            multicall_deposit_auth,
            multicall_withdraw_auth,
            multicall_auth,
        ])
        .multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_no_auth_fails() {
    let TestConfig {
        env,
        client,
        target_client,
        owner,
        ..
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        function_call!(owner, target_client.method(42u32)),
        function_call!(owner, target_client.transfer_ownership(new_owner)),
        function_call!(owner, target_client.method(0u32)),
    ];

    client.multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn multicall_incorrect_approver_auth_fails() {
    let TestConfig {
        env,
        client,
        target_client,
        owner,
        ..
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        function_call!(owner, target_client.method(42u32)),
        function_call!(owner, target_client.transfer_ownership(new_owner)),
        function_call!(owner, target_client.method(0u32)),
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
        target_client,
        owner,
        ..
    } = setup();
    let new_owner = Address::generate(&env);

    let function_calls = vec![
        &env,
        function_call!(owner, target_client.method(42u32)),
        function_call!(owner, target_client.transfer_ownership(new_owner)),
        function_call!(owner, target_client.method(0u32)),
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
        target_client,
        owner,
        ..
    } = setup();

    let function_calls = vec![&env, function_call!(owner, target_client.failing())];

    client.mock_all_auths().multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // TestTargetError::TestError
fn multicall_fails_when_target_returns_error() {
    let TestConfig {
        env,
        client,
        target_client,
        owner,
        ..
    } = setup();

    let function_calls = vec![
        &env,
        function_call!(owner, target_client.failing_with_error()),
    ];

    client.mock_all_auths().multicall(&function_calls);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // TestTargetError::TestError
fn multicall_fails_when_some_calls_returns_error() {
    let TestConfig {
        env,
        client,
        target_client,
        owner,
        ..
    } = setup();

    let function_calls = vec![
        &env,
        function_call!(owner, target_client.method(42u32)),
        function_call!(owner, target_client.failing_with_error()),
        function_call!(owner, target_client.method(0u32)),
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
