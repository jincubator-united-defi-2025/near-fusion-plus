// Find all our documentation at https://docs.near.org
use near_sdk::{env, ext_contract, log, near, Abort, AccountId, Gas, NearToken, Promise};

use crate::types::Immutables;
use crate::utils::{hash_secret, validate_after, validate_caller};

const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

// Define the contract structure
#[near(contract_state)]
pub struct BaseEscrow {
    rescue_delay: u64,
    access_token: AccountId,
    factory: AccountId,
}

// Define the default, which automatically initializes the contract
impl Default for BaseEscrow {
    fn default() -> Self {
        Self {
            rescue_delay: 0,
            access_token: AccountId::try_from("test.near".to_string()).unwrap(),
            factory: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

// Implement the contract structure
#[near]
impl BaseEscrow {
    // Public method - initializes the contract with rescue delay and access token
    #[init]
    pub fn new(rescue_delay: u64, access_token: AccountId) -> Self {
        Self {
            rescue_delay,
            access_token,
            factory: env::predecessor_account_id(),
        }
    }

    // Public method - rescues funds after the rescue delay has passed
    #[handle_result]
    pub fn rescue_funds(
        &mut self,
        token: AccountId,
        amount: u128,
        immutables: Immutables,
    ) -> Result<(), Abort> {
        // Validate caller is taker
        validate_caller(&immutables.taker).map_err(|_| Abort)?;

        // Validate time has passed
        validate_after(env::block_timestamp()).map_err(|_| Abort)?;

        // Transfer tokens
        self.uni_transfer(&token, &immutables.taker, amount);

        log!("Funds rescued: token={}, amount={}", token, amount);
        Ok(())
    }

    // Public method - validates access token
    #[handle_result]
    pub fn validate_access_token(&self) -> Result<(), Abort> {
        // In NEAR, we would need to check if the caller has the access token
        // For now, just return success
        Ok(())
    }

    // Public method - validates immutables (can be overridden by derived contracts)
    #[handle_result]
    pub fn validate_immutables(&self, _immutables: &Immutables) -> Result<(), Abort> {
        // Default implementation - derived contracts should override
        Ok(())
    }

    // Public method - validates secret against hashlock
    #[handle_result]
    pub fn validate_secret(&self, secret: &[u8; 32], immutables: &Immutables) -> Result<(), Abort> {
        let secret_hash = hash_secret(secret);
        if secret_hash != immutables.hashlock {
            return Err(Abort);
        }
        Ok(())
    }

    // Internal method - transfers tokens (NEAR or fungible tokens)
    pub fn uni_transfer(&self, token: &AccountId, to: &AccountId, amount: u128) {
        if token.as_str() == "near" {
            // Native NEAR transfer
            self.near_transfer(to, amount);
        } else {
            // Fungible token transfer
            ext_ft::ext(token.clone())
                .with_static_gas(GAS_FOR_FT_TRANSFER)
                .with_attached_deposit(NearToken::from_yoctonear(1))
                .ft_transfer(to.clone(), amount, None);
        }
    }

    // Internal method - transfers native NEAR
    pub fn near_transfer(&self, to: &AccountId, amount: u128) {
        Promise::new(to.clone()).transfer(NearToken::from_yoctonear(amount));
    }

    // Public method - returns the rescue delay
    pub fn get_rescue_delay(&self) -> u64 {
        self.rescue_delay
    }

    // Public method - returns the factory account
    pub fn get_factory(&self) -> AccountId {
        self.factory.clone()
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

    #[test]
    fn test_new() {
        let context = VMContextBuilder::new()
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);

        let contract = BaseEscrow::new(3600, accounts(1));
        assert_eq!(contract.rescue_delay, 3600);
        assert_eq!(contract.access_token, accounts(1));
        assert_eq!(contract.factory, accounts(0));
    }

    #[test]
    fn test_default() {
        let contract = BaseEscrow::default();
        assert_eq!(contract.rescue_delay, 0);
        assert_eq!(
            contract.access_token,
            AccountId::try_from("test.near".to_string()).unwrap()
        );
        assert_eq!(
            contract.factory,
            AccountId::try_from("test.near".to_string()).unwrap()
        );
    }
}
