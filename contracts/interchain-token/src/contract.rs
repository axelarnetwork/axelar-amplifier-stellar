use axelar_soroban_std::token::validate_token_metadata;
use axelar_soroban_std::ttl::{extend_instance_ttl, extend_persistent_ttl};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

use crate::error::ContractError;
use crate::event;
use crate::storage_types::DataKey;

use crate::interface::InterchainTokenInterface;
use crate::storage_types::{AllowanceDataKey, AllowanceValue};
use axelar_soroban_std::interfaces::OwnableInterface;
use axelar_soroban_std::{ensure, interfaces, Upgradable};
use soroban_sdk::token::{StellarAssetInterface, TokenInterface};

use soroban_sdk::{
    assert_with_error, contract, contractimpl, panic_with_error, token, Address, BytesN, Env,
    String,
};
use soroban_token_sdk::event::Events as TokenEvents;

#[contract]
#[derive(Upgradable)]
pub struct InterchainToken;

#[contractimpl]
impl InterchainToken {
    pub fn __constructor(
        env: Env,
        owner: Address,
        minter: Option<Address>,
        token_id: BytesN<32>,
        token_metadata: TokenMetadata,
    ) {
        interfaces::set_owner(&env, &owner);

        if let Err(err) = validate_token_metadata(token_metadata.clone()) {
            panic_with_error!(env, err);
        }

        Self::write_metadata(&env, token_metadata);

        env.storage().instance().set(&DataKey::TokenId, &token_id);

        env.storage().instance().set(&DataKey::Minter(owner), &());

        if let Some(minter) = minter {
            env.storage().instance().set(&DataKey::Minter(minter), &());
        }
    }
}

#[contractimpl]
impl StellarAssetInterface for InterchainToken {
    fn set_admin(env: Env, admin: Address) {
        Self::transfer_ownership(&env, admin);
    }

    fn admin(env: Env) -> Address {
        Self::owner(&env)
    }

    fn set_authorized(_env: Env, _id: Address, _authorize: bool) {
        todo!()
    }

    fn authorized(_env: Env, _id: Address) -> bool {
        todo!()
    }

    fn mint(env: Env, to: Address, amount: i128) {
        if let Err(err) = Self::mint_from(&env, Self::owner(&env), to, amount) {
            panic_with_error!(env, err);
        }
    }

    fn clawback(_env: Env, _from: Address, _amount: i128) {
        todo!()
    }
}

#[contractimpl]
impl InterchainTokenInterface for InterchainToken {
    fn token_id(env: &Env) -> BytesN<32> {
        env.storage()
            .instance()
            .get(&DataKey::TokenId)
            .expect("token id not found")
    }

    fn is_minter(env: &Env, minter: Address) -> bool {
        env.storage().instance().has(&DataKey::Minter(minter))
    }

    fn mint_from(
        env: &Env,
        minter: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        minter.require_auth();

        ensure!(
            Self::is_minter(env, minter.clone()),
            ContractError::NotMinter
        );

        Self::validate_amount(env, amount);

        Self::receive_balance(env, to.clone(), amount);

        extend_instance_ttl(env);

        TokenUtils::new(env).events().mint(minter, to, amount);

        Ok(())
    }

    fn add_minter(env: &Env, minter: Address) {
        Self::owner(env).require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Minter(minter.clone()), &());

        extend_instance_ttl(env);

        event::add_minter(env, minter);
    }

    fn remove_minter(env: &Env, minter: Address) {
        Self::owner(env).require_auth();

        env.storage()
            .instance()
            .remove(&DataKey::Minter(minter.clone()));

        extend_instance_ttl(env);

        event::remove_minter(env, minter);
    }
}

