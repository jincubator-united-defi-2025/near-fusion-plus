// Find all our documentation at https://docs.near.org
use crate::types::{Extension, LimitOrderError, Order};
use near_sdk::{ext_contract, log, AccountId, Gas, NearToken, Promise, borsh::{BorshSerialize, BorshDeserialize}};

// Gas for cross-contract calls
const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

/// Order library for processing limit orders
#[derive(BorshSerialize, BorshDeserialize)]
pub struct OrderLib {
    domain_separator: [u8; 32],
}

impl Default for OrderLib {
    fn default() -> Self {
        Self {
            domain_separator: [0u8; 32],
        }
    }
}

impl OrderLib {
    /// Initialize the contract
    pub fn new(domain_separator: [u8; 32]) -> Self {
        Self { domain_separator }
    }

    /// Calculate order hash
    pub fn hash_order(&self, order: Order) -> [u8; 32] {
        self.compute_order_hash(&order)
    }

    /// Get receiver for an order
    pub fn get_receiver(&self, order: Order) -> AccountId {
        if order.receiver.as_str() == "0x0000000000000000000000000000000000000000" {
            order.maker
        } else {
            order.receiver
        }
    }

    /// Calculate making amount based on taking amount
    pub fn calculate_making_amount(
        &self,
        order: Order,
        extension: Extension,
        requested_taking_amount: u128,
        remaining_making_amount: u128,
        order_hash: [u8; 32],
    ) -> Result<u128, LimitOrderError> {
        self.compute_making_amount(
            &order,
            &extension,
            requested_taking_amount,
            remaining_making_amount,
            &order_hash,
        )
    }

    /// Calculate taking amount based on making amount
    pub fn calculate_taking_amount(
        &self,
        order: Order,
        extension: Extension,
        requested_making_amount: u128,
        remaining_making_amount: u128,
        order_hash: [u8; 32],
    ) -> Result<u128, LimitOrderError> {
        self.compute_taking_amount(
            &order,
            &extension,
            requested_making_amount,
            remaining_making_amount,
            &order_hash,
        )
    }

    /// Validate extension for an order
    pub fn validate_extension(
        &self,
        order: Order,
        extension: Extension,
    ) -> Result<bool, LimitOrderError> {
        self.validate_order_extension(&order, &extension)
    }

    /// Process order execution
    pub fn process_order(
        &mut self,
        order: Order,
        extension: Extension,
        taker: AccountId,
        taking_amount: u128,
    ) -> Result<u128, LimitOrderError> {
        // Validate order amounts
        if taking_amount == 0 {
            return Err(LimitOrderError::SwapWithZeroAmount);
        }

        // Calculate making amount
        let order_hash = self.hash_order(order.clone());
        let making_amount = self.calculate_making_amount(
            order.clone(),
            extension.clone(),
            taking_amount,
            order.making_amount,
            order_hash,
        )?;

        // Validate extension
        if !self.validate_extension(order.clone(), extension)? {
            return Err(LimitOrderError::InvalidExtension);
        }

        // Execute the swap
        self.execute_swap(&order, &taker, making_amount, taking_amount)?;

        log!(
            "Order processed: making_amount={}, taking_amount={}",
            making_amount,
            taking_amount
        );

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

    /// Get domain separator
    pub fn get_domain_separator(&self) -> [u8; 32] {
        self.domain_separator
    }

    // Internal helper functions
    fn compute_order_hash(&self, order: &Order) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&self.domain_separator);
        data.extend_from_slice(&order.salt.to_le_bytes());
        data.extend_from_slice(order.maker.as_bytes());
        data.extend_from_slice(order.receiver.as_bytes());
        data.extend_from_slice(order.maker_asset.as_bytes());
        data.extend_from_slice(order.taker_asset.as_bytes());
        data.extend_from_slice(&order.making_amount.to_le_bytes());
        data.extend_from_slice(&order.taking_amount.to_le_bytes());

