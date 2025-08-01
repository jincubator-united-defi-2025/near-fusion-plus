// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId, Gas, Promise, NearToken,
    collections::UnorderedMap,
    ext_contract,
};
use crate::types::{Order, Extension, MakerTraits, BitInvalidatorData, RemainingInvalidator, LimitOrderError};
use crate::utils::{hash_order, validate_signature, is_order_expired, validate_order_amounts};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Order mixin for processing limit orders
#[near(contract_state)]
pub struct OrderMixin {
    domain_separator: [u8; 32],
    weth: AccountId,
    bit_invalidator: UnorderedMap<AccountId, BitInvalidatorData>,
    remaining_invalidator: UnorderedMap<(AccountId, [u8; 32]), RemainingInvalidator>,
    paused: bool,
}

impl Default for OrderMixin {
    fn default() -> Self {
        Self {
            domain_separator: [0u8; 32],
            weth: AccountId::try_from("test.near".to_string()).unwrap(),
            bit_invalidator: UnorderedMap::new(b"b"),
            remaining_invalidator: UnorderedMap::new(b"r"),
            paused: false,
        }
    }
}

#[near]
impl OrderMixin {
    /// Initialize the contract
    #[init]
    pub fn new(domain_separator: [u8; 32], weth: AccountId) -> Self {
        Self {
            domain_separator,
            weth,
            bit_invalidator: UnorderedMap::new(b"b"),
            remaining_invalidator: UnorderedMap::new(b"r"),
            paused: false,
        }
    }

    /// Get bit invalidator for order
    pub fn bit_invalidator_for_order(&self, maker: AccountId, slot: u64) -> bool {
        if let Some(data) = self.bit_invalidator.get(&maker) {
            data.check_slot(slot)
        } else {
            false
        }
    }

    /// Get remaining invalidator for order
    pub fn remaining_invalidator_for_order(&self, maker: AccountId, order_hash: [u8; 32]) -> u128 {
        if let Some(invalidator) = self.remaining_invalidator.get(&(maker, order_hash)) {
            invalidator.remaining()
        } else {
            0
        }
    }

    /// Get raw remaining invalidator for order
    pub fn raw_remaining_invalidator_for_order(&self, maker: AccountId, order_hash: [u8; 32]) -> u128 {
        self.remaining_invalidator_for_order(maker, order_hash)
    }

    /// Simulate order execution
    pub fn simulate(&self, target: AccountId, data: Vec<u8>) {
        // In a real implementation, we would delegate the call
        // For now, we'll just log the simulation
        log!("Simulating call to {} with data: {:?}", target, data);
        env::panic_str("SimulationResults");
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, maker_traits: MakerTraits, order_hash: [u8; 32]) {
        let maker = env::predecessor_account_id();
        
        if maker_traits.use_bit_invalidator() {
            let mut data = self.bit_invalidator.get(&maker).unwrap_or_default();
            let invalidator = data.mass_invalidate(maker_traits.nonce_or_epoch(), 0);
            self.bit_invalidator.insert(&maker, &data);
            
            log!("Bit invalidator updated: maker={}, slot_index={}, slot_value={}", 
                 maker, maker_traits.nonce_or_epoch() >> 8, invalidator);
        } else {
            let invalidator = RemainingInvalidator::fully_filled();
            self.remaining_invalidator.insert(&(maker, order_hash), &invalidator);
            
            log!("Order cancelled: order_hash={:?}", order_hash);
        }
    }

    /// Cancel multiple orders
    pub fn cancel_orders(&mut self, maker_traits: Vec<MakerTraits>, order_hashes: Vec<[u8; 32]>) {
        if maker_traits.len() != order_hashes.len() {
            env::panic_str("Mismatch arrays lengths");
        }
        
        for i in 0..maker_traits.len() {
            self.cancel_order(maker_traits[i].clone(), order_hashes[i]);
        }
    }

    /// Fill order
    #[handle_result]
    pub fn fill_order(
        &mut self,
        order: Order,
        extension: Extension,
        signature: Vec<u8>,
        taker: AccountId,
        taking_amount: u128,
    ) -> Result<u128, LimitOrderError> {
        // Check if contract is paused
        if self.paused {
            return Err(LimitOrderError::ContractPaused);
        }

        // Validate order
        let order_hash = hash_order(&order, &self.domain_separator);
        if self.is_order_invalidated(&order, &order_hash) {
            return Err(LimitOrderError::OrderInvalidated);
        }

        // Validate signature
        if !validate_signature(&order, &signature, &order.maker) {
            return Err(LimitOrderError::InvalidSignature);
        }

        // Check if order is expired
        if is_order_expired(&order) {
            return Err(LimitOrderError::OrderExpired);
        }

        // Validate amounts
        if !validate_order_amounts(&order, taking_amount) {
            return Err(LimitOrderError::InvalidAmounts);
        }

        // Calculate making amount
        let making_amount = self.calculate_making_amount(
            &order,
            &extension,
            taking_amount,
            order.making_amount,
            &order_hash,
        )?;

        // Execute the swap
        self.execute_swap(&order, &taker, making_amount, taking_amount)?;

        // Update remaining amount
        self.update_remaining_amount(&order, &order_hash, taking_amount);

        log!("Order filled: order_hash={:?}, making_amount={}, taking_amount={}", 
             order_hash, making_amount, taking_amount);

        Ok(making_amount)
    }

