use near_sdk::AccountId;
use crate::types::{Order, Immutables, Timelocks, ExtraDataArgs, FactoryError, U256, ValidationData};

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

/// Check if multiple fills are allowed
pub fn allow_multiple_fills(traits: &crate::types::MakerTraits) -> bool {
    traits.use_bit_invalidator()
}

/// Validate partial fill
pub fn is_valid_partial_fill(
    making_amount: u128,
    remaining_making_amount: u128,
    order_making_amount: u128,
    parts_amount: u128,
    validated_index: u64,
) -> bool {
    if parts_amount < 2 {
        return false;
    }
    
    let expected_making_amount = order_making_amount * (validated_index as u128 + 1) / parts_amount;
    making_amount == expected_making_amount && remaining_making_amount == order_making_amount - making_amount
}

/// Process Merkle proof (simplified for NEAR)
pub fn process_merkle_proof(
    proof: &[[u8; 32]],
    leaf: [u8; 32],
    index: u64,
) -> Result<[u8; 32], FactoryError> {
    // In a real implementation, this would validate the Merkle proof
    // For now, we'll return the leaf hash (simplified)
    let mut current_hash = leaf;
    
    for (i, proof_element) in proof.iter().enumerate() {
        let bit = (index >> i) & 1;
        if bit == 0 {
            // Left child
            let mut data = Vec::new();
            data.extend_from_slice(&current_hash);
            data.extend_from_slice(proof_element);
            current_hash = near_sdk::env::keccak256(&data).try_into().unwrap();
        } else {
            // Right child
            let mut data = Vec::new();
            data.extend_from_slice(proof_element);
            data.extend_from_slice(&current_hash);
            current_hash = near_sdk::env::keccak256(&data).try_into().unwrap();
        }
    }
    
    Ok(current_hash)
}

/// Validate Merkle proof
pub fn validate_merkle_proof(
    proof: &[[u8; 32]],
    leaf: [u8; 32],
    index: u64,
    root: [u8; 32],
) -> bool {
    match process_merkle_proof(proof, leaf, index) {
        Ok(computed_root) => computed_root == root,
        Err(_) => false,
    }
}

/// Extract parts amount from hashlock info
pub fn extract_parts_amount(hashlock_info: &[u8; 32]) -> u128 {
    // Extract the high 16 bits as parts amount
    let mut parts_bytes = [0u8; 16];
    parts_bytes.copy_from_slice(&hashlock_info[0..16]);
    u128::from_le_bytes(parts_bytes)
}

/// Extract root from hashlock info
pub fn extract_root(hashlock_info: &[u8; 32]) -> [u8; 32] {
    // Extract the low 32 bytes as root
    let mut root = [0u8; 32];
    root.copy_from_slice(&hashlock_info[0..32]);
    root
}

/// Create key for validation data
pub fn create_validation_key(order_hash: &[u8; 32], root_shortened: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(order_hash);
    data.extend_from_slice(root_shortened);
    near_sdk::env::keccak256(&data).try_into().unwrap()
} 