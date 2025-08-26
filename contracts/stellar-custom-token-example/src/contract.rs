use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;
use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Address, Env, String};

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let util = TokenUtils::new(e);
    util.metadata().set_metadata(&metadata);
}

pub fn read_administrator(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&String::from_str(e, "admin"))
        .unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    e.storage()
        .instance()
        .set(&String::from_str(e, "admin"), id);
}

pub fn is_minter(e: &Env, minter: Address) -> bool {
    e.storage()
        .persistent()
        .get(&(String::from_str(e, "minter"), minter))
        .unwrap_or(false)
}

pub fn add_minter(e: &Env, minter: Address) {
    e.storage()
        .persistent()
        .set(&(String::from_str(e, "minter"), minter), &true);
}

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    e.storage().persistent().get(&addr).unwrap_or(0)
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    e.storage().persistent().set(&addr, &amount);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    let new_balance = balance
        .checked_add(amount)
        .unwrap_or_else(|| panic!("balance overflow"));
    write_balance(e, addr, new_balance);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    let new_balance = balance
        .checked_sub(amount)
        .unwrap_or_else(|| panic!("balance underflow"));
    write_balance(e, addr, new_balance);
}

#[contract]
pub struct CustomToken;

#[contractimpl]
impl CustomToken {
    pub fn __constructor(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if decimal > 18 {
            panic!("Decimal must not be greater than 18");
        }
        write_administrator(&e, &admin);
        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    pub fn mint_from(e: Env, minter: Address, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        minter.require_auth();

        if !is_minter(&e, minter.clone()) {
            panic!("not a minter");
        }

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(minter, to, amount);
    }

    pub fn burn(e: Env, from: Address, amount: i128) {
        check_nonnegative_amount(amount);
        from.require_auth();

        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    pub fn add_minter(e: Env, minter: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        add_minter(&e, minter);
    }

    pub fn decimals(e: Env) -> u32 {
        TokenUtils::new(&e).metadata().get_metadata().decimal
    }

    pub fn name(e: Env) -> String {
        TokenUtils::new(&e).metadata().get_metadata().name
    }

    pub fn symbol(e: Env) -> String {
        TokenUtils::new(&e).metadata().get_metadata().symbol
    }
}
