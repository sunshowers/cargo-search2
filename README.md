# cargo-search2

[![cargo-search2 on crates.io](https://img.shields.io/crates/v/cargo-search2)](https://crates.io/crates/cargo-search2) [![Documentation (latest release)](https://docs.rs/cargo-search2/badge.svg)](https://docs.rs/cargo-search2/) [![Documentation (main)](https://img.shields.io/badge/docs-main-brightgreen)](https://sunshowers.github.io/cargo-search2/rustdoc/cargo_search2/) [![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE) [![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

A binary utility that provides a more convenient version of `cargo search`.

## Installation

Grab pre-built binaries for your platform from [the releases page](https://github.com/sunshowers/cargo-search2/releases/latest).

To grab releases in CI or other automation:

```sh
curl -LsSf https://github.com/sunshowers/cargo-search2/releases/latest/download/cargo-search2-{platform}.zip | tar xzf - -C ~/.cargo/bin
```

Install manually:

```sh
cargo install cargo-search2
```

## Examples

```sh
$ cargo search2 serde
# output:
serde 1.0.130 (hash: blake2b24:2e6117acae4f6f2d2d698c9fc0b43de60ac26556f44ad96b)

$ cargo search2 serde --message-format toml
serde = "1.0.130"

$ cargo search2 serde --message-format json
{"crate-name":"serde","version":"1.0.130","hash":"blake2b24:2e6117acae4f6f2d2d698c9fc0b43de60ac26556f44ad96b"}

# Get an exact version or semver range
$ cargo search2 serde --req '=1.0.120'
serde 1.0.120 (hash: blake2b24:f79131f50c298569f841e47287c3b0a443c29cfda3ffd475)

# Use --cache-version/-c to invalidate caches
$ cargo run -- serde --req '=1.0.120' -c1
serde 1.0.120 (hash: blake2b24:168778241e3b0e96f5547fba1eb4b4b84efa8291c3c14b37)
$ cargo run -- serde --req '=1.0.120' -c2
serde 1.0.120 (hash: blake2b24:344d8a8d08a16dbce8c4e0d90fd6aad513631ca081185ea8)

# Step outputs for GitHub Actions
$ cargo search2 serde --message-format github
::set-output name=crate-name::serde
::set-output name=version::1.0.120
::set-output name=hash::blake2b24:f79131f50c298569f841e47287c3b0a443c29cfda3ffd475
```

For an example GitHub Action, see [`.github/workflows/example.yml](.github/workflows/example.yml).

## Why?

People sometimes use `cargo search` to generate cache keys for tools. This is an easier way to achieve the same goal with a
couple more features.

`cargo search2` supports:
* looking for exact versions and semver ranges
* producing results in plain, JSON, and GitHub Actions formats
* easy cache invalidation through the `--cache-version` option

## Hash contents

The hashes produced by `cargo search2` are derived from:
* The name of the crate
* The exact version number
* The cache version

Notably, the operating system and other parts of the environment are *not* part of the hash.

## Stability guarantees

Within a semver range, the command-line interface is append-only, and hashes stay the same.

## Contributing

Pull requests are welcome! Please follow the [code of conduct](CODE_OF_CONDUCT.md).

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
