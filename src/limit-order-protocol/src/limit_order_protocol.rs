use super::order_mixin::OrderMixin;
use crate::types::{Extension, LimitOrderError, MakerTraits, Order};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near,
    serde::{Deserialize, Serialize},
    AccountId,
};

/// Main Limit Order Protocol contract
#[near(contract_state)]
pub struct LimitOrderProtocol {
    pub order_mixin: OrderMixin,
    pub owner: AccountId,
}

impl Default for LimitOrderProtocol {
    fn default() -> Self {
        Self {
            order_mixin: OrderMixin::default(),
            owner: AccountId::new_unvalidated("".to_string()),
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

    /// Get WETH address
    pub fn get_weth(&self) -> AccountId {
        self.order_mixin.get_weth()
    }

    /// Check if caller is owner
    fn only_owner(&self) {
        if &env::predecessor_account_id() != &self.owner {
            env::panic_str("Only owner can call this function");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::json_types::U128;
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
        let contract = LimitOrderProtocol::new(domain_separator, weth.clone());

        assert_eq!(contract.domain_separator(), domain_separator);
        assert_eq!(contract.get_weth(), weth);
        assert_eq!(contract.get_owner(), accounts(1));
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_pause_unpause() {
        let context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = LimitOrderProtocol::new([1u8; 32], accounts(2));

        contract.pause();
        assert!(contract.is_paused());

        contract.unpause();
        assert!(!contract.is_paused());
    }

    #[test]
    fn test_cancel_order() {
        let context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = LimitOrderProtocol::new([1u8; 32], accounts(2));
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

    #[test]
    fn test_bit_invalidator() {
        let context = get_context(accounts(1));
        testing_env!(context.build());

        let contract = LimitOrderProtocol::new([1u8; 32], accounts(2));

        // Test bit invalidator for non-existent maker
        let result = contract.bit_invalidator_for_order(accounts(3), 0);
        assert!(!result);
    }
}
