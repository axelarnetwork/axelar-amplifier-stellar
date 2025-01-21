# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- *(interchain-token-service)* Cleanup its execute handlers ([#157](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/157)) - ([1a5876d](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/1a5876d89ac9eff147c728fd2ce778fdc2f1565c))
- Rename assert_auth macros ([#138](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/138)) - ([8239e41](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/8239e4126cdccb4156f737dd6e20fad5c2bfc239))
- [**breaking**] Update package name and references for release ([#145](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/145)) - ([bb19538](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/bb195386eeda9c75d4da33eb0cf29fd9cb9b621c))
- Restrict exports to contract and contract clients ([#103](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/103)) - ([4c25023](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/4c250237afce95fcd687f74e350b6b272a3d295d))
- Extract ownership management into sharable interface ([#97](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/97)) - ([df2d7d8](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/df2d7d8106e26c143757d26dfc321ffd5778d23b))
- Move shared interfaces in preparation of ownership trait extraction ([#96](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/96)) - ([e63006a](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e63006a4f17abccbd1922389f1c03cc1735220b3))

### 🧪 Testing

- Improve test coverage ([#163](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/163)) - ([d753e51](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/d753e51b535c6234f81017a55e81046128c958bd))

### ⚙️ Miscellaneous Tasks

- Use rustfmt nightly build to introduce opinionated imports ordering ([#141](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/141)) - ([e19f588](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/e19f5887dcb7f648d1aacb0fedbd6dfa9bf45eb2))
- Rename dev_dependencies to dev-dependencies in interchain token ([#63](https://github.com/axelarnetwork/axelar-cgp-stellar/pull/63)) - ([a7106c7](https://github.com/axelarnetwork/axelar-cgp-stellar/commit/a7106c7633ea4d95470330880562ae1dfe9404ed))

### Contributors

* @AttissNgo
* @milapsheth
* @talalashraf
* @TanvirDeol
* @ahramy
* @cgorenflo
* @hydrobeam

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
