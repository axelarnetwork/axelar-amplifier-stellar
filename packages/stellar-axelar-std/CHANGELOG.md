# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0](https://github.com/axelarnetwork/axelar-amplifier-stellar/compare/stellar-axelar-std-v1.0.0...stellar-axelar-std-v1.1.0)

### ⛰️ Features

- *(axelar-std-derive)* Add status support to contractstorage macro ([#234](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/234)) - ([4ffb0cb](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/4ffb0cbef155ce4185a4c41ab45258c10527f598))
- *(axelar-std-derive)* Add contractstorage attribute macro ([#216](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/216)) - ([94632d8](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/94632d86cc36315c19750777aa0bf5724d104d7f))
- Check authorization at the root in the upgrader contract ([#294](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/294)) - ([b3d7019](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/b3d701985f98b9ab8eee4e7110f9a3c3bf68143a))
- Make the soroban-sdk available through stellar-axelar-std ([#284](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/284)) - ([d049765](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/d049765a90c59a472f55bd67a05622532526e515))
- Block regular contract endpoints during migration ([#279](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/279)) - ([cb79a78](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cb79a7884e6a28c6f41b94c4cbf73e0cba2a8756))
- Add custom executable interface for AxelarExecutableInterface for ease of use ([#265](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/265)) - ([18fb30e](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/18fb30eb84dc0b7e0251d24dc0a31479f07a8183))
- Increase threshold for extending storage rent ([#266](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/266)) - ([9881f1f](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/9881f1ff2e120c6e87b5097a95737ae90a083410))
- Add only_owner and only_operator macros ([#240](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/240)) - ([458c97d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/458c97d65b6e62ea0be2595f1d1ff792f75f747d))

### 🐛 Bug Fixes

- *(interchain-token-service)* Ensure token metadata has valid ASCII encoding ([#263](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/263)) - ([f9896b0](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/f9896b0433efa2ef7893fee679244806600677cb))

### 🚜 Refactor

- *(axelar-operators)* Migrate Operators to Operator ([#252](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/252)) - ([6682292](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/66822927410d94f1ad6238899cf6029479754fac))
- *(axelar-std)* Use contractstorage for macro storage ([#276](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/276)) - ([258cb8b](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/258cb8bc832a538404ef22cf6fe213a5240d6436))
- *(axelar-std-derive)* Simplify upgradable macro ([#256](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/256)) - ([5d328c0](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/5d328c0a0eed997d0a3b4efeb5dcfc76516fdeee))
- *(axelar-std-derive)* Simplify contractstorage macro ([#241](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/241)) - ([bdc02e6](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/bdc02e640c07a81758e487269a5473fcccf54b37))
- *(interchain-token-service)* Use contractstorage ([#246](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/246)) - ([94b47ef](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/94b47ef469c84048eb3b56e7adc951effc3f3733))
- *(upgrader)* Remove soroban-sdk ([#295](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/295)) - ([46fe5bf](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/46fe5bfa4d30f15148d0ad21c4d219f7db5dd443))
- Move ownable and operatable modules to new test files ([#262](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/262)) - ([33b489f](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/33b489f5375e9b18c179e15ecd4c5b0d8e042c7a))
- Move the run_migration function into a clearly defined interface ([#239](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/239)) - ([7bd306d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/7bd306d9d2d4f1045814decd569188c29486d924))

### ⚙️ Miscellaneous Tasks

- Update auth invocation macros ([#269](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/269)) - ([be8d3b9](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/be8d3b9d2763c1862dad3d99b04fabcd48fe1b76))
- Remove all unused derive macros ([#258](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/258)) - ([3a172e1](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/3a172e17f413e68f80da7c8284b5c48ff70da745))

### Contributors

* @cgorenflo
* @ahramy
* @nbayindirli
* @TanvirDeol

## [1.0.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-axelar-std-v0.3.0...stellar-axelar-std-v1.0.0)

### ⚙️ Miscellaneous Tasks

- Update package descriptions ([#226](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/226)) - ([1881ec7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/1881ec723644734f0c19c32db143e7a539f74ad3))

### Contributors

* @ahramy

## [0.3.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-axelar-std-v0.2.2...stellar-axelar-std-v0.3.0)

### ⛰️ Features

- *(axelar-gateway)* Add more queries ([#207](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/207)) - ([ca3b486](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/ca3b4861a1a26b63cad5f12daa86a71a29107cee))
- *(axelar-std)* [**breaking**] Add pausable interface ([#204](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/204)) - ([0d4af95](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/0d4af958562e502df15dcd6bc50ec4ec66cbae46))
- *(axelar-std-derive)* Add macro to execute when contract is not paused ([#214](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/214)) - ([03d1a48](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/03d1a48b8ad9d0f4f87fc18d1ffbe6405c814fb5))
- *(token-manager)* Add token manager for ITS ([#215](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/215)) - ([42d7b34](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/42d7b348a4b419ce77c35688f93ba803c2e5ef1e))

### 🐛 Bug Fixes

- *(axelar-std-derive)* Cleanup dependencies ([#213](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/213)) - ([c986ce8](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/c986ce8f689d118e78f6d1435bbe7bffd42ad3fd))

### 🚜 Refactor

- *(interchain-token-service)* Separate ITS logic into modules ([#219](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/219)) - ([86c7bac](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/86c7bac9cf2e52d515c841dc6c4e571e12645e90))

### Contributors

* @milapsheth

## [0.2.2](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-axelar-std-v0.2.1...stellar-axelar-std-v0.2.2)

### 🚜 Refactor

- Move test modules into lib.rs ([#199](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/199)) - ([51a638a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/51a638a52bdaebc4928aab9e191b28a90e73f338))

### Contributors

* @AttissNgo

## [0.2.1](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-axelar-std-v0.2.0...stellar-axelar-std-v0.2.1)

### ⚙️ Miscellaneous Tasks

- Update description for packages ([#196](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/196)) - ([a20b6ab](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a20b6ab2633b3ca407c440b9ce35ff0071384638))

### Contributors

* @ahramy

## [0.2.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-axelar-std-v0.1.0...stellar-axelar-std-v0.2.0)

### 🚜 Refactor

- [**breaking**] Rename packages and move tests under src ([#185](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/185)) - ([804c962](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/804c962a667a7889c447decf8155c4f56c7b1bdb))

### Contributors

* @ahramy

## [0.1.0]

### ⛰️ Features

- Simplify event definition via IntoEvent derive macro ([#136](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/136)) - ([9052c78](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/9052c7886b8d2ea12f33a1fdcceaa7d159890c4e))

### 🚜 Refactor

- Update mock auth macro to support non root auth  ([#134](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/134)) - ([7b6a553](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/7b6a55385fc0bdcbd7d6bf065ddaa0f81dceb51f))
- Rename assert_auth macros ([#138](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/138)) - ([8239e41](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/8239e4126cdccb4156f737dd6e20fad5c2bfc239))
- [**breaking**] Update package name and references for release ([#145](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/145)) - ([bb19538](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bb195386eeda9c75d4da33eb0cf29fd9cb9b621c))

### 🧪 Testing

- Check auth is used in assert_auth ([#151](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/151)) - ([4d8e920](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4d8e92065d528cd48a08319449b80f32322e5b08))

### Contributors

* @milapsheth
* @ahramy
* @nbayindirli
* @TanvirDeol
