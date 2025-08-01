// Find all our documentation at https://docs.near.org
use super::order_mixin::OrderMixin;
use crate::types::{Extension, LimitOrderError, MakerTraits, Order};
use near_sdk::{env, near, AccountId, NearToken};

/// Main Limit Order Protocol contract
#[near(contract_state)]
pub struct LimitOrderProtocol {
    order_mixin: OrderMixin,
    owner: AccountId,
}

impl Default for LimitOrderProtocol {
    fn default() -> Self {
        Self {
            order_mixin: OrderMixin::default(),
            owner: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

#[near]
impl LimitOrderProtocol {
    /// Initialize the contract
    #[init]
    pub fn new(domain_separator: [u8; 32], weth: AccountId) -> Self {
        Self {
            order_mixin: OrderMixin::new(domain_separator, weth),
            owner: env::predecessor_account_id(),
        }
    }

    /// Get domain separator
    pub fn domain_separator(&self) -> [u8; 32] {
        self.order_mixin.get_domain_separator()
    }

    /// Pause all trading functionality
    pub fn pause(&mut self) {
        self.only_owner();
        self.order_mixin.pause();
    }

    /// Unpause all trading functionality
    pub fn unpause(&mut self) {
        self.only_owner();
        self.order_mixin.unpause();
    }

    /// Check if contract is paused
    pub fn is_paused(&self) -> bool {
        self.order_mixin.is_paused()
    }

    /// Get bit invalidator for order
    pub fn bit_invalidator_for_order(&self, maker: AccountId, slot: u64) -> bool {
        self.order_mixin.bit_invalidator_for_order(maker, slot)
    }

    /// Get remaining invalidator for order
    pub fn remaining_invalidator_for_order(&self, maker: AccountId, order_hash: [u8; 32]) -> u128 {
        self.order_mixin
            .remaining_invalidator_for_order(maker, order_hash)
    }

    /// Get raw remaining invalidator for order
    pub fn raw_remaining_invalidator_for_order(
        &self,
        maker: AccountId,
        order_hash: [u8; 32],
    ) -> u128 {
        self.order_mixin
            .raw_remaining_invalidator_for_order(maker, order_hash)
    }

    /// Simulate order execution
    pub fn simulate(&self, target: AccountId, data: Vec<u8>) {
        self.order_mixin.simulate(target, data);
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, maker_traits: MakerTraits, order_hash: [u8; 32]) {
        self.order_mixin.cancel_order(maker_traits, order_hash);
    }

    /// Cancel multiple orders
    pub fn cancel_orders(&mut self, maker_traits: Vec<MakerTraits>, order_hashes: Vec<[u8; 32]>) {
        self.order_mixin.cancel_orders(maker_traits, order_hashes);
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
        self.order_mixin
            .fill_order(order, extension, signature, taker, taking_amount)
    }

    /// Get owner
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    /// Get WETH token
    pub fn get_weth(&self) -> AccountId {
        self.order_mixin.get_weth()
    }

    /// Only owner modifier
    fn only_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "Only owner can call this");
    }
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

        let contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        assert_eq!(contract.get_owner(), accounts(0));
        assert_eq!(contract.get_weth(), accounts(1));
    }

    #[test]
    fn test_pause_unpause() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
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

        let mut contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        let maker_traits = MakerTraits::default();
        let order_hash = [1u8; 32];

        contract.cancel_order(maker_traits, order_hash);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_bit_invalidator() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = LimitOrderProtocol::new([0u8; 32], accounts(1));
        let result = contract.bit_invalidator_for_order(accounts(1), 0);
        assert!(!result); // Should be false for default state
    }
}
