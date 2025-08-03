# API Reference

## Overview

This document provides a comprehensive API reference for all NEAR Fusion+ contracts. It includes function signatures, parameters, return values, and usage examples.

## Limit Order Protocol

### Contract: `LimitOrderProtocol`

#### Constructor

```rust
pub fn new(domain_separator: [u8; 32], weth: AccountId) -> Self
```

**Parameters:**

- `domain_separator`: EIP-712 domain separator for signature verification
- `weth`: Wrapped NEAR token address

**Returns:** `LimitOrderProtocol` instance

#### View Functions

##### `domain_separator()`

```rust
pub fn domain_separator(&self) -> [u8; 32]
```

**Returns:** Domain separator for EIP-712 signature verification

##### `is_paused()`

```rust
pub fn is_paused(&self) -> bool
```

**Returns:** `true` if contract is paused, `false` otherwise

##### `get_owner()`

```rust
pub fn get_owner(&self) -> AccountId
```

**Returns:** Contract owner address

##### `get_weth()`

```rust
pub fn get_weth(&self) -> AccountId
```

**Returns:** WETH token address

##### `bit_invalidator_for_order()`

```rust
pub fn bit_invalidator_for_order(&self, maker: AccountId, slot: u64) -> bool
```

**Parameters:**

- `maker`: Maker account address
- `slot`: Slot number to check

**Returns:** `true` if slot is invalidated, `false` otherwise

##### `remaining_invalidator_for_order()`

```rust
pub fn remaining_invalidator_for_order(&self, maker: AccountId, order_hash: [u8; 32]) -> u128
```

**Parameters:**

- `maker`: Maker account address
- `order_hash`: Order hash

**Returns:** Remaining amount for the order

#### State-Changing Functions

##### `pause()`

```rust
pub fn pause(&mut self)
```

**Access:** Owner only
**Effect:** Pauses all trading functionality

##### `unpause()`

```rust
pub fn unpause(&mut self)
```

**Access:** Owner only
**Effect:** Resumes trading functionality

##### `cancel_order()`

```rust
pub fn cancel_order(&mut self, maker_traits: MakerTraits, order_hash: [u8; 32])
```

**Parameters:**

- `maker_traits`: Maker traits for the order
- `order_hash`: Hash of the order to cancel

**Effect:** Cancels the specified order

##### `cancel_orders()`

```rust
pub fn cancel_orders(&mut self, maker_traits: Vec<MakerTraits>, order_hashes: Vec<[u8; 32]>)
```

**Parameters:**

- `maker_traits`: Vector of maker traits
- `order_hashes`: Vector of order hashes

**Effect:** Cancels multiple orders in batch

##### `fill_order()`

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

**Parameters:**

- `order`: Order to fill
- `extension`: Order extension data
- `signature`: Order signature
- `taker`: Taker account address
- `taking_amount`: Amount to take

**Returns:** Making amount that was filled, or error

## Cross-Chain Swap

### Contract: `BaseEscrow`

#### Constructor

```rust
pub fn new(rescue_delay: u64, access_token: AccountId) -> Self
```

**Parameters:**

- `rescue_delay`: Time delay before funds can be rescued
- `access_token`: Token required for access to certain functions

**Returns:** `BaseEscrow` instance

#### View Functions

##### `get_rescue_delay()`

```rust
pub fn get_rescue_delay(&self) -> u64
```

**Returns:** Rescue delay in seconds

##### `get_factory()`

```rust
pub fn get_factory(&self) -> AccountId
```

**Returns:** Factory contract address

#### State-Changing Functions

##### `rescue_funds()`

```rust
pub fn rescue_funds(
    &mut self,
    token: AccountId,
    amount: u128,
    immutables: Immutables,
) -> Result<(), Abort>
```

**Parameters:**

- `token`: Token address to rescue
- `amount`: Amount to rescue
- `immutables`: Immutable parameters

**Returns:** Success or error

##### `validate_access_token()`

```rust
pub fn validate_access_token(&self) -> Result<(), Abort>
```

**Returns:** Success if caller has access token, error otherwise

##### `validate_immutables()`

```rust
pub fn validate_immutables(&self, immutables: &Immutables) -> Result<(), Abort>
```

**Parameters:**

- `immutables`: Immutable parameters to validate

**Returns:** Success if valid, error otherwise

##### `validate_secret()`

```rust
pub fn validate_secret(&self, secret: &[u8; 32], immutables: &Immutables) -> Result<(), Abort>
```

**Parameters:**

- `secret`: Secret to validate
- `immutables`: Immutable parameters

**Returns:** Success if secret matches hashlock, error otherwise

## Escrow System

### Contract: `EscrowSrc`

#### Constructor

```rust
pub fn new(rescue_delay: u32, access_token: AccountId) -> Self
```

**Parameters:**

