use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    MigrationNotAllowed = 1,
    OperatorAlreadyAdded = 2,
    NotAnOperator = 3,
    MigrationInProgress = 4,
}
