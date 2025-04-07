use stellar_axelar_std::{contract, contractimpl, soroban_sdk, Bytes, Env};

use crate::error::ContractError;
use crate::storage;
#[contract]
pub struct TimeLock;

#[contractimpl]
impl TimeLock {
    pub fn __constructor(env: &Env, minimum_time_delay: u64) {
        storage::set_minimum_time_delay(env, &minimum_time_delay);
    }

    pub fn get_time_lock(env: &Env, hash: Bytes) -> u64 {
        Self::get_time_lock_eta(env, hash)
    }

    pub fn schedule_time_lock(env: &Env, hash: Bytes, eta: u64) -> Result<u64, ContractError> {
        if hash.len() == 0 {
            return Err(ContractError::InvalidTimeLockHash);
        }

        if Self::get_time_lock_eta(env, hash.clone()) != 0 {
            return Err(ContractError::TimeLockAlreadyScheduled);
        }

        let min_delay = storage::minimum_time_delay(env);
        let current_time = env.ledger().timestamp();
        let minimum_eta = current_time + min_delay.try_into().unwrap_or(0);

        let final_eta = if eta < minimum_eta { minimum_eta } else { eta };

        Self::set_time_lock_eta(env, hash, final_eta);

        Ok(final_eta)
    }

    pub fn cancel_time_lock(env: &Env, hash: Bytes) -> Result<(), ContractError> {
        if hash.len() == 0 {
            return Err(ContractError::InvalidTimeLockHash);
        }
        Self::set_time_lock_eta(env, hash, 0);
        Ok(())
    }

    pub fn finalize_time_lock(env: &Env, hash: Bytes) -> Result<(), ContractError> {
        let eta = Self::get_time_lock(env, hash.clone());

        if hash.len() == 0 || eta == 0 {
            return Err(ContractError::InvalidTimeLockHash);
        }

        let current_time = env.ledger().timestamp();
        if current_time < eta {
            return Err(ContractError::TimeLockNotReady);
        }

        Self::set_time_lock_eta(env, hash, 0);

        Ok(())
    }

    fn get_time_lock_eta(env: &Env, hash: Bytes) -> u64 {
        storage::try_proposal_time_lock(env, hash)
            .map(|eta| eta.try_into().unwrap_or(0))
            .unwrap_or(0)
    }

    fn set_time_lock_eta(env: &Env, hash: Bytes, eta: u64) {
        storage::set_proposal_time_lock(env, hash, &eta.into());
    }
}
