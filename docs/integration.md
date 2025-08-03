# Integration Guide

## Overview

This guide provides comprehensive instructions for integrating NEAR Fusion+ into your applications. It covers everything from basic setup to advanced usage patterns.

## Prerequisites

### Development Environment

- **Rust**: Latest stable version (see `rust-toolchain.toml`)
- **cargo-near**: NEAR smart contract development toolkit
- **NEAR CLI**: Command line interface for NEAR
- **Node.js**: For frontend integration (optional)

### Installation

```bash
# Install cargo-near
cargo install cargo-near

# Install NEAR CLI
npm install -g near-cli

# Clone the repository
git clone <repository-url>
cd near-fusion-plus
```

## Basic Setup

### 1. Build Contracts

```bash
# Build all contracts
cargo near build

# Build specific contract
cd src/limit-order-protocol
cargo near build
```

### 2. Deploy Contracts

```bash
# Deploy for testing
cargo near deploy build-non-reproducible-wasm <account-id>

# Deploy for production
cargo near deploy build-reproducible-wasm <account-id>
```

### 3. Initialize Contracts

```rust
// Initialize Limit Order Protocol
let domain_separator = [0u8; 32]; // Set appropriate domain separator
let weth = "weth.near".parse().unwrap();
let limit_order = LimitOrderProtocol::new(domain_separator, weth);

// Initialize Escrow Factory
let factory = BaseEscrowFactory::new(
    "limit-order.near".parse().unwrap(),
    "fee-token.near".parse().unwrap(),
    "access-token.near".parse().unwrap(),
    3600, // 1 hour rescue delay
    3600, // 1 hour rescue delay
    "escrow-src.near".parse().unwrap(),
    "escrow-dst.near".parse().unwrap(),
);
```

## Core Integration Patterns

### 1. Limit Order Creation

```rust
// Create an order
let order = Order {
    salt: 12345,
    maker: "alice.near".parse().unwrap(),
    receiver: "alice.near".parse().unwrap(),
    maker_asset: "usdc.near".parse().unwrap(),
    taker_asset: "near".parse().unwrap(),
    making_amount: 1000_000_000, // 1000 USDC
    taking_amount: 5_000_000_000_000_000_000, // 5 NEAR
    maker_traits: MakerTraits::default(),
};

// Sign the order (client-side)
let signature = sign_order(&order, private_key);

// Submit order to protocol
let result = limit_order_protocol.create_order(order, signature);
```

### 2. Order Execution

```rust
// Fill an order
let taking_amount = 2_500_000_000_000_000_000; // 2.5 NEAR
let extension = Extension::default();

let result = limit_order_protocol.fill_order(
    order,
    extension,
    signature,
    "bob.near".parse().unwrap(),
    taking_amount,
);
```

### 3. Cross-Chain Swap

```rust
// Create cross-chain swap
let immutables = Immutables {
    taker: "bob.near".parse().unwrap(),
    hashlock: [1u8; 32], // Hash of secret
    timelocks: Timelocks {
        src_withdrawal: env::block_timestamp() + 3600,
        src_cancellation: env::block_timestamp() + 7200,
        dst_withdrawal: env::block_timestamp() + 3600,
        dst_cancellation: env::block_timestamp() + 7200,
    },
    maker_asset: "usdc.near".parse().unwrap(),
    taker_asset: "eth.near".parse().unwrap(),
    maker_amount: 1000_000_000, // 1000 USDC
    taker_amount: 1_000_000_000_000_000_000, // 1 ETH
};

// Create source escrow
let result = escrow_factory.post_interaction(
    order,
    extension,
    order_hash,
    "bob.near".parse().unwrap(),
    making_amount,
    taking_amount,
    remaining_making_amount,
    extra_data,
);
```

### 4. Fee Collection

```rust
// Configure fee structure
let fee_config = FeeConfig {
    fee_recipient: "fee-recipient.near".parse().unwrap(),
    fee_amount: 1000_000_000, // 1000 tokens
    fee_token: "usdc.near".parse().unwrap(),
    fee_percentage: 30, // 0.3%
};

// Fee collection happens automatically during order execution
```

## Advanced Integration

### 1. Merkle Proof Validation

```rust
// Generate Merkle proof (client-side)
let secrets = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
let target_index = 1;
let proof = generate_merkle_proof(&secrets, target_index);

// Validate proof on-chain
let taker_data = TakerData {
    secret_hash: [1u8; 32],
    idx: target_index,
    proof: proof,
};

let result = merkle_invalidator.taker_interaction(
    order,
    extension,
    order_hash,
    "taker.near".parse().unwrap(),
    making_amount,
    taking_amount,
    remaining_making_amount,
    extra_data,
);
```

### 2. Batch Operations

```rust
// Cancel multiple orders
let maker_traits = vec![MakerTraits::default(); 3];
let order_hashes = vec![[1u8; 32], [2u8; 32], [3u8; 32]];

let result = limit_order_protocol.cancel_orders(maker_traits, order_hashes);
```

### 3. Extension System

