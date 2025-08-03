# Jincubator NEAR Fusion+

Integration of Fusion+ with NEAR

## Overview

Initial development of Jincubator NEAR Fusion+ is being developed as part of Unite DeFi 2025.

Key Features

- [1inch Limit Order Protocol](https://github.com/1inch/limit-order-protocol): Migration from Solidity to NEAR Rust Contracts
- [1inch Cross Chain Swap](https://github.com/1inch/cross-chain-swap): Migration from Solidity to NEAR Rust Contracts
- [tee-solver](https://github.com/jincubator-united-defi-2025/tee-solver) : [Updated](https://github.com/jincubator-united-defi-2025/tee-solver/commit/7ea1147d9a3d2eb04e8fdde6162edc8196bc4c0b) to support development on Mac OS.

For a technical overview please see [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/jincubator-united-defi-2025/near-fusion-plus)

## How to Build and Test Locally

Install [`cargo-near`](https://github.com/near/cargo-near)

Then and move into your contracts folder and

```bash
cargo near build
```

```bash
cargo near test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.

If you deploy for debugging purposes move into your contracts folder:

```bash
cargo near deploy build-non-reproducible-wasm <account-id>
```

If you deploy production ready smart contract:

```bash
cargo near deploy build-reproducible-wasm <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)

## Technical References

### Core Protocol Components

- **[Limit Order Protocol](./docs/contracts/limit-order-protocol.md)** - Advanced limit order protocol with partial fills and multiple execution strategies
- **[Cross-Chain Swap](./docs/contracts/cross-chain-swap.md)** - Atomic swaps across different blockchains with time-locked escrows
- **[Escrow System](./docs/contracts/escrow-system.md)** - Multi-layered escrow management for cross-chain operations
- **[Fee Taker](./docs/contracts/fee-taker.md)** - Flexible fee collection and distribution mechanisms
- **[Merkle Storage Invalidator](./docs/contracts/merkle-storage-invalidator.md)** - Advanced proof validation for complex order structures

### Architecture & Integration

- **[Architecture Overview](./docs/architecture.md)** - Comprehensive system architecture and design patterns
- **[Integration Guide](./docs/integration.md)** - Step-by-step integration instructions and examples
- **[API Reference](./docs/api-reference.md)** - Complete API documentation with function signatures and examples
- **[Deployment Guide](./docs/deployment.md)** - Production deployment instructions and best practices

### Security & Development

- **[Security Considerations](./docs/security.md)** - Security model, best practices, and risk mitigation strategies
- **[1inch Limit Order Protocol](https://github.com/1inch/limit-order-protocol)** - Original Solidity implementation
- **[1inch Cross Chain Swap](https://github.com/1inch/cross-chain-swap)** - Original Solidity implementation

### Development Tools

- **[cargo-near](https://github.com/near/cargo-near)** - NEAR smart contract development toolkit for Rust
- **[near CLI](https://near.cli.rs)** - Interact with NEAR blockchain from command line
- **[NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)** - Official NEAR SDK documentation
- **[NEAR Documentation](https://docs.near.org)** - Complete NEAR platform documentation

### Community & Support

- **[NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)** - Community Q&A
- **[NEAR Discord](https://near.chat)** - Real-time community support
- **[NEAR Telegram Developers Community Group](https://t.me/neardev)** - Developer community
- **NEAR DevHub**: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
