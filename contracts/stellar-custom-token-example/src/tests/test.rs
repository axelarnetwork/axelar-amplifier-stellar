use stellar_axelar_std::soroban_sdk::testutils::Address as _;
use stellar_axelar_std::soroban_sdk::{Address, Env, String};

use crate::CustomToken;

fn setup_token(decimals: u32, name: &str, symbol: &str) -> (Env, Address, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id = env.register(
        CustomToken,
        (
            admin.clone(),
            decimals,
            String::from_str(&env, name),
            String::from_str(&env, symbol),
        ),
    );
    (env, contract_id, admin)
}

#[test]
fn test_constructor() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");

    env.as_contract(&contract_id, || {
        assert_eq!(CustomToken::decimals(env.clone()), 6u32);
        assert_eq!(
            CustomToken::name(env.clone()),
            String::from_str(&env, "Test Token")
        );
        assert_eq!(
            CustomToken::symbol(env.clone()),
            String::from_str(&env, "TEST")
        );
    });
}

#[test]
fn test_mint_and_balance() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");
    let user = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::mint(env.clone(), user.clone(), 1000i128);
        assert_eq!(CustomToken::balance(env.clone(), user.clone()), 1000i128);

        CustomToken::burn(env.clone(), user.clone(), 300i128);
        assert_eq!(CustomToken::balance(env.clone(), user), 700i128);
    });
}

#[test]
fn test_mint_from() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");
    let minter = Address::generate(&env);
    let recipient = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::add_minter(env.clone(), minter.clone());
        CustomToken::mint_from(env.clone(), minter, recipient.clone(), 500i128);
        assert_eq!(CustomToken::balance(env.clone(), recipient), 500i128);
    });
}

#[test]
fn test_add_minter() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");
    let minter = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::add_minter(env.clone(), minter);
    });
}

#[test]
fn test_metadata() {
    let (env, contract_id, _admin) = setup_token(8u32, "My Token", "MTK");

    env.as_contract(&contract_id, || {
        assert_eq!(CustomToken::decimals(env.clone()), 8u32);
        assert_eq!(
            CustomToken::name(env.clone()),
            String::from_str(&env, "My Token")
        );
        assert_eq!(
            CustomToken::symbol(env.clone()),
            String::from_str(&env, "MTK")
        );
    });
}

#[test]
#[should_panic(expected = "Decimal must not be greater than 18")]
fn test_constructor_decimal_too_large() {
    let env = Env::default();
    let admin = Address::generate(&env);
    env.register(
        CustomToken,
        (
            admin,
            19u32,
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
        ),
    );
}

#[test]
#[should_panic(expected = "not a minter")]
fn test_mint_from_not_minter() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");
    let non_minter = Address::generate(&env);
    let recipient = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::mint_from(env.clone(), non_minter, recipient, 500i128);
    });
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_burn_insufficient_balance() {
    let (env, contract_id, _admin) = setup_token(6u32, "Test Token", "TEST");
    let user = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::burn(env.clone(), user, 100i128);
    });
}
