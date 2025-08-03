# Escrow System

## Overview

The Escrow System provides a comprehensive framework for managing cross-chain atomic swaps through multiple specialized contracts. It includes factories for creating escrows, source and destination escrow contracts, and advanced features like Merkle validation.

## Contract Architecture

### 1. Base Escrow Factory

The most advanced factory contract with Merkle validation support.

#### Contract Structure

```rust
pub struct BaseEscrowFactory {
    limit_order_protocol: AccountId,
    fee_token: AccountId,
    access_token: AccountId,
    owner: AccountId,
    rescue_delay_src: u32,
    rescue_delay_dst: u32,
    escrow_src_implementation: AccountId,
    escrow_dst_implementation: AccountId,
    proxy_src_bytecode_hash: [u8; 32],
    proxy_dst_bytecode_hash: [u8; 32],
    validated_data: UnorderedMap<[u8; 32], ValidationData>,
}
```

#### Key Features

- **Merkle Validation**: Advanced proof validation for complex orders
- **Multiple Fills**: Support for orders with multiple partial fills
- **Proxy Deployment**: Efficient escrow deployment through proxies
- **Validation Tracking**: Comprehensive validation data storage

#### Post Interaction

```rust
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
) -> Result<(), FactoryError>
```

**Process:**

1. Validate order structure
2. Parse extra data arguments
3. Calculate hashlock based on maker traits
4. Handle multiple fills with Merkle validation
5. Create source escrow with immutables

#### Taker Interaction

```rust
pub fn taker_interaction(
    &mut self,
    order: Order,
    extension: Vec<u8>,
    order_hash: [u8; 32],
    taker: AccountId,
    making_amount: u128,
    taking_amount: u128,
    remaining_making_amount: u128,
    extra_data: Vec<u8>,
) -> Result<(), FactoryError>
```

**Process:**

1. Extract proof data from extension
2. Validate Merkle proof
3. Create destination escrow
4. Store validation data

### 2. Standard Escrow Factory

Basic escrow factory for simple cross-chain swaps.

#### Contract Structure

```rust
pub struct EscrowFactory {
    limit_order_protocol: AccountId,
    fee_token: AccountId,
    access_token: AccountId,
    owner: AccountId,
    rescue_delay_src: u32,
    rescue_delay_dst: u32,
    escrow_src_implementation: AccountId,
    escrow_dst_implementation: AccountId,
    proxy_src_bytecode_hash: [u8; 32],
    proxy_dst_bytecode_hash: [u8; 32],
    validated_data: UnorderedMap<[u8; 32], ValidationData>,
}
```

#### Key Features

- **Simple Escrow Creation**: Basic escrow deployment
- **Standard Validation**: Standard order validation
- **Proxy Support**: Efficient deployment through proxies
- **Validation Tracking**: Basic validation data storage

### 3. Source Escrow Contract

Manages funds on the source chain during cross-chain swaps.

#### Contract Structure

```rust
pub struct EscrowSrc {
    rescue_delay: u32,
    access_token: AccountId,
    owner: AccountId,
}
```

#### Core Functions

##### Withdrawal Functions

```rust
pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
pub fn withdraw_to(&mut self, secret: [u8; 32], target: AccountId, immutables: Immutables) -> Result<(), EscrowError>
pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Features:**

- **Taker-only Withdrawal**: Only taker can withdraw with secret
- **Target Withdrawal**: Withdraw to specific target address
- **Public Withdrawal**: Access token holder can withdraw
- **Timelock Validation**: Time-based security checks

##### Cancellation Functions

```rust
pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
pub fn public_cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Features:**

- **Taker Cancellation**: Taker can cancel after timelock
- **Public Cancellation**: Access token holder can cancel
- **Timelock Validation**: Ensures sufficient time has passed

### 4. Destination Escrow Contract

Manages funds on the destination chain during cross-chain swaps.

#### Contract Structure

```rust
pub struct EscrowDst {
    rescue_delay: u32,
    access_token: AccountId,
    owner: AccountId,
}
```

#### Core Functions

##### Withdrawal Functions

```rust
pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Features:**

- **Secret-based Withdrawal**: Withdraw using revealed secret
- **Public Withdrawal**: Access token holder withdrawal
- **Timelock Validation**: Time-based security checks

##### Cancellation Functions

```rust
pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
pub fn public_cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Features:**

- **Taker Cancellation**: Taker can cancel after timelock
- **Public Cancellation**: Access token holder can cancel
- **Immutables Validation**: Validate immutable parameters

