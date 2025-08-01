use crate::types::Immutables;
use crate::utils::{hash_secret, validate_after, validate_caller};
use near_sdk::{env, ext_contract, log, near, Abort, AccountId, Gas, NearToken, Promise};

const GAS_FOR_FT_TRANSFER: Gas = Gas::from_tgas(10);

#[near(contract_state)]
pub struct BaseEscrow {
    pub rescue_delay: u64,
    pub access_token: AccountId,
    pub factory: AccountId,
}

impl Default for BaseEscrow {
    fn default() -> Self {
        Self {
            rescue_delay: 0,
            access_token: AccountId::try_from("test.near".to_string()).unwrap(),
            factory: AccountId::try_from("test.near".to_string()).unwrap(),
        }
    }
}

#[near]
impl BaseEscrow {
    #[init]
    pub fn new(rescue_delay: u64, access_token: AccountId) -> Self {
        Self {
            rescue_delay,
            access_token,
            factory: env::predecessor_account_id(),
        }
    }

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

    #[handle_result]
    pub fn validate_access_token(&self) -> Result<(), Abort> {
        // In NEAR, we would need to check if the caller has the access token
        // For now, just return success
        Ok(())
    }

    #[handle_result]
    pub fn validate_immutables(&self, _immutables: &Immutables) -> Result<(), Abort> {
        // Default implementation - derived contracts should override
        Ok(())
    }

    #[handle_result]
    pub fn validate_secret(&self, secret: &[u8; 32], immutables: &Immutables) -> Result<(), Abort> {
        let secret_hash = hash_secret(secret);
        if secret_hash != immutables.hashlock {
            return Err(Abort);
        }
        Ok(())
    }

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

    pub fn near_transfer(&self, to: &AccountId, amount: u128) {
        Promise::new(to.clone()).transfer(NearToken::from_yoctonear(amount));
    }

    pub fn get_rescue_delay(&self) -> u64 {
        self.rescue_delay
    }

    pub fn get_factory(&self) -> AccountId {
        self.factory.clone()
    }
}

#[ext_contract(ext_ft)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: u128, memo: Option<String>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_new() {
        let context = VMContextBuilder::new()
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);

        let rescue_delay = 1000;
        let access_token = accounts(2);
        let contract = BaseEscrow::new(rescue_delay, access_token.clone());

        assert_eq!(contract.get_rescue_delay(), rescue_delay);
        assert_eq!(contract.get_factory(), accounts(1));
    }

    #[test]
    fn test_default() {
        let contract = BaseEscrow::default();
        assert_eq!(contract.get_rescue_delay(), 0);
    }
}
