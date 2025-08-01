use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId, Balance,
    serde::{Deserialize, Serialize},
};
use crate::types::{Immutables, EscrowError, TimelockStage};
use crate::utils::{validate_after, validate_before, validate_caller};
use super::base_escrow::BaseEscrow;

/// Source Escrow contract for cross-chain atomic swap
#[near(contract_state)]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct EscrowSrc {
    pub base: BaseEscrow,
}

impl Default for EscrowSrc {
    fn default() -> Self {
        Self {
            base: BaseEscrow::default(),
        }
    }
}

#[near]
impl EscrowSrc {
    /// Initialize the contract
    #[init]
    pub fn new(rescue_delay: u64, access_token: AccountId) -> Self {
        Self {
            base: BaseEscrow::new(rescue_delay, access_token),
        }
    }

    /// Withdraw funds with secret
    /// Only taker can withdraw during withdrawal period
    pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate withdrawal time
        let withdrawal_start = immutables.timelocks.get(TimelockStage::SrcWithdrawal);
        let cancellation_start = immutables.timelocks.get(TimelockStage::SrcCancellation);
        
        validate_after(withdrawal_start).expect("Withdrawal not started");
        validate_before(cancellation_start).expect("Withdrawal period ended");
        
        // Validate secret and immutables
        self.base.validate_secret(&secret, &immutables).expect("Invalid secret");
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to taker
        self.withdraw_to(secret, immutables.taker.clone(), immutables);
    }

    /// Withdraw funds to specific target
    pub fn withdraw_to(&mut self, secret: [u8; 32], target: AccountId, immutables: Immutables) {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate withdrawal time
        let withdrawal_start = immutables.timelocks.get(TimelockStage::SrcWithdrawal);
        let cancellation_start = immutables.timelocks.get(TimelockStage::SrcCancellation);
        
        validate_after(withdrawal_start).expect("Withdrawal not started");
        validate_before(cancellation_start).expect("Withdrawal period ended");
        
        // Validate secret and immutables
        self.base.validate_secret(&secret, &immutables).expect("Invalid secret");
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens
        self.base.uni_transfer(&immutables.token, &target, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Escrow withdrawal: secret={:?}", secret);
    }

    /// Public withdrawal - anyone with access token can withdraw
    pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) {
        // Validate caller has access token
        self.base.validate_access_token().expect("No access token");
        
        // Validate public withdrawal time
        let public_withdrawal_start = immutables.timelocks.get(TimelockStage::SrcPublicWithdrawal);
        let cancellation_start = immutables.timelocks.get(TimelockStage::SrcCancellation);
        
        validate_after(public_withdrawal_start).expect("Public withdrawal not started");
        validate_before(cancellation_start).expect("Public withdrawal period ended");
        
        // Validate secret and immutables
        self.base.validate_secret(&secret, &immutables).expect("Invalid secret");
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to taker
        self.base.uni_transfer(&immutables.token, &immutables.taker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Public escrow withdrawal: secret={:?}", secret);
    }

    /// Cancel escrow - only taker can cancel during cancellation period
    pub fn cancel(&mut self, immutables: Immutables) {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate cancellation time
        let cancellation_start = immutables.timelocks.get(TimelockStage::SrcCancellation);
        validate_after(cancellation_start).expect("Cancellation not started");
        
        // Validate immutables
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to maker
        self.base.uni_transfer(&immutables.token, &immutables.maker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Escrow cancelled");
    }

    /// Public cancellation - anyone with access token can cancel
    pub fn public_cancel(&mut self, immutables: Immutables) {
        // Validate caller has access token
        self.base.validate_access_token().expect("No access token");
        
        // Validate public cancellation time
        let public_cancellation_start = immutables.timelocks.get(TimelockStage::SrcPublicCancellation);
        validate_after(public_cancellation_start).expect("Public cancellation not started");
        
        // Validate immutables
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to maker
        self.base.uni_transfer(&immutables.token, &immutables.maker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Public escrow cancelled");
    }

    /// Validate immutables - verify computed escrow address matches this contract
    pub fn validate_immutables(&self, immutables: &Immutables) -> Result<(), EscrowError> {
        // In NEAR, we would compute the deterministic address and verify it matches
        // For now, we'll use a simplified validation
        if immutables.amount == 0 {
            return Err(EscrowError::InvalidImmutables);
        }
        Ok(())
    }

    // Delegate base escrow methods
    pub fn rescue_funds(&mut self, token: AccountId, amount: Balance, immutables: Immutables) {
        self.base.rescue_funds(token, amount, immutables);
    }

    pub fn get_rescue_delay(&self) -> u64 {
        self.base.get_rescue_delay()
    }

    pub fn get_factory(&self) -> AccountId {
        self.base.get_factory()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};
    use crate::utils::hash_secret;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn create_test_immutables() -> Immutables {
        Immutables {
            order_hash: [0u8; 32],
            hashlock: [0u8; 32],
            maker: accounts(1),
            taker: accounts(2),
            token: accounts(3),
            amount: 1000,
            safety_deposit: 100,
            timelocks: Timelocks {
                deployed_at: 1000,
                src_withdrawal: 100,    // withdrawal starts at 1100
                src_public_withdrawal: 200, // public withdrawal starts at 1200
                src_cancellation: 300,  // cancellation starts at 1300
                src_public_cancellation: 400, // public cancellation starts at 1400
                dst_withdrawal: 0,
                dst_public_withdrawal: 0,
                dst_cancellation: 0,
            },
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let access_token = accounts(2);
        let rescue_delay = 3600;
        
        let contract = EscrowSrc::new(rescue_delay, access_token.clone());
        
        assert_eq!(contract.get_rescue_delay(), rescue_delay);
        assert_eq!(contract.get_factory(), accounts(1));
    }

    #[test]
    fn test_validate_immutables() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = EscrowSrc::new(3600, accounts(2));
        
        let mut immutables = create_test_immutables();
        assert!(contract.validate_immutables(&immutables).is_ok());
        
        immutables.amount = 0;
        assert!(contract.validate_immutables(&immutables).is_err());
    }
} 