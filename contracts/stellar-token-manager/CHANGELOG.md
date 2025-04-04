# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0](https://github.com/axelarnetwork/axelar-amplifier-stellar/compare/stellar-token-manager-v1.0.0...stellar-token-manager-v1.1.0)

### ⛰️ Features

- *(axelar-std-derive)* Globally extend instance TTL on all contract endpoints ([#310](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/310)) - ([010a505](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/010a505376f92a5771f0ca942c1ebd7448f406ae))
- Block regular contract endpoints during migration ([#279](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/279)) - ([cb79a78](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cb79a7884e6a28c6f41b94c4cbf73e0cba2a8756))
- Add only_owner and only_operator macros ([#240](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/240)) - ([458c97d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/458c97d65b6e62ea0be2595f1d1ff792f75f747d))

### 🐛 Bug Fixes

- Avoid ignoring dead_code warnings ([#257](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/257)) - ([b4ada12](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/b4ada1297f2ee1f32da3472b6f37f2b9f607df0c))

### 🚜 Refactor

- *(axelar-std-derive)* Simplify upgradable macro ([#256](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/256)) - ([5d328c0](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/5d328c0a0eed997d0a3b4efeb5dcfc76516fdeee))
- *(interchain-token-service)* Migrate token-manager, interchain-token & destruct flow keys ([#280](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/280)) - ([e629755](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/e6297559ca061f7fccb1625d1f25d3b7b99a40ec))
- *(token-manager)* Remove soroban-sdk ([#290](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/290)) - ([b06c903](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/b06c903543670b9a51e8f87a3c1a6b76d0ae6e0b))
- Move the run_migration function into a clearly defined interface ([#239](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/239)) - ([7bd306d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/7bd306d9d2d4f1045814decd569188c29486d924))

### 📚 Documentation

- Fix docs publish action ([#236](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/236)) - ([cbbc410](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cbbc41005435baf20809c892b196f468c55b84d1))

### Contributors

* @nbayindirli
* @cgorenflo
* @TanvirDeol
* @milapsheth

## [1.0.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-token-manager-v0.1.0...stellar-token-manager-v1.0.0)

### ⚙️ Miscellaneous Tasks

- Update package descriptions ([#226](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/226)) - ([1881ec7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/1881ec723644734f0c19c32db143e7a539f74ad3))

### Contributors

* @ahramy

## [0.1.0]

### ⛰️ Features

- *(token-manager)* Add token manager for ITS ([#215](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/215)) - ([42d7b34](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/42d7b348a4b419ce77c35688f93ba803c2e5ef1e))

### Contributors

* @milapsheth
