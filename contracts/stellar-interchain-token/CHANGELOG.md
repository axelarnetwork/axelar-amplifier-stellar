# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.0](https://github.com/axelarnetwork/axelar-amplifier-stellar/compare/stellar-interchain-token-v1.0.0...stellar-interchain-token-v1.1.0)

### ⛰️ Features

- Block regular contract endpoints during migration ([#279](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/279)) - ([cb79a78](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cb79a7884e6a28c6f41b94c4cbf73e0cba2a8756))
- Add only_owner and only_operator macros ([#240](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/240)) - ([458c97d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/458c97d65b6e62ea0be2595f1d1ff792f75f747d))

### 🐛 Bug Fixes

- *(axelar-std-derive)* Support datum in schema_impl formatting ([#312](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/312)) - ([0c5c789](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/0c5c789866f255703121a89ef150478d133c57fe))
- *(axelar-std-derive)* Enforce contractstorage enums are private ([#267](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/267)) - ([86e62f3](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/86e62f3a2470ddd4d14601f5a6e56ec5021d2233))
- *(interchain-token)* Validate minter existence in add_minter and remove_minter ([#298](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/298)) - ([ff2ad3b](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/ff2ad3b1537fba2ea7ef080c2ae1cc80a3f846c2))
- *(interchain-token)* Pass old and new owners to set_admin event ([#282](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/282)) - ([0522332](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/0522332b0ab3da191a139eeca9ff87328cf9f0bd))
- Remove all unused error codes ([#281](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/281)) - ([09b2913](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/09b2913781902947dfd2f96b4d0bced84e15fd69))
- Remove redundant ttl extensions ([#259](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/259)) - ([57fa6b5](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/57fa6b5b42d5441bc8155ab87981f16cd35eba7c))
- Avoid ignoring dead_code warnings ([#257](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/257)) - ([b4ada12](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/b4ada1297f2ee1f32da3472b6f37f2b9f607df0c))

### 🚜 Refactor

- *(axelar-std-derive)* Simplify upgradable macro ([#256](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/256)) - ([5d328c0](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/5d328c0a0eed997d0a3b4efeb5dcfc76516fdeee))
- *(interchain-token)* Remove soroban-sdk ([#292](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/292)) - ([77e7dd5](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/77e7dd58ba823909273254af4ed59a794fd76e6b))
- *(interchain-token)* Use contractstorage ([#245](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/245)) - ([0e7970b](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/0e7970b3f46f308c803874a7d9166e22da1f3a0f))
- *(interchain-token-service)* Migrate token-manager, interchain-token & destruct flow keys ([#280](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/280)) - ([e629755](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/e6297559ca061f7fccb1625d1f25d3b7b99a40ec))
- Move the run_migration function into a clearly defined interface ([#239](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/239)) - ([7bd306d](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/7bd306d9d2d4f1045814decd569188c29486d924))

### 📚 Documentation

- Fix docs publish action ([#236](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/236)) - ([cbbc410](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/cbbc41005435baf20809c892b196f468c55b84d1))

### 🧪 Testing

- *(interchain-token)* Add tests for token events ([#286](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/286)) - ([c9d3317](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/c9d3317c9a5f443355a923492bf6d6c9b36af791))

### ⚙️ Miscellaneous Tasks

- Remove all unused derive macros ([#258](https://github.com/axelarnetwork/axelar-amplifier-stellar/pull/258)) - ([3a172e1](https://github.com/axelarnetwork/axelar-amplifier-stellar/commit/3a172e17f413e68f80da7c8284b5c48ff70da745))

### Contributors

* @nbayindirli
* @TanvirDeol
* @cgorenflo
* @ahramy
* @milapsheth

## [1.0.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.2.4...stellar-interchain-token-v1.0.0)

### ⚙️ Miscellaneous Tasks

- Update package descriptions ([#226](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/226)) - ([1881ec7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/1881ec723644734f0c19c32db143e7a539f74ad3))

### Contributors

* @ahramy

## [0.2.4](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.2.3...stellar-interchain-token-v0.2.4)

### 🐛 Bug Fixes

- *(interchain-token)* Unimplemented notice for SAC methods ([#225](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/225)) - ([8c31f8e](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/8c31f8e6f56ebed5909c0e448e2758ce988aadbe))

### Contributors

* @milapsheth

## [0.2.3](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.2.2...stellar-interchain-token-v0.2.3)

### 📚 Documentation

- Add docs to contract interfaces ([#175](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/175)) - ([2f17e32](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/2f17e32b33e6d04609c3014e161ce07f9dbbef63))

### Contributors

* @TanvirDeol

## [0.2.2](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.2.1...stellar-interchain-token-v0.2.2)

### 🚜 Refactor

- Move test modules into lib.rs ([#199](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/199)) - ([51a638a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/51a638a52bdaebc4928aab9e191b28a90e73f338))

### Contributors

* @AttissNgo

## [0.2.1](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.2.0...stellar-interchain-token-v0.2.1)

### ⚙️ Miscellaneous Tasks

- Update description for packages ([#196](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/196)) - ([a20b6ab](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a20b6ab2633b3ca407c440b9ce35ff0071384638))

### Contributors

* @ahramy

## [0.2.0](https://github.com/axelarnetwork/axelar-cgp-stellar/compare/stellar-interchain-token-v0.1.0...stellar-interchain-token-v0.2.0)

### 🚜 Refactor

- [**breaking**] Rename packages and move tests under src ([#185](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/185)) - ([804c962](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/804c962a667a7889c447decf8155c4f56c7b1bdb))

### Contributors

* @ahramy

## [0.1.0]

### ⛰️ Features

- *(ITS)* Add chain name and token deploy salt and token sdk to ITS ([#95](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/95)) - ([017d421](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/017d421eb8c131a84de1b49fca89a45b094e2da9))
- *(interchain-token)* Update interchain token ([#71](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/71)) - ([6440cf8](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/6440cf86ea665ed72e8515c0fb01d4fc93f2f63d))
- *(interchain-token-service)* Deploy remote canonical token ([#123](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/123)) - ([bec2a07](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bec2a0723a4e42a6c1db0c435cc65f5a07898326))
- *(interchain-token-service)* Remote interchain token ([#118](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/118)) - ([6ec2622](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/6ec26221bd6a7583b65bde93a2f69a7abb4dacb9))
- *(interchain-token-service)* Add interchain_transfer implementation ([#115](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/115)) - ([ff1f206](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/ff1f2068702f09babb3d0b3afe4a5ebee7f7bbdf))
- *(interchain-token-service)* Deploy interchain token ([#99](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/99)) - ([bdf9443](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bdf9443d55f142a333a5d39a059c9f7479327ce4))
- *(interchain-token-service,interchain-token)* Add contract constructor ([#74](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/74)) - ([4cbaab3](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4cbaab3f1fed2878a1ad5259c40d361b85a4747f))
- *(its)* Add skeleton code for interchain token ([#56](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/56)) - ([a67775a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a67775a76d8195ed4ea89305ee2c9fd8eb087c25))
- Add extend ttl to all contracts ([#124](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/124)) - ([ab4361c](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/ab4361c58daffebd099ab386910b55a4d56d152f))
- Add macros for shared interfaces ([#105](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/105)) - ([4f513f9](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4f513f933d290cc9cc5944e5e39bcda13a136906))
- Add upgrade capabilities to all contracts ([#87](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/87)) - ([9785e8b](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/9785e8bebea93e987af664cedea3234241675d96))

### 🚜 Refactor

- Rename assert_auth macros ([#138](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/138)) - ([8239e41](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/8239e4126cdccb4156f737dd6e20fad5c2bfc239))
- [**breaking**] Update package name and references for release ([#145](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/145)) - ([bb19538](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bb195386eeda9c75d4da33eb0cf29fd9cb9b621c))
- Restrict exports to contract and contract clients ([#103](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/103)) - ([4c25023](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4c250237afce95fcd687f74e350b6b272a3d295d))
- Extract ownership management into sharable interface ([#97](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/97)) - ([df2d7d8](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/df2d7d8106e26c143757d26dfc321ffd5778d23b))
- Move shared interfaces in preparation of ownership trait extraction ([#96](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/96)) - ([e63006a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e63006a4f17abccbd1922389f1c03cc1735220b3))

### ⚙️ Miscellaneous Tasks

- Use rustfmt nightly build to introduce opinionated imports ordering ([#141](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/141)) - ([e19f588](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e19f5887dcb7f648d1aacb0fedbd6dfa9bf45eb2))
- Rename dev_dependencies to dev-dependencies in interchain token ([#63](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/63)) - ([a7106c7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a7106c7633ea4d95470330880562ae1dfe9404ed))

### Contributors

* @TanvirDeol
* @ahramy
* @cgorenflo
* @AttissNgo
* @milapsheth
* @hydrobeam
