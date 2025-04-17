use stellar_axelar_std::{contract, ensure, soroban_sdk, Bytes, Env};

use crate::error::ContractError;
use crate::storage;

#[contract]
pub struct TimeLock;

impl TimeLock {
    pub fn time_lock(env: &Env, hash: Bytes) -> u64 {
        Self::time_lock_eta(env, hash)
    }

    pub fn schedule_time_lock(env: &Env, hash: Bytes, eta: u64) -> Result<u64, ContractError> {
        ensure!(
            Self::time_lock_eta(env, hash.clone()) == 0,
            ContractError::TimeLockAlreadyScheduled
        );

        let min_delay = storage::minimum_time_delay(env);
        let current_time = env.ledger().timestamp();
        let minimum_eta = current_time + min_delay;

        let final_eta = if eta < minimum_eta { minimum_eta } else { eta };

        Self::set_time_lock_eta(env, hash, final_eta);

        Ok(final_eta)
    }

    pub fn cancel_time_lock(env: &Env, hash: Bytes) -> Result<(), ContractError> {
        ensure!(
            Self::time_lock_eta(env, hash.clone()) != 0,
            ContractError::TimeLockNotScheduled
        );

        Self::set_time_lock_eta(env, hash, 0);
        Ok(())
    }

    pub fn finalize_time_lock(env: &Env, hash: Bytes) -> Result<(), ContractError> {
        let eta = Self::time_lock(env, hash.clone());

        ensure!(eta != 0, ContractError::InvalidTimeLockHash);

        let current_time = env.ledger().timestamp();
        ensure!(current_time >= eta, ContractError::TimeLockNotReady);

        Self::set_time_lock_eta(env, hash, 0);

        Ok(())
    }

    fn time_lock_eta(env: &Env, hash: Bytes) -> u64 {
        storage::try_proposal_time_lock(env, hash).unwrap_or(0)
    }

    fn set_time_lock_eta(env: &Env, hash: Bytes, eta: u64) {
        storage::set_proposal_time_lock(env, hash, &eta);
    }
}