#[contractimpl]
impl token::Interface for InterchainToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        extend_instance_ttl(&env);
        Self::read_allowance(&env, from, spender).amount
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        Self::validate_amount(&env, amount);

        Self::write_allowance(
            &env,
            from.clone(),
            spender.clone(),
            amount,
            expiration_ledger,
        );

        extend_instance_ttl(&env);

        TokenUtils::new(&env)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        extend_instance_ttl(&env);
        Self::read_balance(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        Self::validate_amount(&env, amount);
        Self::spend_balance(&env, from.clone(), amount);
        Self::receive_balance(&env, to.clone(), amount);

        extend_instance_ttl(&env);

        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        Self::validate_amount(&env, amount);
        Self::spend_allowance(&env, from.clone(), spender, amount);
        Self::spend_balance(&env, from.clone(), amount);
        Self::receive_balance(&env, to.clone(), amount);

        extend_instance_ttl(&env);

        TokenUtils::new(&env).events().transfer(from, to, amount)
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        Self::validate_amount(&env, amount);
        Self::spend_balance(&env, from.clone(), amount);

        extend_instance_ttl(&env);

        TokenUtils::new(&env).events().burn(from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        Self::validate_amount(&env, amount);
        Self::spend_allowance(&env, from.clone(), spender, amount);
        Self::spend_balance(&env, from.clone(), amount);

        extend_instance_ttl(&env);

        TokenUtils::new(&env).events().burn(from, amount)
    }

    fn decimals(env: Env) -> u32 {
        TokenUtils::new(&env).metadata().get_metadata().decimal
    }

    fn name(env: Env) -> String {
        TokenUtils::new(&env).metadata().get_metadata().name
    }

    fn symbol(env: Env) -> String {
        TokenUtils::new(&env).metadata().get_metadata().symbol
    }
}

impl InterchainToken {
    // Modify this function to add migration logic
    const fn run_migration(_env: &Env, _migration_data: ()) {}

    fn validate_amount(env: &Env, amount: i128) {
        assert_with_error!(env, amount >= 0, ContractError::InvalidAmount);
    }

    fn read_allowance(env: &Env, from: Address, spender: Address) -> AllowanceValue {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        env.storage()
            .temporary()
            .get::<_, AllowanceValue>(&key)
            .map_or(
                AllowanceValue {
                    amount: 0,
                    expiration_ledger: 0,
                },
                |allowance| {
                    if allowance.expiration_ledger < env.ledger().sequence() {
                        AllowanceValue {
                            amount: 0,
                            expiration_ledger: allowance.expiration_ledger,
                        }
                    } else {
                        allowance
                    }
                },
            )
    }

    fn write_allowance(
        env: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let allowance = AllowanceValue {
            amount,
            expiration_ledger,
        };

        assert_with_error!(
            env,
            !(amount > 0 && expiration_ledger < env.ledger().sequence()),
            ContractError::InvalidExpirationLedger
        );

        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        env.storage().temporary().set(&key, &allowance);

        if amount > 0 {
            let live_for = expiration_ledger
                .checked_sub(env.ledger().sequence())
                .unwrap();

            env.storage()
                .temporary()
                .extend_ttl(&key, live_for, live_for)
        }
    }

    fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
        let allowance = Self::read_allowance(env, from.clone(), spender.clone());

        assert_with_error!(
            env,
            allowance.amount >= amount,
            ContractError::InsufficientAllowance
        );

        if amount > 0 {
            Self::write_allowance(
                env,
                from,
                spender,
                allowance
                    .amount
                    .checked_sub(amount)
                    .expect("insufficient allowance"),
                allowance.expiration_ledger,
            );
        }
    }

    fn read_balance(env: &Env, addr: Address) -> i128 {
        let key = DataKey::Balance(addr);
        env.storage()
            .persistent()
            .get::<_, i128>(&key)
            .inspect(|_| {
                // Extend the TTL of the balance entry when the balance is successfully retrieved.
                extend_persistent_ttl(env, &key);
            })
            .unwrap_or_default()
    }

    fn receive_balance(env: &Env, addr: Address, amount: i128) {
        let key = DataKey::Balance(addr);

        env.storage()
            .persistent()
            .update(&key, |balance: Option<i128>| {
                balance.unwrap_or_default() + amount
            });
    }

    fn spend_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Self::read_balance(env, addr.clone());

        assert_with_error!(env, balance >= amount, ContractError::InsufficientBalance);

        Self::write_balance(env, addr, balance - amount);
    }

    fn write_metadata(env: &Env, metadata: TokenMetadata) {
        TokenUtils::new(env).metadata().set_metadata(&metadata);
    }

    fn write_balance(env: &Env, addr: Address, amount: i128) {
        let key = DataKey::Balance(addr);

        env.storage().persistent().set(&key, &amount);

        extend_persistent_ttl(env, &key);
    }
}

#[contractimpl]
impl OwnableInterface for InterchainToken {
    fn owner(env: &Env) -> Address {
        interfaces::owner(env)
    }

    fn transfer_ownership(env: &Env, new_owner: Address) {
        interfaces::transfer_ownership::<Self>(env, new_owner.clone());
        // adhere to reference implementation for tokens and emit predefined soroban event
        TokenEvents::new(env).set_admin(Self::owner(env), new_owner);
    }
}
