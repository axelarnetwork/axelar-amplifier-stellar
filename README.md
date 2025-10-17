# Axelar Cross-chain Gateway Protocol for Soroban

This repo implements Axelar's [cross-chain gateway protocol](https://github.com/axelarnetwork/cgp-spec/tree/main/solidity) in Soroban for use on Stellar. The reference Solidity contracts can be found [here](https://github.com/axelarnetwork/cgp-spec/tree/main/solidity#design).

> Check configuration for CLI and Identity before deployment: https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup

## Docs

Rustdocs for this workspace can be found [here](https://axelarnetwork.github.io/axelar-amplifier-stellar/).

## Install

Install Soroban CLI

```bash
cargo install --locked stellar-cli --features opt
```

## Build

```bash
cargo build
```

## Build wasm

```bash
cargo wasm

# OR

stellar contract build
```

## Test

```bash
cargo test
```

## Coverage

```bash
cargo install cargo-llvm-cov
cargo llvm-cov
cargo llvm-cov --html # Generate coverage report
cargo llvm-cov --open # Generate coverage and open report
```

## Optimize contracts

```bash
./optimize.sh
```

## Deployment

Deployment scripts can be found in this [repo](https://github.com/axelarnetwork/axelar-contract-deployments).

## Releases

The following GitHub workflows exist to orchestrate Stellar contract releases and typically executed in sequence:

1. `release-upload-to-r2` (`reusable-build` + `reusable-upload.yaml`):  Triggered every time there is a commit push, where the workflow compiles the contracts and releases the compiled WASM file to the R2 repository with a short hash.
1. `create-release-pr`:  Creates a release PR, e.g. <https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/368>.
1. `release-dry-run`:  Executes a dry-run before the actual release.
1. `release`:  The actual release. Publishes releases to `crates.io` and uploads them to R2 by version number (e.g. v1.0.0).
1. `stellar-expert-release`: WIP (will be used to validate contracts).
