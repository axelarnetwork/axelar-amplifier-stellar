use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::token::StellarAssetClient;
use stellar_axelar_std::xdr::ToXdr;
use stellar_axelar_std::{vec, Address, Bytes, Env, IntoVal, String, Symbol, Val, Vec};
use test_target::TestTarget;

use crate::contract::{AxelarGovernance, AxelarGovernanceClient};
use crate::types::CommandType;

pub mod test_target {
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

pub fn setup_client<'a>() -> (
    Env,
    AxelarGovernanceClient<'a>,
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

    (
        env,
        client,
        contract_id,
        governance_chain,
        governance_address,
        minimum_time_delay,
    )
}

pub fn setup_payload(
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

pub fn setup_token(env: &Env, contract_id: Address, native_value: i128) -> Address {
    let token = env.register_stellar_asset_contract_v2(contract_id.clone());
    StellarAssetClient::new(env, &token.address())
        .mock_all_auths()
        .mint(&contract_id, &native_value);
    token.address()
}

pub fn setup<'a>() -> (
    Env,
    AxelarGovernanceClient<'a>,
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
