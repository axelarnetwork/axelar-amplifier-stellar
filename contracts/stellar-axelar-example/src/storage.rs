use stellar_axelar_std::{contractstorage, soroban_sdk, Address};

#[contractstorage]
#[derive(Clone, Debug)]
enum DataKey {
    #[instance]
    #[value(Address)]
    Gateway,

    #[instance]
    #[value(Address)]
    GasService,

    #[instance]
    #[value(Address)]
    InterchainTokenService,
}
