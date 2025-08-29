use crate::CustomToken;
use stellar_axelar_std::soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_token(env: &Env, decimals: u32, name: &str, symbol: &str) -> (Address, Address) {
    let admin = Address::generate(env);
    let contract_id = env.register(
        CustomToken,
        (
            admin.clone(),
            decimals,
            String::from_str(env, name),
            String::from_str(env, symbol),
        ),
    );
    (contract_id, admin)
}

#[test]
fn test_constructor() {
    let env = Env::default();
    let (contract_id, _admin) = setup_token(&env, 6u32, "Test Token", "TEST");

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
    let env = Env::default();
    let (contract_id, _admin) = setup_token(&env, 6u32, "Test Token", "TEST");
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
    let env = Env::default();
    let (contract_id, _admin) = setup_token(&env, 6u32, "Test Token", "TEST");
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
    let env = Env::default();
    let (contract_id, _admin) = setup_token(&env, 6u32, "Test Token", "TEST");
    let minter = Address::generate(&env);

    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        CustomToken::add_minter(env.clone(), minter);
    });
}

#[test]
fn test_metadata() {
    let env = Env::default();
    let (contract_id, _admin) = setup_token(&env, 8u32, "My Token", "MTK");

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