## Data Structures

### Immutables

```rust
pub struct Immutables {
    pub taker: AccountId,           // Taker address
    pub hashlock: [u8; 32],        // Hash of the secret
    pub timelocks: Timelocks,       // Time-based constraints
    pub maker_asset: AccountId,     // Asset being sold
    pub taker_asset: AccountId,     // Asset being bought
    pub maker_amount: u128,         // Amount being sold
    pub taker_amount: u128,         // Amount being bought
}
```

### Timelocks

```rust
pub struct Timelocks {
    pub src_withdrawal: u64,        // Source withdrawal time
    pub src_cancellation: u64,      // Source cancellation time
    pub dst_withdrawal: u64,        // Destination withdrawal time
    pub dst_cancellation: u64,      // Destination cancellation time
}
```

### Validation Data

```rust
pub struct ValidationData {
    pub leaf: [u8; 32],            // Merkle tree leaf
    pub index: u64,                 // Leaf index
}
```

## Security Features

### Time-based Security

#### Stage Validation

```rust
pub enum Stage {
    SrcWithdrawal,
    SrcPublicWithdrawal,
    SrcCancellation,
    DstWithdrawal,
    DstPublicWithdrawal,
    DstCancellation,
}
```

- **Stage-based Access**: Different functions available at different stages
- **Timelock Protection**: Prevents premature or delayed operations
- **Graceful Degradation**: Fallback mechanisms for stuck funds

### Access Control

#### Role-based Access

- **Taker Functions**: Only taker can call certain functions
- **Access Token Functions**: Require access token ownership
- **Public Functions**: Available to authorized parties
- **Owner Functions**: Only contract owner can call

#### Validation Functions

```rust
fn validate_taker(caller: &AccountId, immutables: &Immutables) -> Result<(), EscrowError>
fn validate_access_token_holder(caller: &AccountId) -> Result<(), EscrowError>
fn validate_immutables(immutables: &Immutables) -> bool
```

### Error Handling

```rust
pub enum EscrowError {
    TimelockNotReached,
    TimelockExpired,
    InvalidImmutables,
    InvalidSecret,
    UnauthorizedCaller,
    TransferFailed,
}
```

## Integration Points

### External Dependencies

- **Limit Order Protocol**: Triggers escrow creation
- **Fungible Token Standard**: Token transfers
- **NEAR Protocol**: Base blockchain infrastructure

### Internal Dependencies

- **Factory Contracts**: Create and manage escrows
- **Base Escrow**: Foundation for cross-chain swaps
- **Merkle Storage Invalidator**: Proof validation

## Usage Examples

### Creating Source Escrow

```rust
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

### Withdrawing from Source Escrow

```rust
let secret = [1u8; 32];
let result = escrow_src.withdraw(secret, immutables);
```

### Cancelling Destination Escrow

```rust
let result = escrow_dst.cancel(immutables);
```

### Public Withdrawal

```rust
let result = escrow_src.public_withdraw(secret, immutables);
```

## Gas Optimization

### Efficient Storage

- **UnorderedMap**: Gas-efficient storage for validation data
- **Minimal State**: Only essential data stored in contracts
- **Lazy Loading**: On-demand data loading patterns

### Gas Costs

- **Escrow Creation**: ~50,000 gas
- **Withdrawal**: ~30,000 gas
- **Cancellation**: ~25,000 gas
- **Public Operations**: ~35,000 gas

## Testing

### Unit Tests

- Escrow creation and initialization
- Withdrawal and cancellation mechanisms
- Timelock validation
- Access control validation

### Integration Tests

- Cross-contract interactions
- End-to-end swap scenarios
- Error condition handling
- Gas consumption validation

## Monitoring

### Key Events

- `EscrowCreated`: Escrow creation events
- `FundsWithdrawn`: Fund withdrawal events
- `EscrowCancelled`: Escrow cancellation events
- `PublicOperation`: Public operation events

### Metrics

- Escrow creation rate
- Withdrawal success rate
- Cancellation frequency
- Gas consumption patterns
- Cross-chain activity

## Security Considerations

### Best Practices

1. **Timelock Management**: Proper timelock implementation
2. **Access Control**: Strict role-based access control
3. **Secret Management**: Secure secret handling
4. **Error Handling**: Comprehensive error management

### Risk Mitigation

- **Rescue Delays**: Prevent immediate fund access
- **Access Tokens**: Restrict sensitive operations
- **Timelock Validation**: Ensure proper timing
- **Caller Validation**: Verify authorized access
