# `stellar-axelar-*` Crate Release Process

## Prerequisites

### Access

- Access to the [axelar-amplifier-stellar](https://github.com/axelarnetwork/axelar-amplifier-stellar) repository.
  - Responsible for maintaining the stellar-axelar standard libraries and contracts.
- Access to the [axelar-contract-deployments](https://github.com/axelarnetwork/axelar-contract-deployments) repository.
  - Responsible for deploying, upgrading, and migrating Stellar network contracts.

### Changes

- Changes to the `axelar-amplifier-stellar` crates are merged to the `main` branch.
- Changes to the `axelar-contract-deployments` are made in the `main` branch. These should include:
  - A new release doc in the `releases/stellar/` directory: <https://github.com/axelarnetwork/axelar-contract-deployments/tree/main/releases/stellar>
  - Upgrade support for any `stellar-axelar-*` contracts that derive `Upgradable`.
  - Migration support for any `stellar-axelar-*` contracts that are tagged with `#[migratable]`.
  - Deployment support for any non-upgradable `stellar-axelar-*` contracts.

## Release Process

### 1. Build and upload pre-release

1. Navigate to the [Build and upload pre-release](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/pre-release.yaml) GitHub Action.
2. Run the workflow with the following:
    a. Use workflow from: `main` branch
3. If successful, continue; otherwise, address the failure and repeat step 1.

### 2. Dry-Run Release

1. Navigate to the [Dry-Run Release](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/release-dry-run.yaml) GitHub Action.
2. Run the workflow with the following:
    a. Use workflow from: `main` branch
3. If successful, continue; otherwise, address the failure and repeat step 1.

### 3. Release

1. Navigate to the [Release](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/release.yaml) GitHub Action.
2. Run the workflow with the following:
    a. Use workflow from: `main` branch

### 4. Create Release PR

1. Navigate to the [Create Release PR](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/create-release-pr.yaml) GitHub Action.
2. Run the workflow with the following:
    a. Use workflow from: `main` branch
3. Visit the generated PR and self-review the changes.
4. Reach out to maintainers for approval, and merge the PR.

### 5. Verify Release

1. Navigate to the newly released crates on [crates.io](https://crates.io/search?q=stellar-axelar).
2. Verify the version numbers for each of the following crates are as expected:
    - `packages/`
        - [`stellar-axelar-std`](https://crates.io/crates/stellar-axelar-std)
        - [`stellar-axelar-std-derive`](https://crates.io/crates/stellar-axelar-std-derive)
    - `contracts/`
        - [`stellar-axelar-example`](https://crates.io/crates/stellar-axelar-example)
        - [`stellar-axelar-gas-service`](https://crates.io/crates/stellar-axelar-gas-service)
        - [`stellar-axelar-gateway`](https://crates.io/crates/stellar-axelar-gateway)
        - [`stellar-axelar-operators`](https://crates.io/crates/stellar-axelar-operators)
        - [`stellar-interchain-token`](https://crates.io/crates/stellar-interchain-token)
        - [`stellar-interchain-token-service`](https://crates.io/crates/stellar-interchain-token-service)
        - [`stellar-multicall`](https://crates.io/crates/stellar-multicall)
        - [`stellar-token-manager`](https://crates.io/crates/stellar-token-manager)
        - [`stellar-upgrader`](https://crates.io/crates/stellar-upgrader)

Your new release in now ready for permissionless interop with the Axelar network.
