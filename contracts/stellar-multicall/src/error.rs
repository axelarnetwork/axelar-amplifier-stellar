use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    FunctionCallFailed = 1,
}