        // Hash maker traits
        let traits_hash = self.hash_maker_traits(&order.maker_traits);
        data.extend_from_slice(&traits_hash);

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }

    fn hash_maker_traits(&self, traits: &crate::types::MakerTraits) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(&(traits.use_bit_invalidator as u8).to_le_bytes());
        data.extend_from_slice(&(traits.use_epoch_manager as u8).to_le_bytes());
        data.extend_from_slice(&(traits.has_extension as u8).to_le_bytes());
        data.extend_from_slice(&traits.nonce_or_epoch.to_le_bytes());
        data.extend_from_slice(&traits.series.to_le_bytes());

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }

    fn compute_making_amount(
        &self,
        order: &Order,
        extension: &Extension,
        requested_taking_amount: u128,
        _remaining_making_amount: u128,
        _order_hash: &[u8; 32],
    ) -> Result<u128, LimitOrderError> {
        let making_amount_data = extension.maker_amount_data();

        if making_amount_data.is_empty() {
            // Linear proportion
            if order.taking_amount == 0 {
                return Err(LimitOrderError::SwapWithZeroAmount);
            }
            return Ok((order.making_amount * requested_taking_amount) / order.taking_amount);
        }

        // In a real implementation, we would call an external getter contract
        // For now, return a simplified calculation
        Ok(requested_taking_amount)
    }

    fn compute_taking_amount(
        &self,
        order: &Order,
        extension: &Extension,
        requested_making_amount: u128,
        _remaining_making_amount: u128,
        _order_hash: &[u8; 32],
    ) -> Result<u128, LimitOrderError> {
        let taking_amount_data = extension.taker_amount_data();

        if taking_amount_data.is_empty() {
            // Linear proportion
            if order.making_amount == 0 {
                return Err(LimitOrderError::SwapWithZeroAmount);
            }
            return Ok((order.taking_amount * requested_making_amount) / order.making_amount);
        }

        // In a real implementation, we would call an external getter contract
        // For now, return a simplified calculation
        Ok(requested_making_amount)
    }

    fn validate_order_extension(&self, order: &Order, extension: &Extension) -> Result<bool, LimitOrderError> {
        if order.maker_traits.has_extension() {
            if extension.maker_amount_data().is_empty()
                && extension.taker_amount_data().is_empty()
                && extension.predicate_data().is_empty()
                && extension.permit_data().is_empty()
                && extension.pre_interaction_data().is_empty()
                && extension.post_interaction_data().is_empty()
            {
                return Err(LimitOrderError::MissingOrderExtension);
            }

            // Validate extension hash
            let extension_hash = self.hash_extension(extension);
            let order_salt_lower = order.salt & 0xFFFFFFFFFFFFFFFF;
            let hash_lower = u64::from_le_bytes(extension_hash[0..8].try_into().unwrap());

            if hash_lower != order_salt_lower {
                return Err(LimitOrderError::InvalidExtensionHash);
            }
        } else {
            if !extension.maker_amount_data().is_empty()
                || !extension.taker_amount_data().is_empty()
                || !extension.predicate_data().is_empty()
                || !extension.permit_data().is_empty()
                || !extension.pre_interaction_data().is_empty()
                || !extension.post_interaction_data().is_empty()
            {
                return Err(LimitOrderError::UnexpectedOrderExtension);
            }
        }

        Ok(true)
    }

    fn hash_extension(&self, extension: &Extension) -> [u8; 32] {
        let mut data = Vec::new();
        data.extend_from_slice(extension.maker_amount_data());
        data.extend_from_slice(extension.taker_amount_data());
        data.extend_from_slice(extension.predicate_data());
        data.extend_from_slice(extension.permit_data());
        data.extend_from_slice(extension.pre_interaction_data());
        data.extend_from_slice(extension.post_interaction_data());

        near_sdk::env::keccak256(&data).try_into().unwrap()
    }
}

// External contract trait for fungible token transfers
#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer_from(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: u128,
        memo: Option<String>,
    );
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

    fn create_test_order() -> Order {
        Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: crate::types::MakerTraits::default(),
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

        let contract = OrderLib::new([0u8; 32]);
        assert_eq!(contract.get_domain_separator(), [0u8; 32]);
    }

    #[test]
    fn test_hash_order() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = OrderLib::new([0u8; 32]);
        let order = create_test_order();
        let hash = contract.hash_order(order);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_get_receiver() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = OrderLib::new([0u8; 32]);
        let order = create_test_order();
        let receiver = contract.get_receiver(order);
        assert_eq!(receiver, accounts(1)); // Should return the receiver from the order
    }

    #[test]
    fn test_validate_extension() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = OrderLib::new([0u8; 32]);
        let order = create_test_order();
        let extension = create_test_extension();
        let result = contract.validate_extension(order, extension);
        assert!(result.is_ok());
    }
}
