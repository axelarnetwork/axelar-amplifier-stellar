use std::vec::Vec;

use stellar_axelar_std::{Address, Env, String};

use crate::StellarTokenUtils;

fn create_test_issuer(env: &Env) -> Address {
    // Use proper Stellar address starting with G
    Address::from_string(&String::from_str(
        env,
        "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5",
    ))
}

fn create_test_issuer_2(env: &Env) -> Address {
    // Use proper Stellar address starting with G
    Address::from_string(&String::from_str(
        env,
        "GBZXN7PIRZGNMHGA7MUUUF4GWPY5AYPV6LY4UV2GL6VJGIQRXFDNMADI",
    ))
}

fn create_test_issuer_3(env: &Env) -> Address {
    // Another test issuer
    Address::from_string(&String::from_str(
        env,
        "GC57ZJLYGUOMGDGPD5XFDOTQHQER3QL6B72SPTR3XTEKRY3HJ75NCNBL",
    ))
}

fn setup_contract(env: &Env) -> crate::StellarTokenUtilsClient {
    let contract_id = env.register(StellarTokenUtils, ());
    crate::StellarTokenUtilsClient::new(env, &contract_id)
}

// Helper function to test successful cases
fn assert_success(
    env: &Env,
    client: &crate::StellarTokenUtilsClient,
    code: &str,
    issuer: &Address,
) -> Address {
    let code_string = String::from_str(env, code);
    let result = client.try_stellar_asset_contract_address(&code_string, issuer);
    assert!(result.is_ok(), "Failed for code: {}", code);
    result.unwrap().unwrap()
}

// Helper function to test error cases
fn assert_error(env: &Env, client: &crate::StellarTokenUtilsClient, code: &str, issuer: &Address) {
    let code_string = String::from_str(env, code);
    let result = client.try_stellar_asset_contract_address(&code_string, issuer);
    assert!(result.is_err(), "Expected error for code: {}", code);
}

#[test]
fn test_stellar_asset_contract_address_basic_functionality() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test various valid codes
    let valid_codes = vec!["USDC", "BTC", "ETH", "USD", "A", "ABC", "ABCDEFGHIJKL"];

    for code in valid_codes {
        assert_success(&env, &client, code, &issuer);
    }
}

#[test]
fn test_stellar_asset_contract_address_native_assets() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test native asset variants
    let native_codes = vec!["XLM", "native"];

    for code in native_codes {
        assert_success(&env, &client, code, &issuer);
    }
}

#[test]
fn test_stellar_asset_contract_address_error_cases() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test error cases
    let error_codes = vec![
        "",                                            // Empty code
        "ABCDEFGHIJKLM",                               // 13 characters
        "FOURTEENCHAR12",                              // 14 characters
        "THISISAVERYLONGASSETCODETHATEXCEEDSTHELIMIT", // Extremely long
    ];

    for code in error_codes {
        assert_error(&env, &client, code, &issuer);
    }
}

#[test]
fn test_stellar_asset_contract_address_length_boundaries() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test all valid lengths (1-12 characters)
    let valid_lengths = vec![
        "A",            // 1 char
        "AB",           // 2 chars
        "ABC",          // 3 chars
        "ABCD",         // 4 chars (AlphaNum4 boundary)
        "ABCDE",        // 5 chars (AlphaNum12 starts)
        "ABCDEF",       // 6 chars
        "ABCDEFG",      // 7 chars
        "ABCDEFGH",     // 8 chars
        "ABCDEFGHI",    // 9 chars
        "ABCDEFGHIJ",   // 10 chars
        "ABCDEFGHIJK",  // 11 chars
        "ABCDEFGHIJKL", // 12 chars (maximum)
    ];

    let mut addresses = Vec::new();
    for code in &valid_lengths {
        addresses.push(assert_success(&env, &client, code, &issuer));
    }

    // All addresses should be different
    for i in 0..addresses.len() {
        for j in i + 1..addresses.len() {
            assert_ne!(
                addresses[i], addresses[j],
                "Addresses should be different for codes '{}' and '{}'",
                valid_lengths[i], valid_lengths[j]
            );
        }
    }

    // Test invalid lengths
    assert_error(&env, &client, "ABCDEFGHIJKLM", &issuer); // 13 chars
    assert_error(&env, &client, "ABCDEFGHIJKLMN", &issuer); // 14 chars
}

#[test]
fn test_stellar_asset_contract_address_deterministic_behavior() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);
    let code = "CONSISTENT";

    // Multiple calls should return identical addresses
    let addresses: Vec<Address> = (0..5)
        .map(|_| assert_success(&env, &client, code, &issuer))
        .collect();

    // All addresses should be identical
    for i in 1..addresses.len() {
        assert_eq!(addresses[0], addresses[i], "Deterministic behavior failed");
    }
}

#[test]
fn test_stellar_asset_contract_address_different_inputs() {
    let env = Env::default();
    let client = setup_contract(&env);

    let issuer1 = create_test_issuer(&env);
    let issuer2 = create_test_issuer_2(&env);
    let issuer3 = create_test_issuer_3(&env);

    // Test different codes with same issuer
    let codes = vec!["USD", "EUR", "GBP"];
    let mut code_addresses = Vec::new();
    for code in &codes {
        code_addresses.push(assert_success(&env, &client, code, &issuer1));
    }

    // Test same code with different issuers
    let same_code = "STABLE";
    let issuer_addresses = vec![
        assert_success(&env, &client, same_code, &issuer1),
        assert_success(&env, &client, same_code, &issuer2),
        assert_success(&env, &client, same_code, &issuer3),
    ];

    // All addresses should be different
    let all_addresses = [code_addresses, issuer_addresses].concat();
    for i in 0..all_addresses.len() {
        for j in i + 1..all_addresses.len() {
            assert_ne!(
                all_addresses[i], all_addresses[j],
                "All addresses should be unique"
            );
        }
    }
}

#[test]
fn test_stellar_asset_contract_address_alphanumeric_variants() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test different character types
    let test_cases = vec![
        ("USDC", "Standard alphabetic"),
        ("123", "Numeric only"),
        ("USD2024", "Mixed alphanumeric"),
        ("USDC", "Uppercase"),
        ("usdc", "Lowercase"),
    ];

    let mut addresses = Vec::new();
    for (code, description) in &test_cases {
        let addr = assert_success(&env, &client, code, &issuer);
        addresses.push((addr, description));
    }

    // USDC and usdc should produce different addresses (case sensitive)
    assert_ne!(
        addresses[3].0, addresses[4].0,
        "Case sensitivity should produce different addresses"
    );
}

#[test]
fn test_stellar_asset_contract_address_alphanum_type_boundary() {
    let env = Env::default();
    let client = setup_contract(&env);
    let issuer = create_test_issuer(&env);

    // Test the boundary between AlphaNum4 and AlphaNum12
    let alphanum4_code = assert_success(&env, &client, "ABCD", &issuer); // 4 chars
    let alphanum12_code = assert_success(&env, &client, "ABCDE", &issuer); // 5 chars

    // Should produce different addresses due to different asset types
    assert_ne!(
        alphanum4_code, alphanum12_code,
        "AlphaNum4 and AlphaNum12 should produce different addresses"
    );
}
