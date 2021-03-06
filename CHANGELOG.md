# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Nothing.

### Changed

- Nothing.

### Deprecated

- Nothing.

### Removed

- Nothing.

### Fixed

- Nothing.

### Security

- Nothing.


## [0.4.0] - 2020-09-16

### Added

- More verbose error handling in the sandbox launcher.
- New rpc `forge/operations`.
- New docker-compose file to start a setup with the sandbox launcher, tezedge-explorer front-end and tezedge-debugger.

### Changed

- Nothing.

### Deprecated

- Nothing.

### Removed

- Nothing.

### Fixed

- Various bugs in the sandbox launcher.

### Security

- Nothing.


## [0.3.0] - 2020-08-31

### Added

- New configuration parameter `--disable-bootstrap-lookup` to turn off DNS lookup for peers (e.g. used for tests or sandbox).
- New configuration parameter `--db-cfg-max-threads` to better control system resources.
- New RPCs to make baking in sandbox mode possible with tezos-client.
- Support for MacOS (10.13 and newer).
- Enabling core dumps in debug mode (if not set), set max open files for process
- New sandbox module to launch the light-node via RPCs.

### Changed

- Resolved various clippy warnings/errors.
- Drone test runs offline with carthagenet-snapshoted nodes.
- New OCaml FFI - `ocaml-rs` was replaced with a new custom library based on `caml-oxide` to get GC under control and improve performance.
- P2P bootstrap process - NACK version control after metadata exchange.

### Deprecated

- Nothing.

### Removed

- Nothing.

### Fixed

- Nothing.

### Security

- Nothing.

## [0.2.0] - 2020-07-29

### Added

- RPCs for every protocol now support the Tezos indexer 'blockwatch/tzindex'.
- Support for connecting to Mainnet.
- Support for sandboxing, which means an empty TezEdge can be initialized with `tezos-client` for "activate protocol" and do "transfer" operation.

### Changed

- FFI upgrade based on Tezos gitlab latest-release (v7.2), now supports OCaml 4.09.1
- Support for parallel access (readonly context) to Tezos FFI OCaml runtime through r2d2 connection pooling.

### Deprecated

- Nothing.

### Removed

- Nothing.

### Fixed

- Nothing.

### Security

- Nothing.

## [0.1.0] - 2020-06-25

### Added

- Mempool P2P support + FFI prevalidator protocol validation.
- Support for sandboxing (used in drone tests).
- RPC for /inject/operation (draft).
- RPCs for developers for blocks and contracts.
- Possibility to run mulitple sub-process with FFI integration to OCaml.

### Changed

- Upgraded version of riker, RocksDB.
- Improved DRONE integration tests.

## [0.0.2] - 2020-06-01

### Added

- Support for connection to Carthagenet/Mainnet.
- Support for Ubuntu 20 and OpenSUSE Tumbleweed.
- RPCs for indexer blockwatch/tzindex (with drone integration test, which compares indexed data with Ocaml node against TezEdge node).
- Flags `--store-context-actions=BOOL.` If this flag is set to false, the node will persist less data to disk, which increases runtime speed.

### Changed

- P2P speed-up bootstrap - support for p2p_version 1 feature Nack_with_list, extended Nack - with potential peers to connect.

### Removed

- Storing all P2P messages (moved to tezedge-debugger), the node will persist less data to disk.

### Fixed / Security

- Remove bitvec dependency.
- Refactored FFI to Ocaml not using BigArray1's for better GC processing.

## [0.0.1] - 2020-03-31

### Added

- P2P Explorer support with dedicated RPC exposed.
- Exposed RPC for Tezos indexers.
- Ability to connect and bootstrap data from Tezos Babylonnet.
- Protocol FFI integration.

[Unreleased]: https://github.com/simplestaking/tezedge/compare/v0.0.4...HEAD
[0.0.1]: https://github.com/simplestaking/tezedge/releases/v0.0.1
[0.0.2]: https://github.com/simplestaking/tezedge/releases/v0.0.2
[0.1.0]: https://github.com/simplestaking/tezedge/releases/v0.1.0
[0.2.0]: https://github.com/simplestaking/tezedge/releases/v0.2.0
[0.3.0]: https://github.com/simplestaking/tezedge/releases/v0.3.0
[0.4.0]: https://github.com/simplestaking/tezedge/releases/v0.4.0
___
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
