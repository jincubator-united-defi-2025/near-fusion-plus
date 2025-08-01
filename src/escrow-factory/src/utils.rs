use near_sdk::AccountId;
use crate::types::{Order, Immutables, Timelocks, ExtraDataArgs, FactoryError, U256};

/// Compute hash of an order
pub fn hash_order(order: &Order, domain_separator: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(domain_separator);
    data.extend_from_slice(&order.salt.to_le_bytes());
    data.extend_from_slice(order.maker.as_bytes());
    data.extend_from_slice(order.receiver.as_bytes());
    data.extend_from_slice(order.maker_asset.as_bytes());
    data.extend_from_slice(order.taker_asset.as_bytes());
    data.extend_from_slice(&order.making_amount.to_le_bytes());
    data.extend_from_slice(&order.taking_amount.to_le_bytes());

    // Hash maker traits
    let traits_hash = hash_maker_traits(&order.maker_traits);
    data.extend_from_slice(&traits_hash);

    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Hash maker traits
pub fn hash_maker_traits(traits: &crate::types::MakerTraits) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&(traits.use_bit_invalidator as u8).to_le_bytes());
    data.extend_from_slice(&(traits.use_epoch_manager as u8).to_le_bytes());
    data.extend_from_slice(&(traits.has_extension as u8).to_le_bytes());
    data.extend_from_slice(&traits.nonce_or_epoch.to_le_bytes());
    data.extend_from_slice(&traits.series.to_le_bytes());

    near_sdk::env::keccak256(&data).try_into().unwrap()
}

/// Validate order
pub fn validate_order(order: &Order) -> bool {
    order.making_amount > 0 && 
    order.taking_amount > 0 && 
    !order.maker.as_str().is_empty() && 
    !order.receiver.as_str().is_empty() &&
    !order.maker_asset.as_str().is_empty() &&
    !order.taker_asset.as_str().is_empty()
}

/// Parse extra data arguments
pub fn parse_extra_data_args(extra_data: &[u8]) -> Result<ExtraDataArgs, FactoryError> {
    if extra_data.len() < 64 { // Minimum size for hashlock_info + deposits + timelocks
        return Err(FactoryError::InvalidExtraData);
    }
    
    // In a real implementation, this would properly deserialize the extra data
    // For now, we'll create a simplified version
    let mut hashlock_info = [0u8; 32];
    if extra_data.len() >= 32 {
        hashlock_info.copy_from_slice(&extra_data[0..32]);
    }
    
    let deposits = U256 { value: 0 }; // Simplified
    let timelocks = Timelocks::default(); // Simplified
    
    Ok(ExtraDataArgs {
        hashlock_info,
        deposits,
        timelocks,
    })
}

/// Validate that caller is the owner
pub fn validate_owner(caller: &AccountId, owner: &AccountId) -> Result<(), FactoryError> {
    if caller != owner {
        return Err(FactoryError::OnlyOwner);
    }
    Ok(())
}

/// Validate access token (simplified for NEAR)
pub fn validate_access_token(_caller: &AccountId, _access_token: &AccountId) -> Result<(), FactoryError> {
    // In a real implementation, this would check if the caller holds the access token
    // For now, we'll allow any caller (simplified)
    Ok(())
}

/// Create immutable values for escrow
pub fn create_immutables(
    order: &Order,
    order_hash: [u8; 32],
    hashlock: [u8; 32],
    taker: AccountId,
    making_amount: u128,
    safety_deposit: u128,
    timelocks: Timelocks,
) -> Immutables {
    Immutables {
        order_hash,
        hashlock,
        maker: order.maker.clone(),
        taker,
        token: order.maker_asset.clone(),
        amount: making_amount,
        safety_deposit,
        timelocks,
    }
} 