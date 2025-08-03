# Limit Order Protocol

## Overview

The Limit Order Protocol is the core trading engine of NEAR Fusion+, providing advanced limit order functionality with support for partial fills, multiple execution strategies, and sophisticated invalidation mechanisms.

## Contract Structure

### Main Contract: `LimitOrderProtocol`

```rust
pub struct LimitOrderProtocol {
    domain_separator: [u8; 32],
    weth: AccountId,
    bit_invalidator: UnorderedMap<AccountId, BitInvalidatorData>,
    remaining_invalidator: UnorderedMap<(AccountId, [u8; 32]), RemainingInvalidator>,
    paused: bool,
    owner: AccountId,
}
```

### Key Components

- **Domain Separator**: Used for EIP-712 signature verification
- **WETH**: Wrapped NEAR token address
- **Bit Invalidator**: Manages order invalidation through bit masks
- **Remaining Invalidator**: Tracks remaining amounts for partial fills
- **Pause Mechanism**: Emergency pause functionality
- **Owner**: Contract administrator

## Core Functionality

### Order Management

#### Order Structure

```rust
pub struct Order {
    pub salt: u64,                    // Unique order identifier
    pub maker: AccountId,             // Order creator
    pub receiver: AccountId,          // Token receiver
    pub maker_asset: AccountId,       // Asset being sold
    pub taker_asset: AccountId,       // Asset being bought
    pub making_amount: u128,          // Amount being sold
    pub taking_amount: u128,          // Amount being bought
    pub maker_traits: MakerTraits,    // Order customization flags
}
```

#### Maker Traits

```rust
pub struct MakerTraits {
    pub use_bit_invalidator: bool,    // Use bit-based invalidation
    pub use_epoch_manager: bool,      // Use epoch-based management
    pub has_extension: bool,          // Order has extension data
    pub nonce_or_epoch: u64,         // Nonce or epoch value
    pub series: u64,                  // Order series identifier
}
```

### Order Execution

#### Fill Order

```rust
pub fn fill_order(
    &mut self,
    order: Order,
    extension: Extension,
    signature: Vec<u8>,
    taker: AccountId,
    taking_amount: u128,
) -> Result<u128, LimitOrderError>
```

**Process:**

1. Validate order signature
2. Check order invalidation status
3. Calculate making amount based on taking amount
4. Execute token transfers
5. Update order state

#### Extension System

```rust
pub struct Extension {
    pub maker_amount_data: Vec<u8>,   // Maker amount calculation data
    pub taker_amount_data: Vec<u8>,   // Taker amount calculation data
    pub predicate_data: Vec<u8>,      // Execution predicate
    pub permit_data: Vec<u8>,         // Permit2 authorization
    pub pre_interaction_data: Vec<u8>, // Pre-execution hooks
    pub post_interaction_data: Vec<u8>, // Post-execution hooks
}
```

### Invalidation Mechanisms

#### Bit Invalidator

```rust
pub struct BitInvalidatorData {
    pub slots: Vec<u64>,
}
```

- **Purpose**: Efficiently invalidate multiple orders using bit masks
- **Usage**: Set specific bits to invalidate corresponding orders
- **Gas Efficiency**: Batch invalidation reduces gas costs

#### Remaining Invalidator

```rust
pub struct RemainingInvalidator {
    pub remaining: u128,
}
```

- **Purpose**: Track remaining amounts for partial fills
- **Usage**: Update remaining amounts after partial executions
- **Prevention**: Prevents over-filling of orders

### Order Validation

#### Signature Verification

```rust
fn validate_signature(&self, order: &Order, signature: &[u8]) -> bool
```

- **EIP-712**: Uses EIP-712 domain separator for signature verification
- **Maker Verification**: Ensures signature comes from order maker
- **Replay Protection**: Salt prevents signature replay attacks

#### Order State Validation

```rust
fn validate_order_state(&self, order: &Order) -> Result<(), LimitOrderError>
```

- **Invalidation Check**: Verifies order hasn't been invalidated
- **Expiration Check**: Ensures order hasn't expired
- **Amount Validation**: Validates order amounts are within limits

## Security Features

### Access Control

- **Owner Functions**: Only owner can pause/unpause contract
- **Signature Verification**: All orders require valid maker signatures
- **Reentrancy Protection**: Prevents reentrancy attacks

### Pause Mechanism

```rust
pub fn pause(&mut self)     // Pause all trading
pub fn unpause(&mut self)   // Resume trading
pub fn is_paused(&self) -> bool  // Check pause status
```

### Error Handling

```rust
pub enum LimitOrderError {
    InvalidatedOrder,
    TakingAmountExceeded,
    PrivateOrder,
    InvalidSignature,
    OrderExpired,
    // ... additional error types
}
```

## Gas Optimization

### Efficient Storage

- **UnorderedMap**: Gas-efficient storage for invalidation data
- **Batch Operations**: Support for batch order operations
- **Lazy Loading**: On-demand data loading patterns

### Gas Costs

- **Order Creation**: ~50,000 gas
- **Order Execution**: ~100,000 gas (varies with complexity)
- **Invalidation**: ~20,000 gas per order

## Integration Points

### External Contracts

- **Escrow Factory**: Order execution triggers escrow creation
- **Fee Taker**: Fee collection during order execution
- **Token Contracts**: Direct token transfers

### Extension Hooks

- **Pre-Interaction**: Custom logic before order execution
- **Post-Interaction**: Custom logic after order execution
- **Predicate Validation**: Custom execution conditions

## Usage Examples

### Creating an Order

```rust
let order = Order {
    salt: 12345,
    maker: "alice.near".parse().unwrap(),
    receiver: "alice.near".parse().unwrap(),
    maker_asset: "usdc.near".parse().unwrap(),
    taker_asset: "near".parse().unwrap(),
    making_amount: 1000_000_000, // 1000 USDC (6 decimals)
    taking_amount: 5_000_000_000_000_000_000, // 5 NEAR (24 decimals)
    maker_traits: MakerTraits::default(),
};
```

### Filling an Order

```rust
let taking_amount = 2_500_000_000_000_000_000; // 2.5 NEAR
let result = contract.fill_order(
    order,
    extension,
    signature,
    "bob.near".parse().unwrap(),
    taking_amount,
);
```

### Invalidating Orders

```rust
// Bit invalidation
contract.bit_invalidator_for_order("alice.near".parse().unwrap(), 1);

// Remaining invalidation
contract.remaining_invalidator_for_order(
    "alice.near".parse().unwrap(),
    order_hash,
);
```

## Testing

### Unit Tests

- Order creation and validation
- Signature verification
- Invalidation mechanisms
- Error handling

### Integration Tests

- End-to-end order execution
- Cross-contract interactions
- Gas consumption validation

## Monitoring

### Key Events

- `OrderCreated`: Order creation events
- `OrderFilled`: Order execution events
- `OrderCancelled`: Order cancellation events
- `OrderInvalidated`: Order invalidation events

### Metrics

- Total order volume
- Success/failure rates
- Gas consumption patterns
- Most active makers/takers
