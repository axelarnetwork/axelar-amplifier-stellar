use soroban_sdk::Env;

use crate::testutils::{setup_gateway, TestSignerSet};
use crate::AxelarGatewayClient;

pub struct TestConfig<'a> {
    pub env: Env,
    pub signers: TestSignerSet,
    pub client: AxelarGatewayClient<'a>,
}

pub fn setup_env<'a>(previous_signers_retention: u64, num_signers: u64) -> TestConfig<'a> {
    let env = Env::default();
    let (signers, client) = setup_gateway(&env, previous_signers_retention, num_signers);

    TestConfig {
        env,
        signers,
        client,
    }
}
