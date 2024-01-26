# Changelog

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