```rust
// Create custom extension
let extension = Extension {
    maker_amount_data: vec![1, 2, 3],
    taker_amount_data: vec![4, 5, 6],
    predicate_data: vec![7, 8, 9],
    permit_data: vec![10, 11, 12],
    pre_interaction_data: vec![13, 14, 15],
    post_interaction_data: vec![16, 17, 18],
};

// Use extension in order execution
let result = limit_order_protocol.fill_order(
    order,
    extension,
    signature,
    taker,
    taking_amount,
);
```

## Frontend Integration

### 1. JavaScript/TypeScript

```typescript
import { connect, keyStores, WalletConnection } from "near-api-js";

// Connect to NEAR
const near = await connect({
  networkId: "testnet",
  keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  nodeUrl: "https://rpc.testnet.near.org",
});

const wallet = new WalletConnection(near, "my-app");

// Call contract methods
const contract = new Contract(wallet.account(), "limit-order.near", {
  viewMethods: ["get_owner", "is_paused"],
  changeMethods: ["fill_order", "cancel_order"],
});

// Execute order
const result = await contract.fill_order({
  order: orderData,
  extension: extensionData,
  signature: signatureData,
  taker: "bob.near",
  taking_amount: "2500000000000000000",
});
```

### 2. React Integration

```typescript
import React, { useEffect, useState } from "react";
import { useWallet } from "./hooks/useWallet";

function OrderForm() {
  const { wallet, contract } = useWallet();
  const [order, setOrder] = useState(null);

  const submitOrder = async () => {
    try {
      const result = await contract.create_order({
        order: order,
        signature: await signOrder(order),
      });
      console.log("Order submitted:", result);
    } catch (error) {
      console.error("Error submitting order:", error);
    }
  };

  return <form onSubmit={submitOrder}>{/* Order form fields */}</form>;
}
```

## Error Handling

### 1. Contract Errors

```rust
// Handle contract errors
match result {
    Ok(_) => println!("Operation successful"),
    Err(LimitOrderError::InvalidatedOrder) => println!("Order is invalidated"),
    Err(LimitOrderError::TakingAmountExceeded) => println!("Amount too high"),
    Err(LimitOrderError::InvalidSignature) => println!("Invalid signature"),
    Err(e) => println!("Other error: {:?}", e),
}
```

### 2. Frontend Error Handling

```typescript
try {
  const result = await contract.fill_order(orderData);
  console.log("Success:", result);
} catch (error) {
  if (error.message.includes("InvalidatedOrder")) {
    console.error("Order is invalidated");
  } else if (error.message.includes("TakingAmountExceeded")) {
    console.error("Amount too high");
  } else {
    console.error("Unknown error:", error);
  }
}
```

## Testing

### 1. Unit Tests

```bash
# Run unit tests
cargo test

# Run specific test
cargo test test_fill_order
```

### 2. Integration Tests

```bash
# Run integration tests
cd integration-tests
cargo test
```

### 3. Frontend Tests

```bash
# Run frontend tests
npm test

# Run specific test
npm test -- --testNamePattern="OrderForm"
```

## Monitoring and Analytics

### 1. Event Tracking

```typescript
// Track order events
contract.on("OrderCreated", (event) => {
  analytics.track("order_created", {
    maker: event.maker,
    amount: event.amount,
    asset: event.asset,
  });
});

contract.on("OrderFilled", (event) => {
  analytics.track("order_filled", {
    taker: event.taker,
    amount: event.amount,
    gas_used: event.gas_used,
  });
});
```

### 2. Performance Monitoring

```typescript
// Monitor gas usage
const startGas = await contract.get_gas_used();
const result = await contract.fill_order(orderData);
const endGas = await contract.get_gas_used();
const gasUsed = endGas - startGas;

console.log(`Gas used: ${gasUsed}`);
```

## Security Best Practices

### 1. Signature Validation

```typescript
// Always validate signatures client-side
const isValidSignature = await validateSignature(order, signature, publicKey);
if (!isValidSignature) {
  throw new Error("Invalid signature");
}
```

### 2. Amount Validation

```typescript
// Validate amounts before submission
const isValidAmount = validateAmount(amount, minAmount, maxAmount);
if (!isValidAmount) {
  throw new Error("Invalid amount");
}
```

### 3. Error Handling

```typescript
// Comprehensive error handling
try {
  const result = await contract.operation(data);
  return result;
} catch (error) {
  logError(error);
  showUserFriendlyError(error);
  throw error;
}
```

## Deployment Checklist

### 1. Pre-deployment

- [ ] All tests passing
- [ ] Gas optimization completed
- [ ] Security audit completed
- [ ] Documentation updated

### 2. Deployment

- [ ] Deploy contracts to testnet
- [ ] Run integration tests
- [ ] Deploy to mainnet
- [ ] Verify contract addresses

### 3. Post-deployment

- [ ] Monitor contract performance
- [ ] Track user interactions
- [ ] Monitor gas usage
- [ ] Update documentation

## Support and Resources

### 1. Documentation

- [Architecture Overview](./architecture.md)
- [Contract Documentation](./contracts/)
- [API Reference](./api-reference.md)

### 2. Community

- [NEAR Discord](https://near.chat)
- [NEAR Telegram](https://t.me/neardev)
- [GitHub Issues](https://github.com/your-repo/issues)

### 3. Tools

- [NEAR Explorer](https://explorer.near.org)
- [NEAR CLI](https://near.cli.rs)
- [cargo-near](https://github.com/near/cargo-near)
