use soroban_token_sdk::metadata::TokenMetadata;
use stellar_axelar_std::testutils::{Address as _, BytesN as _};
use stellar_axelar_std::{Address, BytesN, Env, String};

use crate::{InterchainToken, InterchainTokenClient};

pub const INITIAL_TOTAL_SUPPLY: u128 = 0;

pub struct TestConfig<'a> {
    pub env: Env,
    pub owner: Address,
    pub client: InterchainTokenClient<'a>,
}

pub fn setup_env<'a>() -> TestConfig<'a> {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_id = env.register(
        InterchainToken,
        (
            &owner,
            Some(owner.clone()),
            BytesN::<32>::random(&env),
            TokenMetadata {
                name: String::from_str(&env, "Token"),
                symbol: String::from_str(&env, "TOKEN"),
                decimal: 7,
            },
        ),
    );
    let client = InterchainTokenClient::new(&env, &contract_id);

    TestConfig { env, owner, client }
}