- `rescue_delay`: Rescue delay in seconds
- `access_token`: Access token address

**Returns:** `EscrowSrc` instance

#### View Functions

##### `get_rescue_delay()`

```rust
pub fn get_rescue_delay(&self) -> u32
```

**Returns:** Rescue delay in seconds

##### `get_access_token()`

```rust
pub fn get_access_token(&self) -> AccountId
```

**Returns:** Access token address

##### `get_owner()`

```rust
pub fn get_owner(&self) -> AccountId
```

**Returns:** Contract owner address

#### State-Changing Functions

##### `withdraw()`

```rust
pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `secret`: Secret for withdrawal
- `immutables`: Immutable parameters

**Returns:** Success or error

##### `withdraw_to()`

```rust
pub fn withdraw_to(&mut self, secret: [u8; 32], target: AccountId, immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `secret`: Secret for withdrawal
- `target`: Target address for withdrawal
- `immutables`: Immutable parameters

**Returns:** Success or error

##### `public_withdraw()`

```rust
pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `secret`: Secret for withdrawal
- `immutables`: Immutable parameters

**Access:** Access token holder only
**Returns:** Success or error

##### `cancel()`

```rust
pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `immutables`: Immutable parameters

**Access:** Taker only
**Returns:** Success or error

##### `public_cancel()`

```rust
pub fn public_cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `immutables`: Immutable parameters

**Access:** Access token holder only
**Returns:** Success or error

### Contract: `EscrowDst`

#### Constructor

```rust
pub fn new(rescue_delay: u32, access_token: AccountId) -> Self
```

**Parameters:**

- `rescue_delay`: Rescue delay in seconds
- `access_token`: Access token address

**Returns:** `EscrowDst` instance

#### View Functions

##### `get_rescue_delay()`

```rust
pub fn get_rescue_delay(&self) -> u32
```

**Returns:** Rescue delay in seconds

##### `get_access_token()`

```rust
pub fn get_access_token(&self) -> AccountId
```

**Returns:** Access token address

##### `get_owner()`

```rust
pub fn get_owner(&self) -> AccountId
```

**Returns:** Contract owner address

#### State-Changing Functions

##### `withdraw()`

```rust
pub fn withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `secret`: Secret for withdrawal
- `immutables`: Immutable parameters

**Access:** Taker only
**Returns:** Success or error

##### `public_withdraw()`

```rust
pub fn public_withdraw(&mut self, secret: [u8; 32], immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `secret`: Secret for withdrawal
- `immutables`: Immutable parameters

**Access:** Access token holder only
**Returns:** Success or error

##### `cancel()`

```rust
pub fn cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `immutables`: Immutable parameters

**Access:** Taker only
**Returns:** Success or error

##### `public_cancel()`

```rust
pub fn public_cancel(&mut self, immutables: Immutables) -> Result<(), EscrowError>
```

**Parameters:**

- `immutables`: Immutable parameters

**Access:** Access token holder only
**Returns:** Success or error

## Fee Taker

### Contract: `FeeTaker`

#### Constructor

```rust
pub fn new(limit_order_protocol: AccountId, access_token: AccountId, weth: AccountId) -> Self
```

**Parameters:**

- `limit_order_protocol`: Limit order protocol address
- `access_token`: Access token address
- `weth`: WETH token address

**Returns:** `FeeTaker` instance

#### View Functions

##### `get_limit_order_protocol()`

```rust
pub fn get_limit_order_protocol(&self) -> AccountId
```

**Returns:** Limit order protocol address

##### `get_access_token()`

```rust
pub fn get_access_token(&self) -> AccountId
```

**Returns:** Access token address

##### `get_weth()`

```rust
pub fn get_weth(&self) -> AccountId
```

**Returns:** WETH token address

##### `get_owner()`

```rust
pub fn get_owner(&self) -> AccountId
```

**Returns:** Contract owner address

#### State-Changing Functions

##### `post_interaction()`

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
) -> Result<(), FeeTakerError>
```

**Parameters:**

- `order`: Order being executed
- `extension`: Order extension data
- `order_hash`: Order hash
- `taker`: Taker address
- `making_amount`: Making amount
- `taking_amount`: Taking amount
- `remaining_making_amount`: Remaining making amount
- `extra_data`: Extra data for fee configuration

**Access:** Limit order protocol only
**Returns:** Success or error

##### `rescue_funds()`

```rust
pub fn rescue_funds(&mut self, token: AccountId, amount: u128) -> Result<(), FeeTakerError>
```

**Parameters:**

- `token`: Token address to rescue
- `amount`: Amount to rescue

**Access:** Owner only
**Returns:** Success or error

## Merkle Storage Invalidator

### Contract: `MerkleStorageInvalidator`

#### Constructor

```rust
pub fn new(limit_order_protocol: AccountId) -> Self
```

**Parameters:**

- `limit_order_protocol`: Limit order protocol address

**Returns:** `MerkleStorageInvalidator` instance

#### View Functions

##### `get_limit_order_protocol()`

```rust
pub fn get_limit_order_protocol(&self) -> AccountId
```

**Returns:** Limit order protocol address

##### `get_last_validated()`

```rust
pub fn get_last_validated(&self, key: [u8; 32]) -> Option<ValidationData>
```

**Parameters:**

- `key`: Validation key

**Returns:** Validation data if exists, `None` otherwise

##### `has_validation_data()`

```rust
pub fn has_validation_data(&self, key: [u8; 32]) -> bool
```

**Parameters:**

- `key`: Validation key

**Returns:** `true` if validation data exists, `false` otherwise

##### `get_all_validation_data()`

```rust
pub fn get_all_validation_data(&self) -> Vec<([u8; 32], ValidationData)>
```

**Returns:** All validation data as vector of (key, data) pairs

#### State-Changing Functions

##### `taker_interaction()`

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
) -> Result<(), InvalidatorError>
```

