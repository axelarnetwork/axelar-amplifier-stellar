#![cfg(test)]
extern crate alloc;
extern crate std;

use std::str::FromStr;
use std::string::{String, ToString};

use stellar_axelar_std::xdr::{
    AccountId, AlphaNum12, AlphaNum4, Asset as XdrAsset, AssetCode12, AssetCode4, Limits,
    ScAddress, WriteXdr,
};
use stellar_axelar_std::{Address, Bytes, Env, String as SorobanString};

use crate::{TokenUtils, TokenUtilsClient};

macro_rules! address_to_string {
    ($addresses:expr) => {
        $addresses
            .iter()
            .map(|addr| addr.to_string().to_string())
            .collect::<std::vec::Vec<std::string::String>>()
    };
}

pub(crate) use address_to_string;

pub fn address_to_str(address: &Address) -> String {
    address.to_string().to_string()
}

pub fn str_to_address(env: &Env, address: &str) -> Address {
    Address::from_string(&SorobanString::from_str(env, address))
}

pub fn address_to_account_id(address: &Address) -> AccountId {
    let address_str = address_to_str(address);

    let ScAddress::Account(account_id) = ScAddress::from_str(&address_str).unwrap() else {
        panic!("not an account");
    };

    account_id
}

pub fn string_to_asset_code<const N: usize>(code: &str) -> [u8; N] {
    std::array::from_fn(|i| code.bytes().nth(i).unwrap_or(0))
}

pub fn create_asset_xdr(env: &Env, code: &str, issuer: &Address) -> Bytes {
    let issuer_account_id = address_to_account_id(issuer);

    let asset = if code.len() <= 4 {
        XdrAsset::CreditAlphanum4(AlphaNum4 {
            asset_code: AssetCode4(string_to_asset_code::<4>(code)),
            issuer: issuer_account_id,
        })
    } else {
        XdrAsset::CreditAlphanum12(AlphaNum12 {
            asset_code: AssetCode12(string_to_asset_code::<12>(code)),
            issuer: issuer_account_id,
        })
    };

    let asset_xdr = asset.to_xdr(Limits::none()).unwrap();
    Bytes::from_slice(env, &asset_xdr)
}

pub fn assert_valid_contract_address(address: &Address) {
    let address_str = address_to_str(address);
    assert!(!address_str.is_empty());

    let ScAddress::Contract(_) = ScAddress::from_str(&address_str).unwrap() else {
        panic!("not a contract");
    };
}

pub fn setup<'a>() -> (Env, TokenUtilsClient<'a>) {
    let env = Env::default();
    let contract_id = env.register(TokenUtils, ());
    let client = TokenUtilsClient::new(&env, &contract_id);
    (env, client)
}
