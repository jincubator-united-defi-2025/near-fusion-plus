use crate::types::{EscrowError, Immutables, Stage};
use near_sdk::{env, AccountId};

/// Validate secret against hashlock
pub fn validate_secret(secret: &[u8; 32], hashlock: &[u8; 32]) -> bool {
    let computed_hash = near_sdk::env::keccak256(secret);
    computed_hash == *hashlock
}

/// Validate immutable values
pub fn validate_immutables(immutables: &Immutables) -> bool {
    // Basic validation - in a real implementation, more checks would be added
    immutables.amount > 0
        && !immutables.maker.as_str().is_empty()
        && !immutables.taker.as_str().is_empty()
        && !immutables.token.as_str().is_empty()
}

/// Check if current time is after the specified stage
pub fn is_after_stage(stage: Stage, timelocks: &crate::types::Timelocks) -> bool {
    let current_time = env::block_timestamp_ms() / 1000; // Convert to seconds
    current_time >= timelocks.get(stage)
}

/// Check if current time is before the specified stage
pub fn is_before_stage(stage: Stage, timelocks: &crate::types::Timelocks) -> bool {
    let current_time = env::block_timestamp_ms() / 1000; // Convert to seconds
    current_time < timelocks.get(stage)
}

/// Validate that caller is the taker
pub fn validate_taker(caller: &AccountId, immutables: &Immutables) -> Result<(), EscrowError> {
    if caller != &immutables.taker {
        return Err(EscrowError::OnlyTaker);
    }
    Ok(())
}

/// Validate that caller has access token (simplified for NEAR)
pub fn validate_access_token_holder(_caller: &AccountId) -> Result<(), EscrowError> {
    // In a real implementation, this would check if the caller holds the access token
    // For now, we'll allow any caller (simplified)
    Ok(())
}

/// Hash the secret to create hashlock
pub fn hash_secret(secret: &[u8; 32]) -> [u8; 32] {
    near_sdk::env::keccak256(secret).try_into().unwrap()
}
