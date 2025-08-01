use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, log, near, AccountId, Gas, Promise,
    serde::{Deserialize, Serialize},
    ext_contract,
    json_types::U128,
};
use near_sdk::ONE_NEAR;
use crate::types::{Order, Extension, LimitOrderError};
use crate::utils::{hash_order, calculate_making_amount, calculate_taking_amount, validate_extension, get_receiver};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Order library for processing limit orders
#[near(contract_state)]
pub struct OrderLib {
    pub domain_separator: [u8; 32],
}

impl Default for OrderLib {
    fn default() -> Self {
        Self {
            domain_separator: [0u8; 32],
        }
    }
}

#[near]
impl OrderLib {
    /// Initialize the contract
    #[init]
    pub fn new(domain_separator: [u8; 32]) -> Self {
        Self { domain_separator }
    }

    /// Calculate order hash
    pub fn hash_order(&self, order: Order) -> [u8; 32] {
        hash_order(&order, &self.domain_separator)
    }

    /// Get receiver for an order
    pub fn get_receiver(&self, order: Order) -> AccountId {
        get_receiver(&order)
    }

    /// Calculate making amount based on taking amount
    #[handle_result]
    pub fn calculate_making_amount(
        &self,
        order: Order,
        extension: Extension,
        requested_taking_amount: u128,
        remaining_making_amount: u128,
        order_hash: [u8; 32],
    ) -> Result<u128, LimitOrderError> {
        calculate_making_amount(
            &order,
            &extension,
            requested_taking_amount,
            remaining_making_amount,
            &order_hash,
        )
    }

    /// Calculate taking amount based on making amount
    #[handle_result]
    pub fn calculate_taking_amount(
        &self,
        order: Order,
        extension: Extension,
        requested_making_amount: u128,
        remaining_making_amount: u128,
        order_hash: [u8; 32],
    ) -> Result<u128, LimitOrderError> {
        calculate_taking_amount(
            &order,
            &extension,
            requested_making_amount,
            remaining_making_amount,
            &order_hash,
        )
    }

    /// Validate extension for an order
    #[handle_result]
    pub fn validate_extension(&self, order: Order, extension: Extension) -> Result<bool, LimitOrderError> {
        validate_extension(&order, &extension)
    }

    /// Process order execution
    #[handle_result]
    pub fn process_order(
        &mut self,
        order: Order,
        extension: Extension,
        taker: AccountId,
        taking_amount: u128,
    ) -> Result<u128, LimitOrderError> {
        // Validate order amounts
        if order.making_amount.0 == 0 || order.taking_amount.0 == 0 {
            return Err(LimitOrderError::SwapWithZeroAmount);
        }

        // Calculate making amount
        let order_hash = self.hash_order(order.clone());
        let making_amount = self.calculate_making_amount(
            order.clone(),
            extension.clone(),
            taking_amount,
            order.making_amount.0,
            order_hash,
        )?;

        // Validate extension
        self.validate_extension(order.clone(), extension)?;

        // Execute the swap
        self.execute_swap(&order, &taker, making_amount, taking_amount)?;

        log!("Order executed: maker={}, taker={}, making_amount={}, taking_amount={}", 
             order.maker, taker, making_amount, taking_amount);

        Ok(making_amount)
    }

    /// Execute the actual swap
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

    /// Get domain separator
    pub fn get_domain_separator(&self) -> [u8; 32] {
        self.domain_separator
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
            maker_traits: crate::types::MakerTraits {
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
        let contract = OrderLib::new(domain_separator);
        
        assert_eq!(contract.get_domain_separator(), domain_separator);
    }

    #[test]
    fn test_hash_order() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = OrderLib::new([1u8; 32]);
        let order = create_test_order();
        
        let hash = contract.hash_order(order);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_get_receiver() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = OrderLib::new([1u8; 32]);
        let order = create_test_order();
        
        let receiver = contract.get_receiver(order);
        assert_eq!(receiver, accounts(2));
    }

    #[test]
    fn test_validate_extension() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        
        let contract = OrderLib::new([1u8; 32]);
        let order = create_test_order();
        let extension = create_test_extension();
        
        let result = contract.validate_extension(order, extension);
        assert!(result.is_ok());
    }
} 