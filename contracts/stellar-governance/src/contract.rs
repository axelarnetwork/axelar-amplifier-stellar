use soroban_sdk::xdr::{FromXdr, ToXdr};
use stellar_axelar_std::events::Event;
use stellar_axelar_std::{
    assert_some, contract, contractimpl, ensure, only_operator, soroban_sdk, vec, Address, Bytes,
    Env, IntoVal, Operatable, Ownable, Pausable, String, Symbol, TryIntoVal, Val, Vec,
};

use crate::error::ContractError;
use crate::event::{
    OperatorProposalApprovedEvent, OperatorProposalCancelledEvent, OperatorProposalExecutedEvent,
    ProposalCancelledEvent, ProposalExecutedEvent, ProposalScheduledEvent,
};
use crate::interface::StellarGovernanceInterface;
use crate::storage;
use crate::timelock::TimeLock;
use crate::types::CommandType;

#[contract]
#[derive(Operatable, Ownable, Pausable)]
pub struct StellarGovernance;

impl StellarGovernance {
    fn only_governance(
        env: &Env,
        source_chain: String,
        source_address: String,
    ) -> Result<(), ContractError> {
        let governance_chain = storage::governance_chain(env);
        let governance_address = storage::governance_address(env);

        if governance_chain != source_chain || governance_address != source_address {
            return Err(ContractError::NotGovernance);
        }

        Ok(())
    }

    fn process_command(
        env: &Env,
        command_type: CommandType,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        eta: u64,
    ) -> Result<(), ContractError> {
        let proposal_hash = Self::get_proposal_hash(
            env,
            target.clone(),
            call_data.clone(),
            function,
            native_value,
        );

        match command_type {
            CommandType::ScheduleTimeLockProposal => {
                let scheduled_eta = TimeLock::schedule_time_lock(env, proposal_hash.clone(), eta)?;

                ProposalScheduledEvent {
                    target,
                    eta: scheduled_eta,
                    proposal_hash,
                    call_data,
                }
                .emit(env);
            }
            CommandType::CancelTimeLockProposal => {
                TimeLock::cancel_time_lock(env, proposal_hash.clone())?;

                ProposalCancelledEvent {
                    target,
                    proposal_hash,
                    call_data,
                }
                .emit(env);
            }
            CommandType::ApproveOperatorProposal => {
                storage::set_operator_approval(env, proposal_hash.clone(), &true);

                OperatorProposalApprovedEvent {
                    target,
                    proposal_hash,
                    call_data,
                }
                .emit(env);
            }
            CommandType::CancelOperatorApproval => {
                storage::set_operator_approval(env, proposal_hash.clone(), &false);

                OperatorProposalCancelledEvent {
                    target,
                    proposal_hash,
                    call_data,
                }
                .emit(env);
            }
        }

        Ok(())
    }

    pub fn get_proposal_hash(
        env: &Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> Bytes {
        let data = vec![
            env,
            target.to_val(),
            call_data.to_val(),
            function.to_val(),
            native_value.into_val(env),
        ];

        Vec::to_xdr(data, env)
    }

    fn call_target(
        env: &Env,
        target: &Address,
        function: &Symbol,
        call_data: &Bytes,
        native_value: Option<i128>,
    ) -> Result<Val, ContractError> {
        if let Some(value) = native_value {
            if value > 0 {
                let token = soroban_sdk::token::Client::new(env, &env.current_contract_address());
                let balance: i128 = token.balance(&env.current_contract_address());

                if balance < value {
                    return Err(ContractError::InsufficientBalance);
                }

                token.transfer(&env.current_contract_address(), target, &value);
            }
        }
        let args = if !call_data.is_empty() {
            vec![env, call_data.to_val()]
        } else {
            Vec::new(env)
        };

        let result = env.invoke_contract::<Val>(target, function, args);
        Ok(result)
    }
}

#[contractimpl]
impl StellarGovernanceInterface for StellarGovernance {
    fn __constructor(
        env: Env,
        gateway: Address,
        owner: Address,
        operator: Address,
        governance_chain: String,
        governance_address: String,
        minimum_time_delay: u64,
    ) -> Result<(), ContractError> {
        if governance_chain.is_empty() || governance_address.is_empty() {
            return Err(ContractError::InvalidAddress);
        }

        stellar_axelar_std::interfaces::set_owner(&env, &owner);
        stellar_axelar_std::interfaces::set_operator(&env, &operator);

        storage::set_gateway(&env, &gateway);
        storage::set_governance_chain(&env, &governance_chain);
        storage::set_governance_address(&env, &governance_address);
        storage::set_minimum_time_delay(&env, &minimum_time_delay);

        Ok(())
    }

