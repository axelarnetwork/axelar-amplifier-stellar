use stellar_axelar_std::events::Event;
use stellar_axelar_std::{ensure, BytesN, Env};

use crate::error::ContractError;
use crate::event::FlowLimitSetEvent;
use crate::storage;

const EPOCH_TIME: u64 = 6 * 60 * 60; // 6 hours in seconds = 21600

pub enum FlowDirection {
    /// An interchain transfer coming in to this chain from another chain
    In,
    /// An interchain transfer going out from this chain to another chain
    Out,
}

impl FlowDirection {
    fn flow(&self, env: &Env, token_id: BytesN<32>) -> i128 {
        match self {
            Self::In => flow_in_amount(env, token_id),
            Self::Out => flow_out_amount(env, token_id),
        }
    }

    fn reverse_flow(&self, env: &Env, token_id: BytesN<32>) -> i128 {
        match self {
            Self::In => flow_out_amount(env, token_id),
            Self::Out => flow_in_amount(env, token_id),
        }
    }

    fn update_flow(&self, env: &Env, token_id: BytesN<32>, new_flow: i128) {
        match self {
            Self::In => storage::set_flow_in(env, token_id, current_epoch(env), &new_flow),
            Self::Out => storage::set_flow_out(env, token_id, current_epoch(env), &new_flow),
        };
    }

    /// Adds flow amount in the specified direction (in/out) for a token.
    /// Flow amounts are stored in temporary storage since they only need to persist for
    /// the 6-hour epoch duration.
    ///
    /// Checks that:
    /// - Flow amount doesn't exceed the flow limit
    /// - The flow in each direction doesn't exceed `i128::MAX`
    /// - Net flow (outgoing minus incoming flow) doesn't exceed the flow limit, i.e |flow - reverse_flow| <= flow_limit
    pub fn add_flow(
        &self,
        env: &Env,
        token_id: BytesN<32>,
        flow_amount: i128,
    ) -> Result<(), ContractError> {
        let Some(flow_limit) = flow_limit(env, token_id.clone()) else {
            return Ok(());
        };

        ensure!(
            flow_amount <= flow_limit,
            ContractError::FlowAmountExceededLimit
        );

        let flow = self.flow(env, token_id.clone());
        let reverse_flow = self.reverse_flow(env, token_id.clone());

        let new_flow = flow
            .checked_add(flow_amount)
            .ok_or(ContractError::FlowAmountOverflow)?;

        // Since `new_flow` and `reverse_flow` are both positive, there won't be any overflow
        let net_flow = new_flow
            .checked_sub(reverse_flow)
            .expect("unexpected overflow");

        // `net_flow.abs()` can't overflow since `net_flow` can never be `i128::MIN` through subtracting two non-negative values
        ensure!(
            net_flow.abs() <= flow_limit,
            ContractError::FlowLimitExceeded
        );

        self.update_flow(env, token_id, new_flow);

        Ok(())
    }
}

pub fn current_epoch(env: &Env) -> u64 {
    env.ledger().timestamp() / EPOCH_TIME
}

pub fn flow_limit(env: &Env, token_id: BytesN<32>) -> Option<i128> {
    storage::try_flow_limit(env, token_id)
}

pub fn set_flow_limit(
    env: &Env,
    token_id: BytesN<32>,
    flow_limit: Option<i128>,
) -> Result<(), ContractError> {
    if let Some(flow_limit) = flow_limit {
        ensure!(flow_limit >= 0, ContractError::InvalidFlowLimit);

        storage::set_flow_limit(env, token_id.clone(), &flow_limit);
    } else {
        storage::remove_flow_limit(env, token_id.clone());
    }

    FlowLimitSetEvent {
        token_id,
        flow_limit,
    }
    .emit(env);

    Ok(())
}

pub fn flow_out_amount(env: &Env, token_id: BytesN<32>) -> i128 {
    storage::try_flow_out(env, token_id, current_epoch(env)).unwrap_or(0)
}

pub fn flow_in_amount(env: &Env, token_id: BytesN<32>) -> i128 {
    storage::try_flow_in(env, token_id, current_epoch(env)).unwrap_or(0)
}
