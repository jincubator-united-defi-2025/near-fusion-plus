use near_sdk::{
    env,
    hash::{hash, CryptoHash},
};
use crate::types::{Immutables, Timelocks, TimelockStage, ValidationData, EscrowError};

/// Compute hash of immutables for deterministic address generation
pub fn hash_immutables(immutables: &Immutables) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&immutables.order_hash);
    data.extend_from_slice(&immutables.hashlock);
    data.extend_from_slice(immutables.maker.as_bytes());
    data.extend_from_slice(immutables.taker.as_bytes());
    data.extend_from_slice(immutables.token.as_bytes());
    data.extend_from_slice(&immutables.amount.to_le_bytes());
    data.extend_from_slice(&immutables.safety_deposit.to_le_bytes());
    
    // Hash timelocks
    let timelocks_hash = hash_timelocks(&immutables.timelocks);
    data.extend_from_slice(&timelocks_hash);
    
    hash(&data).try_into().unwrap()
}

/// Hash timelocks structure
pub fn hash_timelocks(timelocks: &Timelocks) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&timelocks.deployed_at.to_le_bytes());
    data.extend_from_slice(&timelocks.src_withdrawal.to_le_bytes());
    data.extend_from_slice(&timelocks.src_public_withdrawal.to_le_bytes());
    data.extend_from_slice(&timelocks.src_cancellation.to_le_bytes());
    data.extend_from_slice(&timelocks.src_public_cancellation.to_le_bytes());
    data.extend_from_slice(&timelocks.dst_withdrawal.to_le_bytes());
    data.extend_from_slice(&timelocks.dst_public_withdrawal.to_le_bytes());
    data.extend_from_slice(&timelocks.dst_cancellation.to_le_bytes());
    
    hash(&data).try_into().unwrap()
}

/// Compute hash of a secret
pub fn hash_secret(secret: &[u8; 32]) -> [u8; 32] {
    hash(secret).try_into().unwrap()
}

/// Validate that the current time is after the given timestamp
pub fn validate_after(start: u64) -> Result<(), EscrowError> {
    if env::block_timestamp() < start {
        return Err(EscrowError::InvalidTime);
    }
    Ok(())
}

/// Validate that the current time is before the given timestamp
pub fn validate_before(stop: u64) -> Result<(), EscrowError> {
    if env::block_timestamp() >= stop {
        return Err(EscrowError::InvalidTime);
    }
    Ok(())
}

/// Validate that the caller is the expected account
pub fn validate_caller(expected: &near_sdk::AccountId) -> Result<(), EscrowError> {
    if &env::predecessor_account_id() != expected {
        return Err(EscrowError::InvalidCaller);
    }
    Ok(())
}

/// Validate partial fill for multiple secrets
pub fn validate_partial_fill(
    making_amount: u128,
    remaining_making_amount: u128,
    order_making_amount: u128,
    parts_amount: u64,
    validated_index: u64,
) -> Result<bool, EscrowError> {
    if parts_amount < 2 {
        return Err(EscrowError::InvalidSecretsAmount);
    }
    
    let calculated_index = if order_making_amount > 0 {
        ((order_making_amount - remaining_making_amount + making_amount - 1) * parts_amount as u128 / order_making_amount) as u64
    } else {
        0
    };

    if remaining_making_amount == making_amount {
        // If the order is filled to completion, a secret with index i + 1 must be used
        // where i is the index of the secret for the last part.
        Ok(calculated_index + 2 == validated_index)
    } else if order_making_amount != remaining_making_amount {
        // Calculate the previous fill index only if this is not the first fill.
        let prev_calculated_index = if order_making_amount > 0 {
            ((order_making_amount - remaining_making_amount - 1) * parts_amount as u128 / order_making_amount) as u64
        } else {
            0
        };
        if calculated_index == prev_calculated_index {
            Ok(false)
        } else {
            Ok(calculated_index + 1 == validated_index)
        }
    } else {
        Ok(calculated_index + 1 == validated_index)
    }
} 