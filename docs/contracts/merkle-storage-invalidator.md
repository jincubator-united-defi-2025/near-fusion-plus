# Merkle Storage Invalidator

## Overview

The Merkle Storage Invalidator contract provides advanced proof validation for complex order structures, particularly for orders that support multiple partial fills. It implements efficient Merkle tree validation and storage management for cross-chain atomic swaps.

## Contract Structure

### Main Contract: `MerkleStorageInvalidator`

```rust
pub struct MerkleStorageInvalidator {
    limit_order_protocol: AccountId,
    last_validated: UnorderedMap<[u8; 32], ValidationData>,
}
```

### Key Components

- **Limit Order Protocol**: Address of the main limit order protocol
- **Last Validated**: Storage for validation data with Merkle tree information

## Core Functionality

### Merkle Proof Validation

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
) -> Result<(), InvalidatorError>
```

**Process:**

1. Validate caller is limit order protocol
2. Extract post interaction data from extension
3. Parse taker data from extra data
4. Extract root from hashlock info
5. Create validation key
6. Validate Merkle proof
7. Store validation data

#### Proof Validation

```rust
fn validate_merkle_proof(
    proof: &Vec<[u8; 32]>,
    leaf: [u8; 32],
    index: u64,
    root: [u8; 32],
) -> bool
```

- **Proof Verification**: Validates Merkle proof against root
- **Index Calculation**: Calculates correct path through tree
- **Hash Computation**: Computes hashes for proof validation
- **Root Comparison**: Compares computed root with expected root

### Data Structures

#### Taker Data

```rust
pub struct TakerData {
    pub secret_hash: [u8; 32],      // Hash of the secret
    pub idx: u64,                    // Index in Merkle tree
    pub proof: Vec<[u8; 32]>,       // Merkle proof
}
```

#### Validation Data

```rust
pub struct ValidationData {
    pub leaf: [u8; 32],             // Merkle tree leaf
    pub index: u64,                  // Leaf index
}
```

### Storage Management

#### Validation Key Creation

```rust
fn create_validation_key(order_hash: &[u8; 32], root: &[u8; 32]) -> [u8; 32]
```

- **Key Generation**: Creates unique validation key
- **Hash Combination**: Combines order hash and root
- **Uniqueness**: Ensures unique keys for different orders

#### Data Storage

```rust
fn store_validation_data(&mut self, key: [u8; 32], data: ValidationData)
```

- **Efficient Storage**: Gas-efficient data storage
- **Index Tracking**: Tracks validation indices
- **State Management**: Manages validation state

## Merkle Tree Operations

### Tree Construction

#### Leaf Generation

```rust
fn generate_leaf(secret_hash: &[u8; 32], index: u64) -> [u8; 32]
```

- **Hash Combination**: Combines secret hash with index
- **Leaf Creation**: Creates unique leaf for each secret
- **Index Encoding**: Encodes index in leaf structure

#### Proof Generation

```rust
fn generate_proof(secrets: &Vec<[u8; 32]>, target_index: u64) -> Vec<[u8; 32]>
```

- **Tree Traversal**: Traverses tree from leaf to root
- **Sibling Collection**: Collects sibling hashes for proof
- **Path Calculation**: Calculates path through tree

### Validation Process

#### Root Extraction

```rust
fn extract_root(hashlock_info: &[u8; 32]) -> [u8; 32]
```

- **Data Parsing**: Extracts root from hashlock information
- **Format Validation**: Validates data format
- **Root Verification**: Ensures root is valid

#### Index Validation

```rust
fn validate_index(index: u64, max_index: u64) -> bool
```

- **Range Check**: Validates index is within range
- **Bounds Validation**: Ensures index is valid
- **Overflow Prevention**: Prevents index overflow

## Security Features

### Access Control

#### Protocol Validation

```rust
fn validate_limit_order_protocol(caller: &AccountId, protocol: &AccountId) -> Result<(), InvalidatorError>
```

- **Caller Validation**: Only limit order protocol can call
- **Protocol Verification**: Ensures correct protocol address
- **Security**: Prevents unauthorized validation

### Proof Security

#### Proof Validation

- **Cryptographic Security**: Uses cryptographic hash functions
- **Replay Protection**: Prevents proof replay attacks
- **Index Uniqueness**: Ensures unique indices for each validation

#### Data Integrity

- **Hash Verification**: Verifies data integrity through hashing
- **State Consistency**: Maintains consistent validation state
- **Tamper Protection**: Protects against data tampering

### Error Handling

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

## Integration Points

### External Dependencies

- **Limit Order Protocol**: Main protocol contract
- **Base Escrow Factory**: Advanced escrow creation
- **NEAR Protocol**: Base blockchain infrastructure

### Internal Dependencies

- **Order Execution**: Validation during order execution
- **Escrow Creation**: Triggers escrow creation
- **Proof Management**: Manages proof validation

## Usage Examples

### Creating Invalidator

```rust
let invalidator = MerkleStorageInvalidator::new(
    "limit-order.near".parse().unwrap(),
);
```

### Taker Interaction

```rust
let result = invalidator.taker_interaction(
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

### Getting Validation Data

```rust
let validation_data = invalidator.get_last_validated(key);
```

### Checking Validation Status

```rust
let has_data = invalidator.has_validation_data(key);
```

## Gas Optimization

### Efficient Storage

- **UnorderedMap**: Gas-efficient storage for validation data
- **Minimal State**: Only essential data stored
- **Lazy Loading**: On-demand data loading patterns

### Gas Costs

- **Proof Validation**: ~30,000 gas
- **Data Storage**: ~15,000 gas
- **Key Generation**: ~5,000 gas
- **Validation Check**: ~10,000 gas

## Testing

### Unit Tests

- Merkle proof validation
- Data storage and retrieval
- Index validation
- Error condition handling

### Integration Tests

- End-to-end validation scenarios
- Cross-contract interactions
- Gas consumption validation
- Proof generation and validation

## Monitoring

### Key Events

- `ProofValidated`: Proof validation events
- `DataStored`: Data storage events
- `ValidationFailed`: Validation failure events
- `IndexUpdated`: Index update events

### Metrics

- Proof validation success rate
- Storage utilization
- Gas consumption patterns
- Validation frequency
- Error rates

## Merkle Tree Applications

### Multiple Fill Support

- **Partial Fills**: Support for multiple partial fills
- **Order Tracking**: Track fill status through indices
- **Efficient Validation**: Efficient validation for large orders

### Cross-Chain Validation

- **Proof Verification**: Verify proofs across chains
- **State Synchronization**: Synchronize state across chains
- **Atomic Operations**: Ensure atomic validation

### Advanced Features

#### Batch Validation

```rust
fn validate_batch_proofs(proofs: &Vec<MerkleProof>) -> bool
```

- **Batch Processing**: Validate multiple proofs at once
- **Gas Efficiency**: Reduce gas costs for batch operations
- **Parallel Processing**: Process proofs in parallel

#### Dynamic Tree Updates

```rust
fn update_tree(new_leaves: &Vec<[u8; 32]>) -> [u8; 32]
```

- **Dynamic Updates**: Update tree with new leaves
- **Root Recalculation**: Recalculate root after updates
- **State Consistency**: Maintain consistent tree state

## Security Considerations

### Best Practices

1. **Proof Validation**: Comprehensive proof validation
2. **Index Management**: Proper index tracking and validation
3. **Data Integrity**: Ensure data integrity through hashing
4. **Access Control**: Strict access control for validation

### Risk Mitigation

- **Replay Protection**: Prevent proof replay attacks
- **Index Validation**: Validate indices are within bounds
- **Root Verification**: Verify root authenticity
- **State Consistency**: Maintain consistent validation state

### Cryptographic Security

- **Hash Functions**: Use cryptographically secure hash functions
- **Randomness**: Ensure proper randomness for secrets
- **Collision Resistance**: Use collision-resistant hash functions
- **Preimage Resistance**: Ensure preimage resistance
