use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;
use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Address, Env, String};

#[contract]
pub struct CustomToken;

#[contractimpl]
impl CustomToken {
    pub fn __constructor(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if decimal > 18 {
            panic!("Decimal must not be greater than 18");
        }
        Self::write_administrator(&env, &admin);
        Self::write_metadata(
            &env,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        Self::check_nonnegative_amount(amount);
        let admin = Self::read_administrator(&env);
        admin.require_auth();

        Self::receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().mint(admin, to, amount);
    }

    pub fn mint_from(env: Env, minter: Address, to: Address, amount: i128) {
        Self::check_nonnegative_amount(amount);
        minter.require_auth();

        if !Self::is_minter(&env, minter.clone()) {
            panic!("not a minter");
        }

        Self::receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().mint(minter, to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        Self::check_nonnegative_amount(amount);
        from.require_auth();

        Self::spend_balance(&env, from.clone(), amount);
        TokenUtils::new(&env).events().burn(from, amount);
    }

    pub fn add_minter(env: Env, minter: Address) {
        let admin = Self::read_administrator(&env);
        admin.require_auth();

        Self::add_minter_internal(&env, minter);
    }

    pub fn decimals(env: Env) -> u32 {
        TokenUtils::new(&env).metadata().get_metadata().decimal
    }

    pub fn name(env: Env) -> String {
        TokenUtils::new(&env).metadata().get_metadata().name
    }

    pub fn symbol(env: Env) -> String {
        TokenUtils::new(&env).metadata().get_metadata().symbol
    }

    pub fn balance(e: Env, id: Address) -> i128 {
        Self::read_balance_internal(&e, id)
    }
}

impl CustomToken {
    fn check_nonnegative_amount(amount: i128) {
        if amount < 0 {
            panic!("negative amount is not allowed: {}", amount)
        }
    }

    fn write_metadata(env: &Env, metadata: TokenMetadata) {
        let util = TokenUtils::new(env);
        util.metadata().set_metadata(&metadata);
    }

    fn read_administrator(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&String::from_str(env, "admin"))
            .unwrap()
    }

    fn write_administrator(env: &Env, id: &Address) {
        env.storage()
            .instance()
            .set(&String::from_str(env, "admin"), id);
    }

    fn is_minter(env: &Env, minter: Address) -> bool {
        env.storage()
            .persistent()
            .get(&(String::from_str(env, "minter"), minter))
            .unwrap_or(false)
    }

    fn add_minter_internal(env: &Env, minter: Address) {
        env.storage()
            .persistent()
            .set(&(String::from_str(env, "minter"), minter), &true);
    }

    fn read_balance_internal(env: &Env, addr: Address) -> i128 {
        env.storage().persistent().get(&addr).unwrap_or(0)
    }

    fn write_balance(env: &Env, addr: Address, amount: i128) {
        env.storage().persistent().set(&addr, &amount);
    }

    fn receive_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Self::read_balance_internal(env, addr.clone());
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_else(|| panic!("balance overflow"));
        Self::write_balance(env, addr, new_balance);
    }

    fn spend_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Self::read_balance_internal(env, addr.clone());
        if balance < amount {
            panic!("insufficient balance");
        }
        let new_balance = balance
            .checked_sub(amount)
            .unwrap_or_else(|| panic!("balance underflow"));
        Self::write_balance(env, addr, new_balance);
    }
}
