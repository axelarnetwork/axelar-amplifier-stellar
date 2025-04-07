use stellar_axelar_std::contractstorage;
use stellar_axelar_std::{soroban_sdk, Address, Bytes, String};

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

    #[instance]
    #[value(Bytes)]
    GovernanceChainHash,

    #[instance]
    #[value(Bytes)]
    GovernanceAddressHash,

    #[persistent]
    #[value(u64)]
    ProposalTimeLock { proposal_hash: Bytes },

    #[persistent]
    #[value(bool)]
    OperatorApproval { proposal_hash: Bytes },
}
