use contract_a::{ContractA, ContractAClient};
use contract_b::ContractB;
use contract_c::ContractC;
use stellar_axelar_std::Env;

mod contract_a {

    use stellar_axelar_std::{
        auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
        contract, contracterror, contractimpl, soroban_sdk, vec, Address, Env, IntoVal, Symbol,
    };

    use super::contract_b::ContractBClient;

    #[contract]
    pub struct ContractA;

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(u32)]
    pub enum ContractError {
        Failed = 1,
    }

    #[contractimpl]
    impl ContractA {
        pub fn call_b(env: Env, contract_b_address: Address, contract_c_address: Address) {
            // This function authorizes sub-contract calls that are made from
            // the next call A performs on behalf of the current contract.
            // Note, that these *do not* contain direct calls because they are
            // always authorized. So here we pre-authorize call of contract C
            // that will be performed by contract B.
            env.authorize_as_current_contract(vec![
                &env,
                InvokerContractAuthEntry::Contract(SubContractInvocation {
                    context: ContractContext {
                        contract: contract_c_address.clone(),
                        fn_name: Symbol::new(&env, "authorized_fn_c"),
                        args: (env.current_contract_address(),).into_val(&env),
                    },
                    // `sub_invocations` can be used to authorize even deeper
                    // calls.
                    sub_invocations: vec![&env],
                }),
            ]);
            let client = ContractBClient::new(&env, &contract_b_address);
            client.authorized_fn_b(&env.current_contract_address(), &contract_c_address);
        }
    }
}

mod contract_b {
    use stellar_axelar_std::{contract, contracterror, contractimpl, soroban_sdk, Address, Env};

    use super::contract_c::ContractCClient;

    #[contract]
    pub struct ContractB;

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(u32)]
    pub enum ContractError {
        Failed = 1,
    }

    #[contractimpl]
    impl ContractB {
        pub fn authorized_fn_b(env: Env, authorizer: Address, contract_c_address: Address) {
            authorizer.require_auth();
            let client = ContractCClient::new(&env, &contract_c_address);
            client.authorized_fn_c(&authorizer);
        }
    }
}
mod contract_c {

    use stellar_axelar_std::{contract, contracterror, contractimpl, soroban_sdk, Address, Env};

    #[contract]
    pub struct ContractC;

    #[contracterror]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(u32)]
    pub enum ContractError {
        Failed = 1,
    }

    #[contractimpl]
    impl ContractC {
        pub fn authorized_fn_c(_env: Env, authorizer: Address) {
            authorizer.require_auth();
        }
    }
}

#[test]
fn deep_auth() {
    let env = Env::default();
    let a_address = env.register_contract(None, ContractA);
    let b_address = env.register_contract(None, ContractB);
    let c_address = env.register_contract(None, ContractC);
    let client = ContractAClient::new(&env, &a_address);
    client.call_b(&b_address, &c_address);
}
