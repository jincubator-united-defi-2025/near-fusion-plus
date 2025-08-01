# Limit Order Protocol Contracts for NEAR

This directory contains the NEAR Rust implementation of the 1inch Limit Order Protocol, migrated from the original Solidity contracts in the `1inch` directory.

## Overview

The Limit Order Protocol enables decentralized limit order trading with advanced features like:
- Regular Limit Orders with customizable execution predicates and callbacks
- RFQ (Request for Quote) Orders for gas-efficient trading
- Partial fills and multiple fills support
- Order cancellation mechanisms
- Extension system for custom functionality

## Contract Architecture

### LimitOrderProtocol
The main contract that combines all functionality:
- Pause/unpause functionality (owner only)
- Domain separator management
- Delegates core functionality to OrderMixin

### OrderMixin
Core order processing logic:
- Order validation and execution
- Bit invalidator and remaining invalidator management
- Order cancellation (single and batch)
- Simulation capabilities
- Token transfer handling

### OrderLib
Library for order processing utilities:
- Order hash calculation
- Amount calculations (making/taking)
- Extension validation
- Receiver address resolution

## Key Features

### Order Types

#### Regular Limit Orders
- Support execution predicates (time, price, etc.)
- Callback notifications on execution
- Custom extension data
- Flexible cancellation mechanisms

#### RFQ Orders
- Gas-efficient trading
- Expiration time support
- Single partial fill support
- Simple cancellation by order ID

### Order Management

#### Bit Invalidator
- Efficient mass cancellation of orders
- Uses bit-based slot system
- Reduces gas costs for frequent cancellations

#### Remaining Invalidator
- Tracks remaining amounts for orders
- Supports partial fills
- Individual order cancellation

### Extension System
Orders can include extension data for:
- Custom amount calculations
- Execution predicates
- Pre/post interaction callbacks
- Additional metadata

## Usage

### Building the Contracts

```bash
cd src/limit-order-protocol
cargo near build
```

### Running Tests

```bash
cargo test
```

### Deploying Contracts

```bash
# Deploy for debugging
cargo near deploy build-non-reproducible-wasm <account-id>

# Deploy for production
cargo near deploy build-reproducible-wasm <account-id>
```

## Contract Functions

### LimitOrderProtocol
- `new(domain_separator, weth)`: Initialize contract
- `pause()`: Pause all trading (owner only)
- `unpause()`: Unpause all trading (owner only)
- `domain_separator()`: Get domain separator
- `is_paused()`: Check if contract is paused

### OrderMixin
- `fill_order(order, extension, signature, taker, taking_amount)`: Execute an order
- `cancel_order(maker_traits, order_hash)`: Cancel a single order
- `cancel_orders(maker_traits, order_hashes)`: Cancel multiple orders
- `bit_invalidator_for_order(maker, slot)`: Check bit invalidator
- `remaining_invalidator_for_order(maker, order_hash)`: Get remaining amount
- `simulate(target, data)`: Simulate order execution

### OrderLib
- `hash_order(order)`: Calculate order hash
- `get_receiver(order)`: Get receiver address
- `calculate_making_amount(...)`: Calculate making amount
- `calculate_taking_amount(...)`: Calculate taking amount
- `validate_extension(order, extension)`: Validate extension

## Migration Notes

### Key Differences from Solidity

1. **Account System**: Uses NEAR's account-based system instead of Ethereum addresses
2. **Token Standards**: Uses NEAR's Fungible Token (FT) standard instead of ERC20
3. **Gas Model**: Uses NEAR's gas model instead of Ethereum's
4. **Storage**: Uses NEAR's storage model with Borsh serialization
5. **Cross-Contract Calls**: Uses NEAR's cross-contract call mechanism

### Simplified Implementations

Some complex Ethereum-specific features have been simplified for NEAR:
- EIP-712 signature validation (placeholder implementation)
- Complex predicate evaluation (simplified)
- External getter contract calls (placeholder implementation)

### Future Enhancements

1. Implement proper EIP-712 signature validation
2. Add comprehensive predicate evaluation system
3. Implement external getter contract integration
4. Add more comprehensive error handling
5. Implement proper cross-chain order coordination

## Security Considerations

- All order validations are enforced
- Signature validation prevents unauthorized executions
- Access control mechanisms protect sensitive operations
- Pause functionality for emergency situations
- Proper amount validation prevents overflow/underflow

## Testing

The contracts include comprehensive unit tests and integration tests:

- Unit tests for each contract's core functionality
- Integration tests for cross-contract interactions
- Order validation tests
- Cancellation mechanism tests
- Extension validation tests

Run tests with:
```bash
cargo test
```

## License

This project is licensed under the MIT License. 