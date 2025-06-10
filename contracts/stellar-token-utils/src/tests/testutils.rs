#![cfg(test)]
extern crate alloc;
extern crate std;

use crate::{TokenUtils, TokenUtilsClient};
use stellar_axelar_std::Env;

macro_rules! address_strings {
    ($addresses:expr) => {
        $addresses
            .iter()
            .map(|addr| addr.to_string().to_string())
            .collect::<std::vec::Vec<std::string::String>>()
    };
}

pub(crate) use address_strings;

pub fn setup() -> (Env, TokenUtilsClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(TokenUtils, ());
    let client = TokenUtilsClient::new(&env, &contract_id);
    (env, client)
}
