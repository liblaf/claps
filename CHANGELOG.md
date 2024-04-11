# Changelog

## [0.2.0](https://github.com/liblaf/claps/compare/v0.1.0...v0.2.0) (2024-04-11)


### Features

* add alist-cli to the project ([d6c43da](https://github.com/liblaf/claps/commit/d6c43da88295446eb1dee7e469af8f4383d448f0))
* add async-recursion and mime_guess dependencies ([8d3647f](https://github.com/liblaf/claps/commit/8d3647f7ace890afd54d4f552df28e906ab52712))
* add dynamic grouping and proxy selection based on fetched data ([3c4ffa3](https://github.com/liblaf/claps/commit/3c4ffa38ac02d802014e4f6c4db7997741a8d37b))
* add global options for ddns commands ([6c5829f](https://github.com/liblaf/claps/commit/6c5829ffe4faebd3aa44ec2400e11dd61e7e800c))
* add logging functionality to main.py and conf.rs ([95ef2f0](https://github.com/liblaf/claps/commit/95ef2f037fdcb21d1b150d97387fdf19fdb2838b))
* add options for refreshing and prefix in alist-cli upload command ([ff9553b](https://github.com/liblaf/claps/commit/ff9553bf62f670a880d934458cdf6fcd017cdd49))
* add Python configuration and scripts ([83046cf](https://github.com/liblaf/claps/commit/83046cf7bd26d6332fbd4d7937330fb3875fd398))
* add remove command to alist-cli ([1d416ea](https://github.com/liblaf/claps/commit/1d416ea0de38b15d37333c74add9b5226be6c1d3))
* add serde_with crate for custom serialization ([112b028](https://github.com/liblaf/claps/commit/112b028d2210d26b3ed27094c141bf6614be895c))
* add support for displaying traffic information in the list command ([e4ea19a](https://github.com/liblaf/claps/commit/e4ea19a8ea0a10cba6c30a9fc816193077b8b15f))
* **api:** add authentication login endpoint and file system listing functionality ([b01d1d0](https://github.com/liblaf/claps/commit/b01d1d0db56389a5f82ca449da54ae40c7dd3703))
* **cf/tunnel/list:** add `cf tunnel get` command ([#29](https://github.com/liblaf/claps/issues/29)) ([0bf8b8c](https://github.com/liblaf/claps/commit/0bf8b8c68ca7680675d8bb78cd420372391ac076))
* **cf/tunnel/load-balancer:** add command `cf tunnel load-balancer` ([#36](https://github.com/liblaf/claps/issues/36)) ([103ca90](https://github.com/liblaf/claps/commit/103ca903c894dc0088b5c2998e94d2c8e00fcdec))
* **cf/tunnel/put:** add command `cf tunnel put` ([#35](https://github.com/liblaf/claps/issues/35)) ([c5ed87e](https://github.com/liblaf/claps/commit/c5ed87e89425e5f29befd2512728570d3c38223d))
* change header color to green in STYLES constant ([24cbc31](https://github.com/liblaf/claps/commit/24cbc31db6bea7853f9ba4177c1be85bfb1e8c2e))
* **ci:** update CI workflow to include new tools and Rust toolchain installation ([e995472](https://github.com/liblaf/claps/commit/e995472ecc2dd48ad24f34215fcbcbae60efba4d))
* **gfw/ip:** add IP geolocation functionality using ip.sb API ([39d42e0](https://github.com/liblaf/claps/commit/39d42e0e97ccb0e6c8adf8021f09c48e3f88a4bd))
* **gfw/sub/list:** add functionality to list subscriptions ([abc55f6](https://github.com/liblaf/claps/commit/abc55f6c8775854c9285039b8f59fa4a9e14fc89))
* **pic:** add functionality to upload files ([0d6393d](https://github.com/liblaf/claps/commit/0d6393dc317b317e893ac5f7b2c6cb6e8d524b3d))
* update command names and structure for `gfw` CLI ([ede2f0d](https://github.com/liblaf/claps/commit/ede2f0d918a315d49a88fd2c92534111ddf6bb52))
* update default image path to /public/img ([a9ae595](https://github.com/liblaf/claps/commit/a9ae59521d5d4600e96359b20d26ad0027971616))
* update tabled dependency to version 0.15.0 ([24fb3f7](https://github.com/liblaf/claps/commit/24fb3f7ea60c6c29734a050e95a35f709f9e6401))


### Bug Fixes

* add dynamic DNS server address in DNS configuration ([8013bdd](https://github.com/liblaf/claps/commit/8013bdda891de7f3e06ad98d8e6466c68189e159))
* **cf/cmd/tunnel/load_balancer:** update service method to include a flag for secure connection ([ba80d6c](https://github.com/liblaf/claps/commit/ba80d6c1ca18c303c147405221b4b023de693de6))
* **cf/dns/install:** update executable path in ddns service ([365daa0](https://github.com/liblaf/claps/commit/365daa01051da082fa066c39eaaaabd12ee4d6d7))
* **cf/tunnel/load-balancer:** add debug logging for sorted tunnels ([9ac9dc5](https://github.com/liblaf/claps/commit/9ac9dc53a75812448c236955893efb7b7afa5f8d))
* **cf/tunnel/load-balancer:** add logging for tunnel name mapping ([b3cd62c](https://github.com/liblaf/claps/commit/b3cd62cf5fbe71c7201dad4583dd0b22e43879a1))
* **deps:** update dependency httpx to v0.27.0 ([1791806](https://github.com/liblaf/claps/commit/1791806570de7c78d5e58688194b15d7273fa927))
* **deps:** update dependency pydantic to v2.6.2 ([fcecdb2](https://github.com/liblaf/claps/commit/fcecdb22449f77c497679073889f7f8f3d1dba16))
* **deps:** update dependency pydantic to v2.6.3 ([d95fd7b](https://github.com/liblaf/claps/commit/d95fd7b93935aa6954450630b2afe80b3bf1b194))
* **deps:** update dependency pydantic to v2.6.4 ([628481f](https://github.com/liblaf/claps/commit/628481f9a892cef5beced7c9a854c963ef97d02c))
* **deps:** update dependency pydantic to v2.7.0 ([10cbbb4](https://github.com/liblaf/claps/commit/10cbbb4887c0b9f78dea00d4257c3ee24c614674))
* **deps:** update rust crate anyhow to 1.0.81 ([fb4e5ab](https://github.com/liblaf/claps/commit/fb4e5ab6833f77f7a6f2573b2f41523d5c9bddd0))
* **deps:** update rust crate anyhow to 1.0.82 ([10aca47](https://github.com/liblaf/claps/commit/10aca4724f54eb6e1f3f469424427b5945ebdc65))
* **deps:** update rust crate async-recursion to 1.1.0 ([f2c704d](https://github.com/liblaf/claps/commit/f2c704dd751bcd282264e767b12e882591ee5a7a))
* **deps:** update rust crate chrono to 0.4.35 ([6bbc952](https://github.com/liblaf/claps/commit/6bbc952a2303bf1467f26618c7f30ca7d809f9b1))
* **deps:** update rust crate chrono to 0.4.37 ([4fcf809](https://github.com/liblaf/claps/commit/4fcf8099d6e17f3be44a1875a969f9e3232bc698))
* **deps:** update rust crate clap to 4.5.2 ([13bd378](https://github.com/liblaf/claps/commit/13bd3782aba614676ef9848f0110fca1f4ffa151))
* **deps:** update rust crate clap to 4.5.3 ([f8b396b](https://github.com/liblaf/claps/commit/f8b396b981ff105a009b8a8ab0b7c94baa27a35d))
* **deps:** update rust crate clap to 4.5.4 ([6b4f731](https://github.com/liblaf/claps/commit/6b4f7313b39a33f727413f2414a17131b41bbd09))
* **deps:** update rust crate clap_complete to 4.5.2 ([beb84a0](https://github.com/liblaf/claps/commit/beb84a023f7f934ab25bd047703d7c6c8d96c875))
* **deps:** update rust crate reqwest to 0.11.25 ([842346c](https://github.com/liblaf/claps/commit/842346cee600aa5f4b918abcd216d89c7c5e55ee))
* **deps:** update rust crate reqwest to 0.11.26 ([d64f960](https://github.com/liblaf/claps/commit/d64f9605a528fff452e4258ea4bf02740c23095e))
* **deps:** update rust crate reqwest to 0.11.27 ([eef94b2](https://github.com/liblaf/claps/commit/eef94b235e43f284e4a6abfba690930367de975e))
* **deps:** update rust crate reqwest to 0.12.0 ([7470917](https://github.com/liblaf/claps/commit/7470917f42163e856db00e9162ce788060395ffc))
* **deps:** update rust crate reqwest to 0.12.1 ([1e5b223](https://github.com/liblaf/claps/commit/1e5b223994b7bc21a03b37367bf2bc4231b1a3dd))
* **deps:** update rust crate reqwest to 0.12.2 ([a46980c](https://github.com/liblaf/claps/commit/a46980c246a2a8886bf74f69376c7d9e5966ee93))
* **deps:** update rust crate reqwest to 0.12.3 ([55713b2](https://github.com/liblaf/claps/commit/55713b2698a07d8daee43174202f9bd54aa6b6a1))
* **deps:** update rust crate serde to 1.0.197 ([91aecea](https://github.com/liblaf/claps/commit/91aecea24ccfa1399608e86ff4f7fc1bbe5d8218))
* **deps:** update rust crate serde_json to 1.0.114 ([9619825](https://github.com/liblaf/claps/commit/96198251e74d9f86786aed1d8eedcec0db2e765a))
* **deps:** update rust crate serde_json to 1.0.115 ([b6fa79f](https://github.com/liblaf/claps/commit/b6fa79ff53b18d409a0cb9a1c76f1284f86a890b))
* **deps:** update rust crate serde_with to 3.7.0 ([e5cd62f](https://github.com/liblaf/claps/commit/e5cd62f14f757448b7d21f7b67dd73938332190a))
* **deps:** update rust crate shadow-rs to 0.27.0 ([dd0dee8](https://github.com/liblaf/claps/commit/dd0dee817983e3320e569722dac6724a94e687e6))
* **deps:** update rust crate shadow-rs to 0.27.1 ([98a039f](https://github.com/liblaf/claps/commit/98a039fab7318de11780460fc1e8cb86fce7bba4))
* **deps:** update rust crate tokio to 1.37.0 ([f9e0160](https://github.com/liblaf/claps/commit/f9e0160853ce1e29295c936409182660c31773e4))
* **deps:** update rust crate whoami to 1.5.0 ([4b30da6](https://github.com/liblaf/claps/commit/4b30da62b2fdc7769937388aa30f7a8b8ac54124))
* **deps:** update rust crate whoami to 1.5.1 ([68b42ec](https://github.com/liblaf/claps/commit/68b42ec61699070d08791c0ca2a461d365bfdc95))
* **gfw/ip:** refactor GeoIP struct to use Option type for nullable fields ([38577a5](https://github.com/liblaf/claps/commit/38577a557114de6ac5f70d056d140630f11bde9d))
* **gfw/sub/conf:** fix async DNS query and update DNS configuration ([ec89a72](https://github.com/liblaf/claps/commit/ec89a720ecd82c7ad0987ebd0f3877793fef8a73))
* **gfw/sub/conf:** fix handling of DNS queries in the auto DNS module ([5d0c0f5](https://github.com/liblaf/claps/commit/5d0c0f50f4d22e095a74447a3ed22520bdeb7cad))

## [0.1.0](https://github.com/liblaf/claps/compare/v0.0.0...v0.1.0) (2024-01-26)

### Features

- add `sudo` module for running commands with elevated privileges ([4ecefa7](https://github.com/liblaf/claps/commit/4ecefa773bf269076c50e912b054e34afafff7ff))
- **api:** add group delay endpoint and pretty print function ([6df9a8f](https://github.com/liblaf/claps/commit/6df9a8fa2d219b21ae1142311fb64c0db0f53719))
- **api:** add support for chaining proxies ([341119c](https://github.com/liblaf/claps/commit/341119ca3b302b5b0878653e97f581a1c137417c))
- **api:** add support for interacting with Clash API ([720679f](https://github.com/liblaf/claps/commit/720679f61877299ddb0b054579d28c1136376c62))
- **cmd:** refactor command handling and add colorized table output ([3729e62](https://github.com/liblaf/claps/commit/3729e622974a2936dbce2dc178335fbac96983c0))
- **docs:** update command usage and options in README.md ([0affd47](https://github.com/liblaf/claps/commit/0affd47fcd02b5a4ec933ec7fcb11e3982a4c013))
- **install:** add install command ([00bc74a](https://github.com/liblaf/claps/commit/00bc74a96c5770988ffdefc03d33c9d9a2529428))
- **pm:** add search command ([a490ed5](https://github.com/liblaf/claps/commit/a490ed521b9ec09862066a1ae50c4d2e193b7d24))
- remove CLOUDFLARE_ZONE environment variable ([50fc9a1](https://github.com/liblaf/claps/commit/50fc9a14a8f3a20a322644b8eb647d4278ee4323))
- update CI/CD workflows to use GitHub Actions for release process ([6a23004](https://github.com/liblaf/claps/commit/6a2300482f506b5b332857feddf28dc98acd091d))
- **workflows:** add release and build jobs ([833c51b](https://github.com/liblaf/claps/commit/833c51bbc2dfe2fe7cd667677ec1893ad68a34ce))

## 0.0.0 (2023-12-05)

### ⚠ BREAKING CHANGES

- The `ddns-cli` tool introduces a new command line interface and API integration, which may require changes to existing scripts or workflows that interact with DNS records.

### Features

- add ddns-cli command line tool ([9cc78af](https://github.com/liblaf/claps/commit/9cc78af8742855ab491d2c845bb6861ed1492bfe))

### Build System

- update Taskfile.yaml to simplify configuration ([efa21af](https://github.com/liblaf/claps/commit/efa21af183c86f122b52602d148d936dd29a7203))
