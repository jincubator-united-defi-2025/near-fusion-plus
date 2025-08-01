use cross_chain_swap::BaseEscrow;
use escrow_dst::EscrowDst;
use escrow_factory::EscrowFactory;
use escrow_src::EscrowSrc;
use near_sdk::{
    test_utils::{accounts, VMContextBuilder},
    testing_env, AccountId, NearToken,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .predecessor_account_id(predecessor_account_id)
            .attached_deposit(NearToken::from_yoctonear(1));
        builder
    }

    fn create_test_secret() -> (Vec<u8>, Vec<u8>) {
        let secret = vec![1u8; 32];
        let hash = vec![2u8; 32]; // Simplified hash
        (secret, hash)
    }

    #[test]
    fn test_base_escrow_creation() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let escrow = BaseEscrow::new(3600, accounts(1));
        assert_eq!(escrow.get_rescue_delay(), 3600);
        // BaseEscrow doesn't have get_access_token method, so we skip that test
    }

    #[test]
    fn test_escrow_src_creation() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let escrow = EscrowSrc::new(3600, accounts(1));
        assert_eq!(escrow.get_rescue_delay(), 3600);
        assert_eq!(escrow.get_access_token(), accounts(1));
    }

    #[test]
    fn test_escrow_dst_creation() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let escrow = EscrowDst::new(3600, accounts(1));
        assert_eq!(escrow.get_rescue_delay(), 3600);
        assert_eq!(escrow.get_access_token(), accounts(1));
    }

    #[test]
    fn test_escrow_factory_creation() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let factory = EscrowFactory::new(
            accounts(1), // limit order protocol
            accounts(2), // fee token
            accounts(3), // access token
            3600,        // rescue delay src
            3600,        // rescue delay dst
        );
        assert_eq!(factory.get_owner(), accounts(0));
    }

    #[test]
    fn test_cross_chain_swap_flow() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        // Create factory
        let _factory = EscrowFactory::new(
            accounts(1), // limit order protocol
            accounts(2), // fee token
            accounts(3), // access token
            3600,        // rescue delay src
            3600,        // rescue delay dst
        );

        // Create source escrow
        let src_escrow = EscrowSrc::new(3600, accounts(3));

        // Create destination escrow
        let dst_escrow = EscrowDst::new(3600, accounts(3));

        // Verify escrows are created correctly
        assert_eq!(src_escrow.get_rescue_delay(), 3600);
        assert_eq!(src_escrow.get_access_token(), accounts(3));

        assert_eq!(dst_escrow.get_rescue_delay(), 3600);
        assert_eq!(dst_escrow.get_access_token(), accounts(3));
    }

    #[test]
    fn test_secret_generation() {
        let (_secret, hash) = create_test_secret();
        assert_eq!(hash.len(), 32);
    }
}
