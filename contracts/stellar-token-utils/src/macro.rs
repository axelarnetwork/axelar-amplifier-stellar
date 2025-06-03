/// Macro to convert an Address to an AccountId for XDR serialization
macro_rules! address_to_account_id {
    ($issuer:expr) => {{
        let issuer_bytes = $issuer.to_raw_bytes();
        let mut issuer_key_bytes = [0u8; 32];

        // Extract the public key part (last 32 bytes)
        if issuer_bytes.len() >= 32 {
            issuer_key_bytes.copy_from_slice(&issuer_bytes[issuer_bytes.len() - 32..]);
        }

        let public_key = PublicKey::PublicKeyTypeEd25519(issuer_key_bytes.into());
        AccountId(public_key)
    }};
}

/// Macro to convert a String to a fixed-size asset code array
macro_rules! string_to_asset_code {
    ($code:expr, $size:expr) => {{
        let mut code_array = [0u8; $size];
        let code_len = $code.len() as usize;
        let mut code_buffer = [0u8; 32];
        $code.copy_into_slice(&mut code_buffer[..code_len]);
        let copy_len = code_len.min($size);
        code_array[..copy_len].copy_from_slice(&code_buffer[..copy_len]);
        code_array
    }};
}

pub(crate) use {address_to_account_id, string_to_asset_code};
