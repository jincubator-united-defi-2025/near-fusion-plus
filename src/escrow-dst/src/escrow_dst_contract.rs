// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId, Gas, Promise, NearToken,
    ext_contract,
};
use crate::types::{Immutables, Stage, EscrowError, Timelocks};
use crate::utils::{validate_secret, validate_immutables, is_after_stage, is_before_stage, validate_taker, validate_access_token_holder};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Destination Escrow contract for cross-chain atomic swap
#[near(contract_state)]
pub struct EscrowDst {
    rescue_delay: u32,
    access_token: AccountId,
    owner: AccountId,
}

impl Default for EscrowDst {
    fn default() -> Self {
        Self {
            rescue_delay: 3600, // 1 hour default
            access_token: AccountId::try_from("test.near".to_string()).unwrap(),
            owner: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

#[near]
impl EscrowDst {
    /// Initialize the contract
    #[init]
    pub fn new(rescue_delay: u32, access_token: AccountId) -> Self {
        Self {
            rescue_delay,
            access_token,
            owner: env::predecessor_account_id(),
        }
    }

    /// Withdraw funds using secret (taker only)
    #[handle_result]
    pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError> {
        let caller = env::predecessor_account_id();
        
        // Validate caller is taker
        validate_taker(&caller, &immutables)?;
        
        // Check timelocks
        if !is_after_stage(Stage::DstWithdrawal, &immutables.timelocks) {
            return Err(EscrowError::TimelockNotReached);
        }
        
        if !is_before_stage(Stage::DstCancellation, &immutables.timelocks) {
            return Err(EscrowError::TimelockExpired);
        }
        
        self.execute_withdrawal(secret, immutables)
    }

    /// Public withdrawal (access token holder only)
    #[handle_result]
    pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError> {
        let caller = env::predecessor_account_id();
        
        // Validate caller has access token
        validate_access_token_holder(&caller)?;
        
        // Check timelocks
        if !is_after_stage(Stage::DstPublicWithdrawal, &immutables.timelocks) {
            return Err(EscrowError::TimelockNotReached);
        }
        
        if !is_before_stage(Stage::DstCancellation, &immutables.timelocks) {
            return Err(EscrowError::TimelockExpired);
        }
        
        self.execute_withdrawal(secret, immutables)
    }

    /// Cancel escrow (taker only)
    #[handle_result]
    pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError> {
        let caller = env::predecessor_account_id();
        
        // Validate caller is taker
        validate_taker(&caller, &immutables)?;
        
        // Validate immutable values
        if !validate_immutables(&immutables) {
            return Err(EscrowError::InvalidImmutables);
        }
        
        // Check timelocks
        if !is_after_stage(Stage::DstCancellation, &immutables.timelocks) {
            return Err(EscrowError::TimelockNotReached);
        }
        
        self.execute_cancellation(caller, immutables)
    }

    /// Get rescue delay
    pub fn get_rescue_delay(&self) -> u32 {
        self.rescue_delay
    }

    /// Get access token
    pub fn get_access_token(&self) -> AccountId {
        self.access_token.clone()
    }

    /// Get owner
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    // Internal helper functions
    fn execute_withdrawal(&self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError> {
        // Validate immutable values
        if !validate_immutables(&immutables) {
            return Err(EscrowError::InvalidImmutables);
        }
        
        // Validate secret
        if !validate_secret(&secret, &immutables.hashlock) {
            return Err(EscrowError::InvalidSecret);
        }
        
        // Transfer tokens to maker
        self.transfer_tokens(&immutables.token, &immutables.maker, immutables.amount)?;
        
        // Transfer safety deposit to caller
        let caller = env::predecessor_account_id();
        Promise::new(caller).transfer(NearToken::from_yoctonear(immutables.safety_deposit));
        
        log!("Escrow withdrawal successful: secret={:?}", secret);
        
        Ok(())
    }

    fn execute_cancellation(&self, caller: AccountId, immutables: Immutables) -> Result<(), EscrowError> {
        // Transfer tokens back to taker
        self.transfer_tokens(&immutables.token, &immutables.taker, immutables.amount)?;
        
        // Transfer safety deposit to caller
        Promise::new(caller).transfer(NearToken::from_yoctonear(immutables.safety_deposit));
        
        log!("Escrow cancellation successful: taker={}", immutables.taker);
        
        Ok(())
    }

    fn transfer_tokens(&self, token: &AccountId, to: &AccountId, amount: u128) -> Result<(), EscrowError> {
        if token.as_str() == "near" {
            // Native NEAR transfer
            Promise::new(to.clone()).transfer(NearToken::from_yoctonear(amount));
        } else {
            // Fungible token transfer
            ext_ft::ext(token.clone())
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .with_attached_deposit(NearToken::from_yoctonear(1))
                .ft_transfer(to.clone(), amount, None);
        }
        Ok(())
    }
}

// External contract trait for fungible token transfers
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: u128, memo: Option<String>);
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(NearToken::from_yoctonear(1));
        builder
    }

    fn create_test_immutables() -> Immutables {
        Immutables {
            order_hash: [1u8; 32],
            hashlock: [2u8; 32],
            maker: accounts(0),
            taker: accounts(1),
            token: accounts(2),
            amount: 1000,
            safety_deposit: 100,
            timelocks: Timelocks {
                deployed_at: 1000,
                src_withdrawal: 1100,
                src_public_withdrawal: 1200,
                src_cancellation: 1300,
                src_public_cancellation: 1400,
                dst_withdrawal: 1500,
                dst_public_withdrawal: 1600,
                dst_cancellation: 1700,
            },
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = EscrowDst::new(3600, accounts(1));
        assert_eq!(contract.get_rescue_delay(), 3600);
        assert_eq!(contract.get_access_token(), accounts(1));
    }

    #[test]
    fn test_default() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = EscrowDst::default();
        assert_eq!(contract.get_rescue_delay(), 3600);
    }
} 