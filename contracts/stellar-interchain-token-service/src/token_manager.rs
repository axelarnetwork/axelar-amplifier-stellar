use stellar_axelar_std::{vec, Address, Env, IntoVal, Symbol, Val};
use stellar_token_manager::TokenManagerClient;

pub trait TokenManagerClientExt {
    /// Transfer the admin role of a token to a new address.
    fn set_admin(&self, env: &Env, token_address: &Address, new_admin: &Address);

    /// Transfer `amount` of tokens from the token manager to `recipient`.
    fn transfer(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128);

    /// Mint `amount` of tokens to `recipient`.
    fn mint(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128);

    /// Mint `amount` of tokens from token manager to `recipient`.
    fn mint_from(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128);
}

impl TokenManagerClientExt for TokenManagerClient<'_> {
    fn set_admin(&self, env: &Env, token_address: &Address, new_admin: &Address) {
        let _: Val = self.execute(
            token_address,
            &Symbol::new(env, "set_admin"),
            &vec![env, new_admin.to_val()],
        );
    }

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

    fn mint_from(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128) {
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

    fn mint(&self, env: &Env, token_address: &Address, recipient: &Address, amount: i128) {
        let _: Val = self.execute(
            token_address,
            &Symbol::new(env, "mint"),
            &vec![env, recipient.to_val(), amount.into_val(env)],
        );
    }
}
