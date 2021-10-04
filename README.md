# cargo-search2

[![cargo-search2 on crates.io](https://img.shields.io/crates/v/cargo-search2)](https://crates.io/crates/cargo-search2) [![Documentation (latest release)](https://docs.rs/cargo-search2/badge.svg)](https://docs.rs/cargo-search2/) [![Documentation (main)](https://img.shields.io/badge/docs-main-brightgreen)](https://sunshowers.github.io/cargo-search2/rustdoc/cargo_search2/) [![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE) [![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

A binary utility that provides a more convenient version of `cargo search`.

## Installation

Grab pre-built binaries for your platform:

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
{"crate-name":"serde","version":"1.0.130","hash":"blake2b24:48d4caa68090087560e9c4b174bfa95435078e4949195eb1"}

# Get an exact version (also accepts arbitrary versions)
$ cargo search2 serde --req '=1.0.120'
{"crate-name":"serde","version":"1.0.120","hash":"blake2b24:41a5040c3830edc7e758bf828a93fa6a305509cf7bc9017a"}

# Print out GitHub Actions commands
$ cargo search2 serde --message-format github
::set-output name=crate-name::serde
::set-output name=version::1.0.130
::set-output name=hash::blake2b24:48d4caa68090087560e9c4b174bfa95435078e4949195eb1
```

## Why?

People often use `cargo search` to generate cache keys for tools. This is an easier way to achieve the same goal.

In particular, it supports:
* looking for version specifiers
* producing results in both JSON and GitHub Actions formats

## Usage
