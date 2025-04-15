# `stellar-axelar-*` Crate Release Process

## Prerequisites

- Access to the [axelar-amplifier-stellar](https://github.com/axelarnetwork/axelar-amplifier-stellar)
  - Responsible for maintaining the stellar-axelar standard libraries and contracts.

## Release Process

### 1. Dry-Run Release

1. Navigate to the [Dry-Run Release](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/release-dry-run.yaml) GitHub Action.
2. Run the workflow with the following:
   - Use workflow from: `main` branch
3. If successful, continue; otherwise, address the failure and repeat step 1.

### 2. Build and upload pre-release

1. Navigate to the [Build and upload pre-release](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/pre-release.yaml) GitHub Action.
2. Run the workflow with the following:
   - Use workflow from: `main` branch

### 3. Create Release PR

1. Navigate to the [Create Release PR](https://github.com/axelarnetwork/axelar-amplifier-stellar/actions/workflows/create-release-pr.yaml) GitHub Action.
2. Run the workflow with the following:
   - Use workflow from: `main` branch
3. Visit the generated PR and self-review the changes.
4. Reach out to maintainers for approval, and merge the PR.
