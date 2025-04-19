use stellar_axelar_std::interfaces::{OperatableInterface, OwnableInterface};
use stellar_axelar_std::{contractclient, soroban_sdk, Address, Bytes, Env, String, Symbol};

use crate::error::ContractError;

#[contractclient(name = "AxelarGovernanceClient")]
pub trait AxelarGovernanceInterface: OwnableInterface + OperatableInterface {
    /// Initializes the contract.
    ///
    /// # Arguments
    /// * `gateway` - The address of the Axelar gateway contract
    /// * `operator` - The operator address
    /// * `governance_chain` - The name of the governance chain
    /// * `governance_address` - The address of the governance contract
    /// * `minimum_time_delay` - The minimum time delay for timelock operations
    ///
    /// # Errors
    /// - [`ContractError::InvalidAddress`]: If the governance chain or governance address is invalid
    fn __constructor(
        env: Env,
        gateway: Address,
        owner: Address,
        operator: Address,
        governance_chain: String,
        governance_address: String,
        minimum_time_delay: u64,
    ) -> Result<(), ContractError>;

    /// Returns whether an operator proposal has been approved
    ///
    /// # Returns
    /// - `bool`: True if the proposal has been approved, False otherwise
    ///
    /// # Arguments
    /// * `target` - The address of the contract targeted by the proposal
    /// * `call_data` - The call data to be sent to the target contract
    /// * `function` - The function to be called on the target contract
    /// * `native_value` - The amount of native tokens to be sent to the target contract
    fn is_operator_proposal_approved(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> bool;

    /// Executes an operator proposal.
    ///
    /// # Arguments
    /// * `target` - The target address the proposal will call
    /// * `call_data` - The data that encodes the function and arguments to call on the target contract
    /// * `function` - The function to be called on the target contract
    /// * `native_value` - The value of native token to be sent to the target contract
    ///
    /// # Errors
    /// - [`ContractError::OperatorProposalNotApproved`]: If the proposal has not been approved
    /// - Errors propagated by calling the target contract
    fn execute_operator_proposal(
        env: &Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        token_address: Address,
    ) -> Result<(), ContractError>;

    /// Transfers the operator address to a new address.
    ///
    /// # Arguments
    /// * `new_operator` - The new operator address
    ///
    /// # Authorization
    /// - [`OwnableInterface::owner`] must authorize.
    /// - [`OperatableInterface::operator`] must authorize.
    fn transfer_operatorship_wrapper(env: &Env, new_operator: Address);

    /// Returns the ETA of a proposal
    ///
    /// # Returns
    /// - `u64`: The ETA of the proposal
    ///
    /// # Arguments
    /// * `target` - The address of the contract targeted by the proposal
    /// * `call_data` - The call data to be sent to the target contract
    /// * `function` - The function to be called on the target contract
    /// * `native_value` - The amount of native tokens to be sent to the target contract
    fn proposal_eta(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
    ) -> u64;

    /// Executes a proposal
    ///
    /// The proposal is executed by calling the target contract with calldata. Native value is
    /// transferred with the call to the target contract.
    ///
    /// # Arguments
    /// * `target` - The target address of the contract to call
    /// * `call_data` - The data containing the function and arguments for the contract to call
    /// * `function` - The function to be called on the target contract
    /// * `native_value` - The amount of native token to send to the target contract
    ///
    /// # Errors
    /// - Errors propagated by calling the target contract
    fn execute_proposal(
        env: Env,
        target: Address,
        call_data: Bytes,
        function: Symbol,
        native_value: i128,
        token_address: Address,
    ) -> Result<(), ContractError>;

    /// Executes a command
    ///
    /// # Arguments
    /// * `source_chain` - The source chain of the command
    /// * `source_address` - The source address of the command
    /// * `payload` - The payload of the command
    fn execute(
        env: &Env,
        source_chain: String,
        source_address: String,
        payload: Bytes,
    ) -> Result<(), ContractError>;
}
