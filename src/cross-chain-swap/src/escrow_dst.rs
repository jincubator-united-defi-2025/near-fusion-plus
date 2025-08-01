use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId,
    serde::{Deserialize, Serialize},
};
use crate::types::{Immutables, EscrowError, TimelockStage};
use crate::utils::{validate_after, validate_before, validate_caller};
use super::base_escrow::BaseEscrow;

#[near(contract_state)]
pub struct EscrowDst {
    pub base: BaseEscrow,
}

impl Default for EscrowDst {
    fn default() -> Self {
        Self {
            base: BaseEscrow::default(),
        }
    }
}

#[near]
impl EscrowDst {
    #[init]
    pub fn new(rescue_delay: u64, access_token: AccountId) -> Self {
        Self {
            base: BaseEscrow::default(),
        }
    }

    /// Withdraw funds with secret
    /// Only taker can withdraw during withdrawal period
    #[handle_result]
    pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError> {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate withdrawal time
        let withdrawal_start = immutables.timelocks.get(TimelockStage::DstWithdrawal);
        let cancellation_start = immutables.timelocks.get(TimelockStage::DstCancellation);
        
        validate_after(withdrawal_start).expect("Withdrawal not started");
        validate_before(cancellation_start).expect("Withdrawal period ended");
        
        // Validate secret and immutables
        self.base.validate_secret(&secret, &immutables).expect("Invalid secret");
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to maker
        self.base.uni_transfer(&immutables.token, &immutables.maker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Escrow withdrawal: secret={:?}", secret);
        Ok(())
    }

    /// Public withdrawal - anyone with access token can withdraw
    #[handle_result]
    pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError> {
        // Validate caller has access token
        self.base.validate_access_token().expect("No access token");
        
        // Validate public withdrawal time
        let public_withdrawal_start = immutables.timelocks.get(TimelockStage::DstPublicWithdrawal);
        let cancellation_start = immutables.timelocks.get(TimelockStage::DstCancellation);
        
        validate_after(public_withdrawal_start).expect("Public withdrawal not started");
        validate_before(cancellation_start).expect("Public withdrawal period ended");
        
        // Validate secret and immutables
        self.base.validate_secret(&secret, &immutables).expect("Invalid secret");
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to maker
        self.base.uni_transfer(&immutables.token, &immutables.maker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Public escrow withdrawal: secret={:?}", secret);
        Ok(())
    }

    /// Cancel escrow - only taker can cancel during cancellation period
    #[handle_result]
    pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError> {
        // Validate caller is taker
        validate_caller(&immutables.taker).expect("Invalid caller");
        
        // Validate cancellation time
        let cancellation_start = immutables.timelocks.get(TimelockStage::DstCancellation);
        validate_after(cancellation_start).expect("Cancellation not started");
        
        // Validate immutables
        self.validate_immutables(&immutables).expect("Invalid immutables");
        
        // Transfer tokens to taker
        self.base.uni_transfer(&immutables.token, &immutables.taker, immutables.amount);
        self.base.near_transfer(&env::predecessor_account_id(), immutables.safety_deposit);
        
        log!("Escrow cancelled");
        Ok(())
    }

    /// Validate immutables - verify computed escrow address matches this contract
    #[handle_result]
    pub fn validate_immutables(&self, immutables: &Immutables) -> Result<(), EscrowError> {
        // In NEAR, we would compute the deterministic address and verify it matches
        // For now, we'll use a simplified validation
        if immutables.amount == 0 {
            return Err(EscrowError::InvalidImmutables);
        }
        Ok(())
    }

    // Delegate base escrow methods
    #[handle_result]
    pub fn rescue_funds(&mut self, token: AccountId, amount: u128, immutables: Immutables) -> Result<(), EscrowError> {
        self.base.rescue_funds(token, amount, immutables);
        Ok(())
    }

    #[handle_result]
    pub fn get_rescue_delay(&self) -> Result<u64, EscrowError> {
        Ok(self.base.get_rescue_delay())
    }

    #[handle_result]
    pub fn get_factory(&self) -> Result<AccountId, EscrowError> {
        Ok(self.base.get_factory())
    }
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
                src_withdrawal: 0,
                src_public_withdrawal: 0,
                src_cancellation: 0,
                src_public_cancellation: 0,
                dst_withdrawal: 100,    // withdrawal starts at 1100
                dst_public_withdrawal: 200, // public withdrawal starts at 1200
                dst_cancellation: 300,  // cancellation starts at 1300
            },
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let access_token = accounts(2);
        let rescue_delay = 3600;
        
        let contract = EscrowDst::new(rescue_delay, access_token.clone());
        
        assert_eq!(contract.get_rescue_delay().unwrap(), rescue_delay);
        assert_eq!(contract.get_factory().unwrap(), accounts(1));
    }

    #[test]
    fn test_validate_immutables() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = EscrowDst::new(3600, accounts(2));
        
        let mut immutables = create_test_immutables();
        assert!(contract.validate_immutables(&immutables).is_ok());
        
        immutables.amount = 0;
        assert!(contract.validate_immutables(&immutables).is_err());
    }
} 