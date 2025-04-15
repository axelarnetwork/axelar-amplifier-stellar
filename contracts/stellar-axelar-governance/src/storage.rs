use stellar_axelar_std::{contractstorage, soroban_sdk, Address, Bytes, String};

#[contractstorage]
enum DataKey {
    #[instance]
    #[value(Address)]
    Gateway,

    #[instance]
    #[value(String)]
    GovernanceChain,

    #[instance]
    #[value(String)]
    GovernanceAddress,

    #[instance]
    #[value(u64)]
    MinimumTimeDelay,

    #[persistent]
    #[value(u64)]
    ProposalTimeLock { proposal_hash: Bytes },

    #[persistent]
    #[value(bool)]
    OperatorApproval { proposal_hash: Bytes },
}
