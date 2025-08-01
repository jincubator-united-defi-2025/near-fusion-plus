# EscrowFactory Contract

Escrow Factory contract for cross-chain atomic swap on NEAR Protocol.

## Overview

This contract implements the factory functionality for creating escrow contracts for cross-chain atomic swaps. It manages the creation of both source and destination escrow contracts and handles the coordination between them.

## Features

- **Escrow Creation**: Create source and destination escrow contracts
- **Order Management**: Validate and process limit orders
- **Cross-Chain Coordination**: Handle atomic swap coordination
- **Access Control**: Restrict operations to authorized parties
- **Timelock Management**: Configure rescue delays for different chains

## Contract Functions

### Core Functions

- `post_interaction(order, extension, order_hash, taker, making_amount, taking_amount, remaining_making_amount, extra_data)` - Create source escrow after order execution
- `create_dst_escrow(order, extension, order_hash, taker, making_amount, taking_amount, remaining_making_amount, extra_data)` - Create destination escrow

### View Functions

- `get_limit_order_protocol()` - Get limit order protocol address
- `get_fee_token()` - Get fee token address
- `get_access_token()` - Get access token address
- `get_owner()` - Get contract owner
- `get_rescue_delay_src()` - Get rescue delay for source chain
- `get_rescue_delay_dst()` - Get rescue delay for destination chain
- `get_escrow_src_implementation()` - Get source escrow implementation
- `get_escrow_dst_implementation()` - Get destination escrow implementation

## Architecture

The factory contract coordinates between:

1. **Limit Order Protocol**: Receives order execution calls
2. **Source Escrow**: Locks funds on the source chain
3. **Destination Escrow**: Locks funds on the destination chain
4. **Taker**: Executes the atomic swap

## Usage

### Deployment

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown

# Deploy to NEAR
near deploy <account-id> target/wasm32-unknown-unknown/release/escrow_factory.wasm
```

### Initialization

```bash
# Initialize with required parameters
near call <contract-id> new '{
  "limit_order_protocol": "limit-order.near",
  "fee_token": "fee-token.near",
  "access_token": "access-token.near",
  "rescue_delay_src": 3600,
  "rescue_delay_dst": 3600
}' --account-id <account-id>
```

### Testing

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test
```

## Workflow

1. **Order Creation**: Maker creates a limit order
2. **Order Execution**: Taker executes the order through the limit order protocol
3. **Source Escrow Creation**: Factory creates source escrow to lock funds
4. **Destination Escrow Creation**: Factory creates destination escrow to lock funds
5. **Atomic Swap**: Taker completes the swap using secrets

## Security

- All operations are validated against order parameters
- Access control ensures only authorized parties can perform operations
- Timelocks prevent premature withdrawals or cancellations
- Immutable values ensure consistency across operations

## Migration from Solidity

This contract is migrated from the Solidity `EscrowFactory.sol` contract, adapted for NEAR Protocol's architecture and conventions.

## License

MIT License
