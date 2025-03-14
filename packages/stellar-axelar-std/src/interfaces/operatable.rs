use core::fmt::Debug;

use soroban_sdk::{contractclient, Address, Env};

use crate as stellar_axelar_std;
use crate::events::Event;
use crate::interfaces::storage;
use crate::IntoEvent;

#[contractclient(name = "OperatableClient")]
pub trait OperatableInterface {
    /// Returns the address of the contract's operator.
    fn operator(env: &Env) -> Address;

    /// Transfers operatorship of the contract to a new address.
    fn transfer_operatorship(env: &Env, new_operator: Address);
}

/// Default implementation of the [OperatableInterface] trait.
pub fn operator(env: &Env) -> Address {
    storage::operator::interfaces_operator(env)
}

/// Default implementation of the [OperatableInterface] trait. Ensures the current operator is authorized and emits an event after the transfer.
pub fn transfer_operatorship<T: OperatableInterface>(env: &Env, new_operator: Address) {
    let current_operator = T::operator(env);
    current_operator.require_auth();

    set_operator(env, &new_operator);

    OperatorshipTransferredEvent {
        previous_operator: current_operator,
        new_operator,
    }
    .emit(env);
}

/// Default implementation accompanying the [OperatableInterface] trait. This should never be part of a contract interface,
/// but allows contracts internally to set the operator.
pub fn set_operator(env: &Env, operator: &Address) {
    storage::operator::set_interfaces_operator(env, operator);
}

#[derive(Clone, Debug, PartialEq, Eq, IntoEvent)]
pub struct OperatorshipTransferredEvent {
    pub previous_operator: Address,
    pub new_operator: Address,
}

#[cfg(test)]
mod test {
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env};

    use crate::interfaces::testdata::Contract;
    use crate::interfaces::{OperatableClient, OperatorshipTransferredEvent};
    use crate::{assert_auth, assert_auth_err, events};

    fn prepare_client(env: &Env, operator: Option<Address>) -> OperatableClient {
        let owner = Address::generate(env);
        let contract_id = env.register(Contract, (owner, operator));
        OperatableClient::new(env, &contract_id)
    }

    #[test]
    fn operator_fails_if_operator_not_set() {
        let env = Env::default();
        let client = prepare_client(&env, None);

        assert!(client.try_operator().is_err());
    }

    #[test]
    fn operator_returns_correct_operator_when_set() {
        let env = Env::default();
        let operator = Address::generate(&env);
        let client = prepare_client(&env, Some(operator.clone()));

        assert_eq!(client.operator(), operator);
    }

    #[test]
    fn transfer_operatorship_fails_if_caller_is_not_operator() {
        let env = Env::default();
        let operator = Address::generate(&env);
        let client = prepare_client(&env, Some(operator));

        let new_operator = Address::generate(&env);
        assert_auth_err!(new_operator, client.transfer_operatorship(&new_operator));
    }

    #[test]
    fn transfer_operatorship_succeeds_if_caller_is_operator() {
        let env = Env::default();
        let operator = Address::generate(&env);
        let client = prepare_client(&env, Some(operator.clone()));

        assert_eq!(client.operator(), operator);

        let new_operator = Address::generate(&env);
        assert_auth!(operator, client.transfer_operatorship(&new_operator));

        goldie::assert!(events::fmt_last_emitted_event::<OperatorshipTransferredEvent>(&env));

        assert_eq!(client.operator(), new_operator);
    }
}
