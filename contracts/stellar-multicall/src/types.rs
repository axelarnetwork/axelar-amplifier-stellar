use soroban_sdk::{contracttype, Address, Symbol, Val, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionCall {
    pub contract: Address,
    /// There must be an address authorizing each function call, even if the function call itself doesn't need authorization, to prevent frontrunning of individual calls within a multicall
    pub approver: Address,
    pub function: Symbol,
    pub args: Vec<Val>,
}