**Parameters:**

- `order`: Order being validated
- `extension`: Order extension data
- `order_hash`: Order hash
- `taker`: Taker address
- `making_amount`: Making amount
- `taking_amount`: Taking amount
- `remaining_making_amount`: Remaining making amount
- `extra_data`: Extra data containing proof

**Access:** Limit order protocol only
**Returns:** Success or error

## Data Structures

### Order

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

### MakerTraits

```rust
pub struct MakerTraits {
    pub use_bit_invalidator: bool,    // Use bit-based invalidation
    pub use_epoch_manager: bool,      // Use epoch-based management
    pub has_extension: bool,          // Order has extension data
    pub nonce_or_epoch: u64,         // Nonce or epoch value
    pub series: u64,                  // Order series identifier
}
```

### Extension

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

### ValidationData

```rust
pub struct ValidationData {
    pub leaf: [u8; 32],             // Merkle tree leaf
    pub index: u64,                  // Leaf index
}
```

## Error Types

### LimitOrderError

```rust
pub enum LimitOrderError {
    InvalidatedOrder,
    TakingAmountExceeded,
    PrivateOrder,
    InvalidSignature,
    OrderExpired,
    WrongSeriesNonce,
    SwapWithZeroAmount,
    PartialFillNotAllowed,
    OrderIsNotSuitableForMassInvalidation,
    EpochManagerAndBitInvalidatorsAreIncompatible,
    ReentrancyDetected,
    PredicateIsNotTrue,
    TakingAmountTooHigh,
    MakingAmountTooLow,
    TransferFromMakerToTakerFailed,
    TransferFromTakerToMakerFailed,
    MismatchArraysLengths,
    InvalidPermit2Transfer,
    MissingOrderExtension,
    UnexpectedOrderExtension,
    InvalidExtensionHash,
    ContractPaused,
    OrderInvalidated,
    InvalidAmounts,
    InvalidExtension,
}
```

### EscrowError

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

### FeeTakerError

```rust
pub enum FeeTakerError {
    InvalidCaller,
    InvalidFeeConfig,
    FeeNotApplicable,
    TransferFailed,
    InvalidAmount,
    UnauthorizedAccess,
}
```

### InvalidatorError

```rust
pub enum InvalidatorError {
    InvalidCaller,
    InvalidProof,
    InvalidIndex,
    InvalidData,
    StorageError,
    ValidationFailed,
}
```

## Gas Costs

### Estimated Gas Costs

| Operation           | Gas Cost |
| ------------------- | -------- |
| Order Creation      | ~50,000  |
| Order Execution     | ~100,000 |
| Order Cancellation  | ~20,000  |
| Escrow Creation     | ~50,000  |
| Escrow Withdrawal   | ~30,000  |
| Escrow Cancellation | ~25,000  |
| Fee Collection      | ~40,000  |
| Proof Validation    | ~30,000  |
| Token Transfer      | ~20,000  |

_Note: Actual gas costs may vary based on network conditions and contract complexity._

## Events

### Order Events

- `OrderCreated`: Order creation events
- `OrderFilled`: Order execution events
- `OrderCancelled`: Order cancellation events
- `OrderInvalidated`: Order invalidation events

### Escrow Events

- `EscrowCreated`: Escrow creation events
- `FundsWithdrawn`: Fund withdrawal events
- `EscrowCancelled`: Escrow cancellation events
- `PublicOperation`: Public operation events

### Fee Events

- `FeeCollected`: Fee collection events
- `TokensTransferred`: Token transfer events
- `FundsRescued`: Fund rescue events

### Validation Events

- `ProofValidated`: Proof validation events
- `DataStored`: Data storage events
- `ValidationFailed`: Validation failure events
