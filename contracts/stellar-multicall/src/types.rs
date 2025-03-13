use soroban_sdk::{contracttype, Address, Symbol, Val, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub contract: Address,
    pub function: Symbol,
    pub args: Vec<Val>,
}
