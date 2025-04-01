use stellar_axelar_std::{
    contract, contractimpl, interfaces, only_owner, soroban_sdk, Address, Env, Ownable, Symbol,
    Upgradable, Val, Vec,
};

use crate::error::ContractError;
use crate::interface::TokenManagerInterface;

#[contract]
#[derive(Ownable, Upgradable)]
#[migratable]
pub struct TokenManager;

#[contractimpl]
impl TokenManager {
    pub fn __constructor(env: &Env, owner: Address) {
        interfaces::set_owner(env, &owner);
    }
}

#[contractimpl]
impl TokenManagerInterface for TokenManager {
    #[only_owner]
    fn execute(
        env: &Env,
        contract: Address,
        func: Symbol,
        args: Vec<Val>,
    ) -> Result<Val, ContractError> {
        let res: Val = env.invoke_contract(&contract, &func, args);

        Ok(res)
    }
}
