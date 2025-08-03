# Jincubator NEAR Fusion+

Integration of Fusion+ with NEAR

## Overview

Initial development of Jincubator NEAR Fusion+ is being developed as part of Unite DeFi 2025.

Key Features

- [1inch Limit Order Protocol](https://github.com/1inch/limit-order-protocol): Migration from Solidity to NEAR Rust Contracts
- [1inch Cross Chain Swap](https://github.com/1inch/cross-chain-swap): Migration from Solidity to NEAR Rust Contracts

For a technical overview please see [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/jincubator-united-defi-2025/near-fusion-plus)

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

If you deploy for debugging purposes:

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
