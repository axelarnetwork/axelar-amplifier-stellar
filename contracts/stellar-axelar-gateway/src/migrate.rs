use soroban_sdk::{Env, String, Vec};
use stellar_axelar_std::interfaces::CustomMigratableInterface;

use crate::contract::AxelarGateway;
use crate::error::ContractError;
use crate::storage;

pub mod legacy_storage {
    use soroban_sdk::{contracttype, BytesN, String};
    use stellar_axelar_std::contractstorage;

    #[contracttype]
    #[derive(Clone, Debug)]
    pub struct MessageApprovalKey {
        pub source_chain: String,
        pub message_id: String,
    }

    #[contracttype]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum MessageApprovalValue {
        NotApproved,
        Approved(BytesN<32>),
        Executed,
    }

    #[contractstorage]
    enum LegacyDataKey {
        #[persistent]
        #[value(MessageApprovalValue)]
        MessageApproval {
            message_approval_key: MessageApprovalKey,
        },
    }
}

impl CustomMigratableInterface for AxelarGateway {
    type MigrationData = Vec<(String, String)>;
    type Error = ContractError;

    fn __migrate(env: &Env, migration_data: Self::MigrationData) -> Result<(), Self::Error> {
        for (source_chain, message_id) in migration_data {
            let message_approval_key = legacy_storage::MessageApprovalKey {
                source_chain: source_chain.clone(),
                message_id: message_id.clone(),
            };

            let legacy_message_approval =
                legacy_storage::try_message_approval(env, message_approval_key)
                    .ok_or(ContractError::InvalidMessageApproval)?;

            let message_approval = match legacy_message_approval {
                legacy_storage::MessageApprovalValue::Approved(hash) => {
                    Some(storage::MessageApprovalValue::Approved(hash))
                }
                legacy_storage::MessageApprovalValue::Executed => {
                    Some(storage::MessageApprovalValue::Executed)
                }
                legacy_storage::MessageApprovalValue::NotApproved => None,
            };

            storage::set_message_approval(
                env,
                source_chain,
                message_id,
                &message_approval.ok_or(ContractError::InvalidMessageApproval)?,
            );
        }

        Ok(())
    }
}
