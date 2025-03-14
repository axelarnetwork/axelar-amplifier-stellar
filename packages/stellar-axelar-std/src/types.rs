use soroban_sdk::{contracttype, token::TokenClient, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub address: Address,
    pub amount: i128,
}

impl Token {
    pub fn client<'a>(&self, env: &'a Env) -> TokenClient<'a> {
        TokenClient::new(env, &self.address)
    }
}
