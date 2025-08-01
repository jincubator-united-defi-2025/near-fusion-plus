// Find all our documentation at https://docs.near.org
use near_sdk::{
    env, log, near, AccountId, NearToken, Promise, serde_json,
};
use crate::types::{Order, FeeTakerError};
use crate::utils::{validate_limit_order_protocol, parse_fee_config, transfer_tokens_with_fee, validate_fee_config, is_fee_applicable};

/// Fee Taker extension contract for limit order protocol
/// Handles fee collection for limit orders
#[near(contract_state)]
pub struct FeeTaker {
    limit_order_protocol: AccountId,
    access_token: AccountId,
    weth: AccountId,
    owner: AccountId,
}

impl Default for FeeTaker {
    fn default() -> Self {
        Self {
            limit_order_protocol: AccountId::try_from("test.near".to_string()).unwrap(),
            access_token: AccountId::try_from("test.near".to_string()).unwrap(),
            weth: AccountId::try_from("test.near".to_string()).unwrap(),
            owner: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

#[near]
impl FeeTaker {
    /// Initialize the contract
    #[init]
    pub fn new(
        limit_order_protocol: AccountId,
        access_token: AccountId,
        weth: AccountId,
    ) -> Self {
        Self {
            limit_order_protocol,
            access_token,
            weth,
            owner: env::predecessor_account_id(),
        }
    }

    /// Post interaction for fee collection
    #[handle_result]
    pub fn post_interaction(
        &mut self,
        order: Order,
        extension: Vec<u8>,
        order_hash: [u8; 32],
        taker: AccountId,
        making_amount: u128,
        taking_amount: u128,
        remaining_making_amount: u128,
        extra_data: Vec<u8>,
    ) -> Result<(), FeeTakerError> {
        // Only limit order protocol can call this
        validate_limit_order_protocol(&env::predecessor_account_id(), &self.limit_order_protocol)?;

        // Parse fee configuration from extension
        let fee_config = parse_fee_config(&extension)?;

        // Validate fee configuration
        validate_fee_config(&fee_config, &order.receiver)?;

        // Check if fee is applicable
        if is_fee_applicable(&fee_config) {
            // Transfer tokens with fee
            transfer_tokens_with_fee(
                &order.taker_asset,
                &taker,
                &order.receiver,
                taking_amount,
                &fee_config,
            )?;
        } else {
            // Regular transfer without fee
            Promise::new(order.receiver.clone()).function_call(
                "ft_transfer".to_string(),
                serde_json::to_vec(&serde_json::json!({
                    "receiver_id": order.receiver,
                    "amount": taking_amount.to_string(),
                    "msg": ""
                })).unwrap(),
                NearToken::from_yoctonear(1),
                near_sdk::Gas::from_tgas(10),
            );
        }

        log!("Fee collected for order: order_hash={:?}, taker={}, amount={}", order_hash, taker, taking_amount);

        Ok(())
    }

    /// Rescue funds accidentally sent to the contract
    #[handle_result]
    pub fn rescue_funds(
        &mut self,
        token: AccountId,
        amount: u128,
    ) -> Result<(), FeeTakerError> {
        // Only owner can rescue funds
        if env::predecessor_account_id() != self.owner {
            return Err(FeeTakerError::OnlyOwner);
        }

        // Transfer tokens to owner
        Promise::new(self.owner.clone()).function_call(
            "ft_transfer".to_string(),
            serde_json::to_vec(&serde_json::json!({
                "receiver_id": self.owner,
                "amount": amount.to_string(),
                "msg": ""
            })).unwrap(),
            NearToken::from_yoctonear(1),
            near_sdk::Gas::from_tgas(10),
        );

        log!("Funds rescued: token={}, amount={}", token, amount);

        Ok(())
    }

    /// Get limit order protocol address
    pub fn get_limit_order_protocol(&self) -> AccountId {
        self.limit_order_protocol.clone()
    }

    /// Get access token address
    pub fn get_access_token(&self) -> AccountId {
        self.access_token.clone()
    }

    /// Get WETH address
    pub fn get_weth(&self) -> AccountId {
        self.weth.clone()
    }

    /// Get owner
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
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

    #[test]
    fn test_new() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = FeeTaker::new(
            accounts(1),
            accounts(2),
            accounts(3),
        );
        assert_eq!(contract.get_limit_order_protocol(), accounts(1));
        assert_eq!(contract.get_access_token(), accounts(2));
        assert_eq!(contract.get_weth(), accounts(3));
        assert_eq!(contract.get_owner(), accounts(0));
    }

    #[test]
    fn test_default() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = FeeTaker::default();
        assert_eq!(contract.get_limit_order_protocol(), accounts(0));
        assert_eq!(contract.get_access_token(), accounts(0));
        assert_eq!(contract.get_weth(), accounts(0));
        assert_eq!(contract.get_owner(), accounts(0));
    }
} 