    fn is_operator_proposal_approved(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> bool {
        let proposal_hash =
            Self::get_proposal_hash(&env, target, call_data, function, native_value);
        storage::try_operator_approval(&env, proposal_hash).unwrap_or(false)
    }

    fn execute_operator_proposal(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> Result<(), ContractError> {
        Self::operator(&env).require_auth();

        let proposal_hash = Self::get_proposal_hash(
            &env,
            target.clone(),
            call_data.clone(),
            function.clone(),
            native_value,
        );

        ensure!(
            storage::try_operator_approval(&env, proposal_hash.clone()).unwrap_or(false),
            ContractError::OperatorProposalNotApproved
        );

        storage::set_operator_approval(&env, proposal_hash.clone(), &false);

        Self::call_target(&env, &target, &function, &call_data, Some(native_value))?;

        OperatorProposalExecutedEvent {
            target,
            proposal_hash,
            call_data,
        }
        .emit(&env);

        Ok(())
    }

    #[only_operator]
    fn transfer_operatorship_wrapper(env: &Env, new_operator: Address) {
        stellar_axelar_std::interfaces::set_operator(env, &new_operator);
    }

    fn get_proposal_eta(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> u64 {
        let proposal_hash =
            Self::get_proposal_hash(&env, target, call_data, function, native_value);

        TimeLock::get_time_lock(&env, proposal_hash)
    }

    fn execute_proposal(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> Result<(), ContractError> {
        let proposal_hash = Self::get_proposal_hash(
            &env,
            target.clone(),
            call_data.clone(),
            function.clone(),
            native_value,
        );

        let _ = TimeLock::finalize_time_lock(&env, proposal_hash.clone());

        let _ = Self::call_target(&env, &target, &function, &call_data, Some(native_value))?;

        ProposalExecutedEvent {
            target,
            proposal_hash,
            call_data,
        }
        .emit(&env);

        Ok(())
    }

    fn execute(
        env: &Env,
        source_chain: String,
        source_address: String,
        payload: Bytes,
    ) -> Result<(), ContractError> {
        Self::only_governance(env, source_chain, source_address)?;
        let params: Vec<Val> = Vec::from_xdr(env, &payload).unwrap();

        let command_type_num: u32 = assert_some!(params.get(0))
            .try_into_val(env)
            .expect("Failed to convert to u32");

        let target: Address = assert_some!(params.get(1))
            .try_into_val(env)
            .expect("Failed to convert to Address");

        let call_data: Bytes = assert_some!(params.get(2))
            .try_into_val(env)
            .expect("Failed to convert to Bytes");

        let function: Symbol = assert_some!(params.get(3))
            .try_into_val(env)
            .expect("Failed to convert to Symbol");

        let native_value: i128 = assert_some!(params.get(4))
            .try_into_val(env)
            .expect("Failed to convert to i128");

        let eta: u64 = assert_some!(params.get(5))
            .try_into_val(env)
            .expect("Failed to convert to u64");

        let command_type = match command_type_num {
            0 => CommandType::ScheduleTimeLockProposal,
            1 => CommandType::CancelTimeLockProposal,
            2 => CommandType::ApproveOperatorProposal,
            3 => CommandType::CancelOperatorApproval,
            _ => return Err(ContractError::InvalidCommandType),
        };

        Self::process_command(
            env,
            command_type,
            target,
            call_data,
            function,
            native_value,
            eta,
        )?;

        Ok(())
    }
}
