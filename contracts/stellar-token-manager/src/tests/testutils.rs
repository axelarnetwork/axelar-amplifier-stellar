use stellar_axelar_std::testutils::Address as _;
use stellar_axelar_std::{Address, Env};

use crate::{TokenManager, TokenManagerClient};

pub struct TestConfig<'a> {
    pub env: Env,
    pub owner: Address,
    pub client: TokenManagerClient<'a>,
}

pub fn setup_env<'a>() -> TestConfig<'a> {
    let env = Env::default();

    let owner = Address::generate(&env);
    let contract_id = env.register(TokenManager, (owner.clone(),));
    let client = TokenManagerClient::<'a>::new(&env, &contract_id);

    TestConfig { env, owner, client }
}
