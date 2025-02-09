use soroban_sdk::{contracttype, Address};
use stellar_axelar_std::contractstorage;

#[contractstorage]
#[derive(Clone, Debug)]
pub enum DataKey {
    #[instance]
    #[status]
    Operator { account: Address },
}
