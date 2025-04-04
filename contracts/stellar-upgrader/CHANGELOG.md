# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0](https://github.com/axelarnetwork/axelar-amplifier-stellar/compare/stellar-upgrader-v1.0.0...stellar-upgrader-v1.1.0)

### ⛰️ Features

- Check authorization at the root in the upgrader contract ([#294](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/294)) - ([b3d7019](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/b3d701985f98b9ab8eee4e7110f9a3c3bf68143a))
- Block regular contract endpoints during migration ([#279](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/279)) - ([cb79a78](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cb79a7884e6a28c6f41b94c4cbf73e0cba2a8756))
- Add only_owner and only_operator macros ([#240](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/240)) - ([458c97d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/458c97d65b6e62ea0be2595f1d1ff792f75f747d))

### 🐛 Bug Fixes

- *(axelar-std-derive)* Enforce contractstorage enums are private ([#267](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/267)) - ([86e62f3](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/86e62f3a2470ddd4d14601f5a6e56ec5021d2233))
- *(upgrader)* Cast upgrader contract invoke_contract to val ([#305](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/305)) - ([7f1edc2](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/7f1edc25426e8a950129545705c61ca78b51d48e))

### 🚜 Refactor

- *(axelar-operators)* Migrate Operators to Operator ([#252](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/252)) - ([6682292](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/66822927410d94f1ad6238899cf6029479754fac))
- *(interchain-token-service)* Migrate token-manager, interchain-token & destruct flow keys ([#280](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/280)) - ([e629755](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/e6297559ca061f7fccb1625d1f25d3b7b99a40ec))
- *(upgrader)* Remove soroban-sdk ([#295](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/295)) - ([46fe5bf](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/46fe5bfa4d30f15148d0ad21c4d219f7db5dd443))
- *(upgrader)* Use contractstorage ([#247](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/247)) - ([71a8cae](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/71a8cae51792ec812f2cca073a44836b825a97d0))

### ⚙️ Miscellaneous Tasks

- Update auth invocation macros ([#269](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/269)) - ([be8d3b9](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/be8d3b9d2763c1862dad3d99b04fabcd48fe1b76))

### Contributors

* @TanvirDeol
* @nbayindirli
* @cgorenflo
* @ahramy

## [1.0.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.2.4...stellar-upgrader-v1.0.0)

### ⚙️ Miscellaneous Tasks

- Update package descriptions ([#226](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/226)) - ([1881ec7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/1881ec723644734f0c19c32db143e7a539f74ad3))

### Contributors

* @ahramy

## [0.2.4](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.2.3...stellar-upgrader-v0.2.4)

### ⚙️ Miscellaneous Tasks

- Add build check to fail on warnings ([#227](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/227)) - ([af05c6f](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/af05c6f670f7d324eebbadb6f611412527761603))

### Contributors

* @TanvirDeol

## [0.2.3](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.2.2...stellar-upgrader-v0.2.3)

### 📚 Documentation

- Add docs to contract interfaces ([#175](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/175)) - ([2f17e32](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/2f17e32b33e6d04609c3014e161ce07f9dbbef63))

### Contributors

* @TanvirDeol

## [0.2.2](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.2.1...stellar-upgrader-v0.2.2)

### 🚜 Refactor

- Move test modules into lib.rs ([#199](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/199)) - ([51a638a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/51a638a52bdaebc4928aab9e191b28a90e73f338))

### Contributors

* @AttissNgo

## [0.2.1](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.2.0...stellar-upgrader-v0.2.1)

### ⚙️ Miscellaneous Tasks

- Update description for packages ([#196](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/196)) - ([a20b6ab](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a20b6ab2633b3ca407c440b9ce35ff0071384638))

### Contributors

* @ahramy

## [0.2.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-upgrader-v0.1.0...stellar-upgrader-v0.2.0)

### 🚜 Refactor

- [**breaking**] Rename packages and move tests under src ([#185](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/185)) - ([804c962](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/804c962a667a7889c447decf8155c4f56c7b1bdb))

### Contributors

* @ahramy

## [0.1.0]

### ⛰️ Features

- *(axelar-gateway)* Emit event when upgrade is completed ([#85](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/85)) - ([7c17383](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/7c17383e77b925e8f9d52f8d362b4e1918a6f377))
- *(upgrader)* Add generalized atomic upgrade and migration capabilities ([#77](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/77)) - ([48507e2](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/48507e256ef91a89b0a7da1fb88cbb1a5ad5ebea))
- Add upgrade capabilities to all contracts ([#87](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/87)) - ([9785e8b](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/9785e8bebea93e987af664cedea3234241675d96))

### 🚜 Refactor

- *(upgrader)* Clean up the UpgraderInterface and simplify the contract ([#84](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/84)) - ([8f298ff](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/8f298ff7585a29e6adef7cf29fdbf71c0c1e146b))
- Update mock auth macro to support non root auth  ([#134](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/134)) - ([7b6a553](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/7b6a55385fc0bdcbd7d6bf065ddaa0f81dceb51f))
- [**breaking**] Update package name and references for release ([#145](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/145)) - ([bb19538](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bb195386eeda9c75d4da33eb0cf29fd9cb9b621c))
- Restrict exports to contract and contract clients ([#103](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/103)) - ([4c25023](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4c250237afce95fcd687f74e350b6b272a3d295d))
- Extract ownership management into sharable interface ([#97](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/97)) - ([df2d7d8](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/df2d7d8106e26c143757d26dfc321ffd5778d23b))
- Move shared interfaces in preparation of ownership trait extraction ([#96](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/96)) - ([e63006a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e63006a4f17abccbd1922389f1c03cc1735220b3))

### ⚙️ Miscellaneous Tasks

- Use rustfmt nightly build to introduce opinionated imports ordering ([#141](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/141)) - ([e19f588](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e19f5887dcb7f648d1aacb0fedbd6dfa9bf45eb2))

### Contributors

* @ahramy
* @cgorenflo
* @milapsheth
