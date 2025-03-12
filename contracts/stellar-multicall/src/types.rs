use soroban_sdk::{contracttype, Address, Symbol, Val, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MulticallData {
    pub contract_address: Address,
    pub function: Symbol,
    pub args: Vec<Val>,
}
