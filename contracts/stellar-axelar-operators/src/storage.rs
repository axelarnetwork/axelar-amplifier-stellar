use stellar_axelar_std::{contractstorage, soroban_sdk, Address};

#[contractstorage]
enum DataKey {
    #[instance]
    #[status]
    Operator { account: Address },
}
