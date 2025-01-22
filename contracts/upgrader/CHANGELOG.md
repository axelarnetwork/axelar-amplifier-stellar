# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### 🧪 Testing

- Improve test coverage ([#163](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/163)) - ([d753e51](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/d753e51b535c6234f81017a55e81046128c958bd))

### ⚙️ Miscellaneous Tasks

- Revert duplicated release v0.1.0 ([#168](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/168)) - ([b672e2f](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/b672e2f7515d55833c997b94667d21d1d108fd69))
- Use rustfmt nightly build to introduce opinionated imports ordering ([#141](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/141)) - ([e19f588](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e19f5887dcb7f648d1aacb0fedbd6dfa9bf45eb2))

### Contributors

* @ahramy
* @talalashraf
* @AttissNgo
* @cgorenflo
* @milapsheth

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
