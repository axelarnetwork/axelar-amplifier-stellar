use soroban_sdk::xdr::{FromXdr, ToXdr};
use stellar_axelar_std::events::Event;
use stellar_axelar_std::{
    contract, contractimpl, ensure, only_operator, soroban_sdk, vec, when_not_paused, Address,
    Bytes, Env, IntoVal, Operatable, Ownable, Pausable, String, Symbol, TryIntoVal, Val, Vec,
};

use crate::error::ContractError;
use crate::event::{
    OperatorProposalApprovedEvent, OperatorProposalCancelledEvent, OperatorProposalExecutedEvent,
    ProposalCancelledEvent, ProposalExecutedEvent, ProposalScheduledEvent,
};
use crate::interface::AxelarGovernanceInterface;
use crate::timelock::TimeLock;
use crate::types::CommandType;
use crate::{get_params, storage};

#[contract]
#[derive(Operatable, Ownable, Pausable)]
pub struct AxelarGovernance;

impl AxelarGovernance {
    fn only_governance(
        env: &Env,
        source_chain: String,
        source_address: String,
    ) -> Result<(), ContractError> {
        let governance_chain = storage::governance_chain(env);
        let governance_address = storage::governance_address(env);

        ensure!(
            governance_chain == source_chain && governance_address == source_address,
            ContractError::NotGovernance
        );

        Ok(())
    }

    #[when_not_paused]
    fn process_command(
        env: &Env,
        command_type: CommandType,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        eta: u64,
    ) -> Result<(), ContractError> {
        let proposal_hash = Self::proposal_hash(
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
                    proposal_hash,
                    target,
                    call_data,
                    eta: scheduled_eta,
                }
                .emit(env);
            }
            CommandType::CancelTimeLockProposal => {
                TimeLock::cancel_time_lock(env, proposal_hash.clone())?;

                ProposalCancelledEvent {
                    proposal_hash,
                    target,
                    call_data,
                }
                .emit(env);
            }
            CommandType::ApproveOperatorProposal => {
                storage::set_operator_approval(env, proposal_hash.clone(), &true);

                OperatorProposalApprovedEvent {
                    proposal_hash,
                    target,
                    call_data,
                }
                .emit(env);
            }
            CommandType::CancelOperatorApproval => {
                storage::set_operator_approval(env, proposal_hash.clone(), &false);

                OperatorProposalCancelledEvent {
                    proposal_hash,
                    target,
                    call_data,
                }
                .emit(env);
            }
        }

        Ok(())
    }

    pub fn proposal_hash(
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
        token_address: &Address,
    ) -> Result<Val, ContractError> {
        if let Some(value) = native_value {
            if value > 0 {
                let token_client = soroban_sdk::token::Client::new(env, token_address);
                let balance: i128 = token_client.balance(&env.current_contract_address());

                ensure!(balance >= value, ContractError::InsufficientBalance);

                token_client.transfer(&env.current_contract_address(), target, &value);
            }
        }
        let args = if call_data.is_empty() {
            Vec::new(env)
        } else {
            vec![env, call_data.to_val()]
        };

        Ok(env.invoke_contract::<Val>(target, function, args))
    }
}

#[contractimpl]
impl AxelarGovernanceInterface for AxelarGovernance {
    fn __constructor(
        env: Env,
        gateway: Address,
        owner: Address,
        operator: Address,
        governance_chain: String,
        governance_address: String,
        minimum_time_delay: u64,
    ) -> Result<(), ContractError> {
        ensure!(
            !governance_chain.is_empty() && !governance_address.is_empty(),
            ContractError::InvalidAddress
        );

        storage::set_gateway(&env, &gateway);
        stellar_axelar_std::interfaces::set_owner(&env, &owner);
        stellar_axelar_std::interfaces::set_operator(&env, &operator);

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
        let proposal_hash = Self::proposal_hash(&env, target, call_data, function, native_value);
        storage::try_operator_approval(&env, proposal_hash).unwrap_or(false)
    }

    #[when_not_paused]
    #[only_operator]
    fn execute_operator_proposal(
        env: &Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        token_address: Address,
    ) -> Result<(), ContractError> {
        let proposal_hash = Self::proposal_hash(
            env,
            target.clone(),
            call_data.clone(),
            function.clone(),
            native_value,
        );

        ensure!(
            storage::try_operator_approval(env, proposal_hash.clone()).unwrap_or(false),
            ContractError::OperatorProposalNotApproved
        );

        storage::set_operator_approval(env, proposal_hash.clone(), &false);

        Self::call_target(
            env,
            &target,
            &function,
            &call_data,
            Some(native_value),
            &token_address,
        )?;

        OperatorProposalExecutedEvent {
            target,
            proposal_hash,
            call_data,
        }
        .emit(env);

        Ok(())
    }

    #[only_operator]
    fn transfer_operatorship_wrapper(env: &Env, new_operator: Address) {
        stellar_axelar_std::interfaces::set_operator(env, &new_operator);
    }

    fn proposal_eta(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> u64 {
        let proposal_hash = Self::proposal_hash(&env, target, call_data, function, native_value);

        TimeLock::time_lock(&env, proposal_hash)
    }

    fn execute_proposal(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        token_address: Address,
    ) -> Result<(), ContractError> {
        let proposal_hash = Self::proposal_hash(
            &env,
            target.clone(),
            call_data.clone(),
            function.clone(),
            native_value,
        );

        TimeLock::finalize_time_lock(&env, proposal_hash.clone())?;

        Self::call_target(
            &env,
            &target,
            &function,
            &call_data,
            Some(native_value),
            &token_address,
        )?;

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

        let (command_type_num, target, call_data, function, native_value, eta) =
            get_params!(env, params, u32, Address, Bytes, Symbol, i128, u64);

        let command_type = match command_type_num {
            1 => CommandType::ScheduleTimeLockProposal,
            2 => CommandType::CancelTimeLockProposal,
            3 => CommandType::ApproveOperatorProposal,
            4 => CommandType::CancelOperatorApproval,
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
        )
    }
}
