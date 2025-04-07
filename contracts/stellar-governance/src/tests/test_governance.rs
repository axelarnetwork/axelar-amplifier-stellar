#![cfg(test)]

mod test_target {
    use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Env};

    #[contract]
    pub struct TestTarget;

    #[contractimpl]
    impl TestTarget {
        pub fn call_target(env: Env) -> bool {
            true
        }
    }
}
