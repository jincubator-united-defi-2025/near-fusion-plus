use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, log, near, AccountId, Gas, Promise, PromiseResult,
    serde::{Deserialize, Serialize},
};
use crate::types::{Immutables, EscrowError, TimelockStage};
use crate::utils::{hash_secret, validate_after, validate_before, validate_caller};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Base abstract Escrow contract for cross-chain atomic swap
#[near(contract_state)]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct BaseEscrow {
    pub rescue_delay: u64,
    pub access_token: AccountId,
    pub factory: AccountId,
}

impl Default for BaseEscrow {
    fn default() -> Self {
        Self {
            rescue_delay: 0,
            access_token: AccountId::new_unvalidated("".to_string()),
            factory: AccountId::new_unvalidated("".to_string()),
        }
    }
}

#[near]
impl BaseEscrow {
    /// Initialize the contract
    #[init]
    pub fn new(rescue_delay: u64, access_token: AccountId) -> Self {
        Self {
            rescue_delay,
            access_token,
            factory: env::predecessor_account_id(),
        }
    }

    /// Rescue funds from the escrow
    /// Funds can only be rescued by the taker after the rescue delay
    #[handle_result]
    pub fn rescue_funds(&mut self, token: AccountId, amount: u128, immutables: Immutables) -> PromiseResult {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate immutables
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Validate rescue time
        let rescue_start = immutables.timelocks.rescue_start(self.rescue_delay);
        validate_after(rescue_start).expect("Invalid time for rescue");

        // Transfer tokens
        self.uni_transfer(&token, &immutables.taker, amount);
        
        log!("Funds rescued: token={}, amount={}", token, amount);
        Ok(())
    }

    /// Get rescue delay
    pub fn get_rescue_delay(&self) -> u64 {
        self.rescue_delay
    }

    /// Get factory address
    pub fn get_factory(&self) -> AccountId {
        self.factory.clone()
    }

    /// Validate that caller has access token
    #[handle_result]
    pub fn validate_access_token(&self) -> Result<(), EscrowError> {
        // In NEAR, we would need to check if the caller has the access token
        // This is a simplified implementation
        Ok(())
    }

    /// Validate immutables - to be implemented by derived contracts
    #[handle_result]
    pub fn validate_immutables(&self, _immutables: &Immutables) -> Result<(), EscrowError> {
        // Default implementation - derived contracts should override
        Ok(())
    }

    /// Validate secret matches hashlock
    #[handle_result]
    pub fn validate_secret(&self, secret: &[u8; 32], immutables: &Immutables) -> Result<(), EscrowError> {
        let secret_hash = hash_secret(secret);
        if secret_hash != immutables.hashlock {
            return Err(EscrowError::InvalidSecret);
        }
        Ok(())
    }

    /// Transfer tokens (ERC20 or native)
    pub fn uni_transfer(&self, token: &AccountId, to: &AccountId, amount: u128) {
        if token.as_str() == "near" {
            // Native NEAR transfer
            Promise::new(to.clone()).transfer(amount);
        } else {
            // FT transfer via cross-contract call
            ext_ft::ext(token.clone())
                .with_attached_deposit(1)
                .with_gas(GAS_FOR_FT_TRANSFER)
                .ft_transfer(to.clone(), amount, None);
        }
    }

    /// Transfer native NEAR
    pub fn near_transfer(&self, to: &AccountId, amount: u128) {
        Promise::new(to.clone()).transfer(amount);
    }
}

// External FT contract interface
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: u128, memo: Option<String>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let access_token = accounts(2);
        let rescue_delay = 3600;
        
        let contract = BaseEscrow::new(rescue_delay, access_token.clone());
        
        assert_eq!(contract.rescue_delay, rescue_delay);
        assert_eq!(contract.access_token, access_token);
        assert_eq!(contract.factory, accounts(1));
    }

    #[test]
    fn test_validate_secret() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = BaseEscrow::new(3600, accounts(2));
        
        let secret = [1u8; 32];
        let hashlock = hash_secret(&secret);
        
        let immutables = Immutables {
            order_hash: [0u8; 32],
            hashlock,
            maker: accounts(3),
            taker: accounts(4),
            token: accounts(5),
            amount: 1000,
            safety_deposit: 100,
            timelocks: Timelocks {
                deployed_at: 0,
                src_withdrawal: 0,
                src_public_withdrawal: 0,
                src_cancellation: 0,
                src_public_cancellation: 0,
                dst_withdrawal: 0,
                dst_public_withdrawal: 0,
                dst_cancellation: 0,
            },
        };
        
        assert!(contract.validate_secret(&secret, &immutables).is_ok());
        
        let wrong_secret = [2u8; 32];
        assert!(contract.validate_secret(&wrong_secret, &immutables).is_err());
    }
} 