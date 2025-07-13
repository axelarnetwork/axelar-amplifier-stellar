use stellar_axelar_std::{vec, Address, Env, IntoVal, Symbol, Val};
use stellar_token_manager::TokenManagerClient;

pub trait TokenManagerClientExt {
    /// Transfer `amount` of tokens from the token manager to `recipient`.
    fn transfer(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128);

    /// Mint `amount` of tokens to `recipient`.
    fn mint(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128);

    /// Approve the service contract to spend tokens on behalf of the token manager.
    fn approve_service(&self, env: &Env, token_address: &Address, service_address: &Address);
}

impl TokenManagerClientExt for TokenManagerClient<'_> {
    fn transfer(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128) {
        let _: Val = self.execute(
            token_address,
            &Symbol::new(env, "transfer"),
            &vec![
                env,
                self.address.to_val(),
                recipient.to_val(),
                amount.into_val(env),
            ],
        );
    }

    fn mint(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128) {
        let _: Val = self.execute(
            token_address,
            &Symbol::new(env, "mint_from"),
            &vec![
                env,
                self.address.to_val(),
                recipient.to_val(),
                amount.into_val(env),
            ],
        );
    }

    fn approve_service(&self, env: &Env, token_address: &Address, service_address: &Address) {
        // Set expiration to 1 year from now (approximately 6.3M ledgers at 5 seconds/ledger)
        let expiration_ledger = env.ledger().sequence() + 6_300_000;
        let _: Val = self.execute(
            token_address,
            &Symbol::new(env, "approve"),
            &vec![
                env,
                self.address.to_val(),
                service_address.to_val(),
                i128::MAX.into_val(env),
                expiration_ledger.into_val(env),
            ],
        );
    }
}
