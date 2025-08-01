use limit_order_protocol::utils;
use limit_order_protocol::{Extension, LimitOrderProtocol, MakerTraits, Order, TakerTraits};
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

    #[test]
    fn test_limit_order_protocol_creation() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let protocol = LimitOrderProtocol::new([1u8; 32], accounts(1));
        assert_eq!(protocol.domain_separator(), [1u8; 32]);
        assert_eq!(protocol.get_weth(), accounts(1));
    }

    #[test]
    fn test_limit_order_protocol_pause_unpause() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut protocol = LimitOrderProtocol::new([1u8; 32], accounts(1));
        assert!(!protocol.is_paused());

        protocol.pause();
        assert!(protocol.is_paused());

        protocol.unpause();
        assert!(!protocol.is_paused());
    }

    #[test]
    fn test_order_creation() {
        let order = Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: MakerTraits::default(),
        };

        assert_eq!(order.salt, 12345);
        assert_eq!(order.maker, accounts(0));
        assert_eq!(order.receiver, accounts(1));
        assert_eq!(order.maker_asset, accounts(2));
        assert_eq!(order.taker_asset, accounts(3));
        assert_eq!(order.making_amount, 1000);
        assert_eq!(order.taking_amount, 1000);
    }

    #[test]
    fn test_extension_validation() {
        let order = Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: MakerTraits::default(),
        };

        let extension = Extension::default();
        let result = utils::validate_extension(&order, &extension);
        assert!(result.is_ok());
    }

    #[test]
    fn test_signature_validation() {
        let order = Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: MakerTraits::default(),
        };

        let signature = [0u8; 65];
        let result = utils::validate_signature(
            &order,
            &signature,
            &accounts(0), // domain separator
        );
        // validate_signature returns bool, not Result
        assert!(result);
    }

    #[test]
    fn test_amount_calculations() {
        let order = Order {
            salt: 12345,
            maker: accounts(0),
            receiver: accounts(1),
            maker_asset: accounts(2),
            taker_asset: accounts(3),
            making_amount: 1000,
            taking_amount: 1000,
            maker_traits: MakerTraits::default(),
        };

        let extension = Extension::default();
        let making_amount =
            utils::calculate_making_amount(&order, &extension, order.taking_amount, 0, &[0u8; 32]);
        assert_eq!(making_amount, Ok(order.taking_amount));

        let taking_amount =
            utils::calculate_taking_amount(&order, &extension, order.making_amount, 0, &[0u8; 32]);
        assert_eq!(taking_amount, Ok(order.making_amount));
    }

    #[test]
    fn test_maker_traits() {
        let traits = MakerTraits::default();
        assert!(!traits.has_extension());
    }

    #[test]
    fn test_taker_traits() {
        let traits = TakerTraits::default();
        // TakerTraits doesn't have has_extension method, so we just test creation
        assert_eq!(traits.allow_multiple_fills, false);
        assert_eq!(traits.allow_partial_fill, false);
        assert_eq!(traits.allow_expired_orders, false);
        assert_eq!(traits.allow_private_orders, false);
    }

    #[test]
    fn test_extension_creation() {
        let extension = Extension::default();
        assert_eq!(extension.maker_amount_data, Vec::<u8>::new());
        assert_eq!(extension.taker_amount_data, Vec::<u8>::new());
        assert_eq!(extension.predicate_data, Vec::<u8>::new());
        assert_eq!(extension.permit_data, Vec::<u8>::new());
        assert_eq!(extension.pre_interaction_data, Vec::<u8>::new());
        assert_eq!(extension.post_interaction_data, Vec::<u8>::new());
    }

    #[test]
    fn test_limit_order_protocol_with_domain_separator() {
        let domain_separator = [2u8; 32];
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let protocol = LimitOrderProtocol::new(domain_separator, accounts(1));
        assert_eq!(protocol.domain_separator(), domain_separator);
    }

    #[test]
    fn test_limit_order_protocol_owner_management() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let protocol = LimitOrderProtocol::new([1u8; 32], accounts(1));
        assert_eq!(protocol.get_owner(), accounts(0));
    }
}
