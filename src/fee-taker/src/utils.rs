use near_sdk::{AccountId, NearToken, Promise, serde_json};
use crate::types::{FeeConfig, FeeTakerError};

/// Validate that caller is the limit order protocol
pub fn validate_limit_order_protocol(caller: &AccountId, limit_order_protocol: &AccountId) -> Result<(), FeeTakerError> {
    if caller != limit_order_protocol {
        return Err(FeeTakerError::OnlyLimitOrderProtocol);
    }
    Ok(())
}

/// Validate access token (simplified for NEAR)
pub fn validate_access_token(_caller: &AccountId, _access_token: &AccountId) -> Result<(), FeeTakerError> {
    // In a real implementation, this would check if the caller holds the access token
    // For now, we'll allow any caller (simplified)
    Ok(())
}

/// Parse fee configuration from extension data
pub fn parse_fee_config(extension: &[u8]) -> Result<FeeConfig, FeeTakerError> {
    if extension.len() < 32 { // Minimum size for fee data
        return Err(FeeTakerError::InvalidAmount);
    }
    
    // In a real implementation, this would properly parse the fee configuration
    // For now, we'll create a simplified version
    let fee_amount = if extension.len() >= 16 {
        u128::from_le_bytes(extension[0..16].try_into().unwrap())
    } else {
        0
    };
    
    let custom_receiver = if extension.len() >= 17 {
        extension[16] != 0
    } else {
        false
    };
    
    let fee_receiver = if extension.len() >= 49 {
        // Extract account ID from bytes (simplified)
        AccountId::try_from("fee_receiver.near".to_string()).unwrap()
    } else {
        AccountId::try_from("default_fee_receiver.near".to_string()).unwrap()
    };
    
    Ok(FeeConfig {
        fee_amount,
        fee_receiver,
        custom_receiver,
    })
}

/// Transfer tokens with fee
pub fn transfer_tokens_with_fee(
    token: &AccountId,
    from: &AccountId,
    to: &AccountId,
    amount: u128,
    fee_config: &FeeConfig,
) -> Result<(), FeeTakerError> {
    // Calculate fee amount
    let fee_amount = if fee_config.fee_amount > 0 {
        fee_config.fee_amount
    } else {
        0
    };
    
    let transfer_amount = amount.saturating_sub(fee_amount);
    
    if transfer_amount == 0 {
        return Err(FeeTakerError::InvalidAmount);
    }
    
    // Transfer main amount
    Promise::new(to.clone()).function_call(
        "ft_transfer".to_string(),
        serde_json::to_vec(&serde_json::json!({
            "receiver_id": to,
            "amount": transfer_amount.to_string(),
            "msg": ""
        })).unwrap(),
        NearToken::from_yoctonear(1),
        near_sdk::Gas::from_tgas(10),
    );
    
    // Transfer fee if applicable
    if fee_amount > 0 {
        Promise::new(fee_config.fee_receiver.clone()).function_call(
            "ft_transfer".to_string(),
            serde_json::to_vec(&serde_json::json!({
                "receiver_id": &fee_config.fee_receiver,
                "amount": fee_amount.to_string(),
                "msg": ""
            })).unwrap(),
            NearToken::from_yoctonear(1),
            near_sdk::Gas::from_tgas(10),
        );
    }
    
    Ok(())
}

/// Validate fee configuration
pub fn validate_fee_config(fee_config: &FeeConfig, receiver: &AccountId) -> Result<(), FeeTakerError> {
    if fee_config.fee_amount > 0 && !fee_config.custom_receiver && fee_config.fee_receiver != *receiver {
        return Err(FeeTakerError::InconsistentFee);
    }
    Ok(())
}

/// Calculate fee amount
pub fn calculate_fee_amount(amount: u128, fee_rate: u128) -> u128 {
    amount.saturating_mul(fee_rate).saturating_div(10000) // Basis points
}

/// Check if fee is applicable
pub fn is_fee_applicable(fee_config: &FeeConfig) -> bool {
    fee_config.fee_amount > 0
} 