    /// Check if order is invalidated
    fn is_order_invalidated(&self, order: &Order, order_hash: &[u8; 32]) -> bool {
        let maker = &order.maker;
        
        if order.maker_traits.use_bit_invalidator() {
            self.bit_invalidator_for_order(maker.clone(), order.maker_traits.nonce_or_epoch())
        } else {
            self.remaining_invalidator_for_order(maker.clone(), *order_hash) == 0
        }
    }

    /// Calculate making amount
    fn calculate_making_amount(
        &self,
        order: &Order,
        _extension: &Extension,
        requested_taking_amount: u128,
        _remaining_making_amount: u128,
        _order_hash: &[u8; 32],
    ) -> Result<u128, LimitOrderError> {
        // For now, use linear proportion
        // In a real implementation, this would use the extension data
        let making_amount = (order.making_amount * requested_taking_amount) / order.taking_amount;
        Ok(making_amount)
    }

    /// Execute the swap
    fn execute_swap(
        &self,
        order: &Order,
        taker: &AccountId,
        making_amount: u128,
        taking_amount: u128,
    ) -> Result<(), LimitOrderError> {
        // Transfer tokens from taker to maker
        self.transfer_tokens(&order.taker_asset, taker, &order.maker, taking_amount)?;
        
        // Transfer tokens from maker to taker
        self.transfer_tokens(&order.maker_asset, &order.maker, taker, making_amount)?;
        
        Ok(())
    }

    /// Transfer tokens
    fn transfer_tokens(
        &self,
        token: &AccountId,
        from: &AccountId,
        to: &AccountId,
        amount: u128,
    ) -> Result<(), LimitOrderError> {
        if token.as_str() == "near" {
            // Native NEAR transfer
            Promise::new(to.clone()).transfer(NearToken::from_yoctonear(amount));
        } else {
            // Fungible token transfer
            ext_ft::ext(token.clone())
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .with_attached_deposit(NearToken::from_yoctonear(1))
                .ft_transfer_from(from.clone(), to.clone(), amount, None);
        }
        Ok(())
    }

    /// Update remaining amount
    fn update_remaining_amount(&mut self, order: &Order, order_hash: &[u8; 32], filled_amount: u128) {
        let maker = &order.maker;
        
        if !order.maker_traits.use_bit_invalidator() {
            let current_remaining = self.remaining_invalidator_for_order(maker.clone(), *order_hash);
            let new_remaining = if current_remaining > filled_amount {
                current_remaining - filled_amount
            } else {
                0
            };
            
            let invalidator = RemainingInvalidator::new(new_remaining);
            self.remaining_invalidator.insert(&(maker.clone(), *order_hash), &invalidator);
        }
    }

    /// Pause the contract
    pub fn pause(&mut self) {
        self.paused = true;
        log!("Contract paused");
    }

    /// Unpause the contract
    pub fn unpause(&mut self) {
        self.paused = false;
        log!("Contract unpaused");
    }

    /// Check if contract is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Get domain separator
    pub fn get_domain_separator(&self) -> [u8; 32] {
        self.domain_separator
    }

    /// Get WETH token
    pub fn get_weth(&self) -> AccountId {
        self.weth.clone()
    }
}

// External contract trait for fungible token transfers
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: u128, memo: Option<String>);
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
        VMContextBuilder::new()
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(NearToken::from_yoctonear(1))
    }

    fn create_test_order() -> Order {
        Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: MakerTraits::default(),
        }
    }

    fn create_test_extension() -> Extension {
        Extension {
            maker_amount_data: vec![],
            taker_amount_data: vec![],
            predicate_data: vec![],
            permit_data: vec![],
            pre_interaction_data: vec![],
            post_interaction_data: vec![],
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = OrderMixin::new([0u8; 32], accounts(1));
        assert_eq!(contract.get_weth(), accounts(1));
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_pause_unpause() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = OrderMixin::new([0u8; 32], accounts(1));
        assert!(!contract.is_paused());

        contract.pause();
        assert!(contract.is_paused());

        contract.unpause();
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_cancel_order() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = OrderMixin::new([0u8; 32], accounts(1));
        let maker_traits = MakerTraits::default();
        let order_hash = [1u8; 32];

        contract.cancel_order(maker_traits, order_hash);
        // Test passes if no panic occurs
    }
} 