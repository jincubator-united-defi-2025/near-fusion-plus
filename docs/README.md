# NEAR Fusion+ Documentation

## Overview

NEAR Fusion+ is a comprehensive DeFi protocol that migrates 1inch's Limit Order Protocol and Cross-Chain Swap functionality to the NEAR blockchain. This project implements advanced trading features including limit orders, cross-chain atomic swaps, and sophisticated escrow mechanisms.

## Architecture

The protocol consists of several interconnected smart contracts that work together to provide a complete DeFi trading experience:

### Core Components

1. **Limit Order Protocol** - Handles limit order creation, execution, and management
2. **Cross-Chain Swap** - Enables atomic swaps across different blockchains
3. **Escrow System** - Manages secure fund escrow for cross-chain operations
4. **Fee Management** - Handles fee collection and distribution
5. **Merkle Validation** - Provides proof validation for complex order structures

### Contract Structure

```
src/
├── limit-order-protocol/     # Main limit order functionality
├── cross-chain-swap/         # Cross-chain atomic swap implementation
├── base-escrow-factory/      # Advanced escrow factory with Merkle validation
├── escrow-factory/           # Standard escrow factory
├── escrow-src/              # Source chain escrow contract
├── escrow-dst/              # Destination chain escrow contract
├── fee-taker/               # Fee collection and management
└── merkle-storage-invalidator/ # Merkle proof validation
```

## Key Features

- **Limit Orders**: Advanced limit order protocol with partial fills and multiple execution strategies
- **Cross-Chain Swaps**: Atomic swaps between different blockchains with time-locked escrows
- **Merkle Proofs**: Efficient validation for complex order structures
- **Fee Management**: Flexible fee collection and distribution mechanisms
- **Security**: Comprehensive validation and timelock mechanisms

## Documentation Sections

- [Architecture Overview](./architecture.md)
- [Contract Documentation](./contracts/)
  - [Limit Order Protocol](./contracts/limit-order-protocol.md)
  - [Cross-Chain Swap](./contracts/cross-chain-swap.md)
  - [Escrow System](./contracts/escrow-system.md)
  - [Fee Taker](./contracts/fee-taker.md)
  - [Merkle Storage Invalidator](./contracts/merkle-storage-invalidator.md)
- [Integration Guide](./integration.md)
- [Security Considerations](./security.md)
- [API Reference](./api-reference.md)
- [Deployment Guide](./deployment.md)

## Quick Start

1. **Build Contracts**: `cargo near build`
2. **Run Tests**: `cargo test`
3. **Deploy**: Use the deployment scripts in `deployment-scripts/`

## Development

- **Rust Version**: See `rust-toolchain.toml`
- **NEAR SDK**: v5.15.1
- **Testing**: Integration tests in `integration-tests/`

## Contributing

Please refer to the main [README.md](../README.md) for development setup and contribution guidelines.
