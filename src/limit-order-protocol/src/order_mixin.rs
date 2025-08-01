use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId, Gas, Promise,
    serde::{Deserialize, Serialize},
    collections::UnorderedMap,
    ext_contract,
    json_types::U128,
};
use near_sdk::ONE_NEAR;
use crate::types::{Order, Extension, MakerTraits, BitInvalidatorData, RemainingInvalidator, LimitOrderError};
use crate::utils::{hash_order, validate_signature, is_order_expired, validate_order_amounts};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Order mixin for processing limit orders
#[near(contract_state)]
pub struct OrderMixin {
    pub domain_separator: [u8; 32],
    pub weth: AccountId,
    pub bit_invalidator: UnorderedMap<AccountId, BitInvalidatorData>,
    pub remaining_invalidator: UnorderedMap<(AccountId, [u8; 32]), RemainingInvalidator>,
    pub paused: bool,
}

impl Default for OrderMixin {
    fn default() -> Self {
        Self {
            domain_separator: [0u8; 32],
            weth: AccountId::new_unvalidated("".to_string()),
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
            env::panic_str("MismatchArraysLengths");
        }
        
        for (maker_traits, order_hash) in maker_traits.into_iter().zip(order_hashes) {
            self.cancel_order(maker_traits, order_hash);
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
            env::panic_str("Contract is paused");
        }

        // Validate order amounts
        validate_order_amounts(&order)?;

        // Validate signature
        if !validate_signature(&order, &signature, &order.maker)? {
            return Err(LimitOrderError::BadSignature);
        }

        // Check if order is expired
        if is_order_expired(&order, env::block_timestamp()) {
            return Err(LimitOrderError::OrderExpired);
        }

        // Check if order is invalidated
        let order_hash = hash_order(&order, &self.domain_separator);
        if self.is_order_invalidated(&order, &order_hash) {
            return Err(LimitOrderError::InvalidatedOrder);
        }

        // Calculate making amount
        let making_amount = self.calculate_making_amount(
            &order,
            &extension,
            taking_amount,
            order.making_amount.0,
            &order_hash,
        )?;

        // Execute the swap
        self.execute_swap(&order, &taker, making_amount, taking_amount)?;

        // Update remaining amount
        self.update_remaining_amount(&order, &order_hash, making_amount);

        log!("Order filled: order_hash={:?}, remaining_amount={}", order_hash, order.making_amount.0 - making_amount);

        Ok(making_amount)
    }

    /// Check if order is invalidated
    fn is_order_invalidated(&self, order: &Order, order_hash: &[u8; 32]) -> bool {
        if order.maker_traits.use_bit_invalidator() {
            let slot = order.maker_traits.nonce_or_epoch() >> 8;
            self.bit_invalidator_for_order(order.maker.clone(), slot)
        } else {
            let remaining = self.remaining_invalidator_for_order(order.maker.clone(), *order_hash);
            remaining == 0
        }
    }

    /// Calculate making amount
    fn calculate_making_amount(
        &self,
        order: &Order,
        extension: &Extension,
        requested_taking_amount: u128,
        _remaining_making_amount: u128,
        _order_hash: &[u8; 32],
    ) -> Result<u128, LimitOrderError> {
        if extension.making_amount_data().is_empty() {
            // Linear proportion
            if order.taking_amount.0 == 0 {
                return Err(LimitOrderError::SwapWithZeroAmount);
            }
            return Ok((order.making_amount.0 * requested_taking_amount) / order.taking_amount.0);
        }
        
        // In a real implementation, we would call an external getter contract
        Ok(requested_taking_amount)
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

    /// Transfer tokens between accounts
    fn transfer_tokens(
        &self,
        token: &AccountId,
        from: &AccountId,
        to: &AccountId,
        amount: u128,
    ) -> Result<(), LimitOrderError> {
        if token.as_str() == "near" {
            // Native NEAR transfer
            Promise::new(to.clone()).transfer(amount * ONE_NEAR);
        } else {
            // FT transfer via cross-contract call
            ext_ft::ext(token.clone())
                .with_attached_deposit(ONE_NEAR)
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .ft_transfer_from(from.clone(), to.clone(), U128(amount), None);
        }
        Ok(())
    }

    /// Update remaining amount for an order
    fn update_remaining_amount(&mut self, order: &Order, order_hash: &[u8; 32], filled_amount: u128) {
        if !order.maker_traits.use_bit_invalidator() {
            let key = (order.maker.clone(), *order_hash);
            let mut invalidator = self.remaining_invalidator.get(&key).unwrap_or_default();
            let new_remaining = if invalidator.remaining() > filled_amount {
                invalidator.remaining() - filled_amount
            } else {
                0
            };
            invalidator.update_remaining(new_remaining);
            self.remaining_invalidator.insert(&key, &invalidator);
        }
    }

    /// Pause the contract
    pub fn pause(&mut self) {
        // In a real implementation, we would check if caller is owner
        self.paused = true;
        log!("Contract paused");
    }

    /// Unpause the contract
    pub fn unpause(&mut self) {
        // In a real implementation, we would check if caller is owner
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

    /// Get WETH address
    pub fn get_weth(&self) -> AccountId {
        self.weth.clone()
    }
}

// External FT contract interface
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};
    use near_sdk::json_types::U128;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn create_test_order() -> Order {
        Order {
            salt: 12345,
            maker: accounts(1),
            receiver: accounts(2),
            maker_asset: accounts(3),
            taker_asset: accounts(4),
            making_amount: U128(1000),
            taking_amount: U128(500),
            maker_traits: MakerTraits {
                use_bit_invalidator: false,
                use_epoch_manager: false,
                has_extension: false,
                nonce_or_epoch: 0,
                series: 0,
            },
        }
    }

    fn create_test_extension() -> Extension {
        Extension {
            making_amount_data: vec![],
            taking_amount_data: vec![],
            predicate_data: vec![],
            interaction_data: vec![],
        }
    }

    #[test]
    fn test_new() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let domain_separator = [1u8; 32];
        let weth = accounts(2);
        let contract = OrderMixin::new(domain_separator, weth.clone());
        
        assert_eq!(contract.get_domain_separator(), domain_separator);
        assert_eq!(contract.get_weth(), weth);
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_pause_unpause() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let mut contract = OrderMixin::new([1u8; 32], accounts(2));
        
        contract.pause();
        assert!(contract.is_paused());
        
        contract.unpause();
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_cancel_order() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let mut contract = OrderMixin::new([1u8; 32], accounts(2));
        let maker_traits = MakerTraits {
            use_bit_invalidator: false,
            use_epoch_manager: false,
            has_extension: false,
            nonce_or_epoch: 0,
            series: 0,
        };
        let order_hash = [1u8; 32];
        
        contract.cancel_order(maker_traits, order_hash);
        
        // Check that order is invalidated
        let remaining = contract.remaining_invalidator_for_order(accounts(1), order_hash);
        assert_eq!(remaining, 0);
    }
} 