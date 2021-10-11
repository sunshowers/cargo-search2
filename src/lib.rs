// Copyright (c) The cargo-search2 Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A version of `cargo search`, suitable for CI caching in GitHub Actions and elsewhere.
//!
//! Provides output in JSON and GitHub Actions formats.
//!
//! For more, see the [crates.io page](https://crates.io/cargo-search2).

use blake2b_simd::Params;
use color_eyre::{
    eyre::{bail, eyre, WrapErr},
    Result,
};
use crates_index::BareIndex;
use semver::{Version, VersionReq};
use serde::Serialize;
use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

static DEFAULT_INDEX_URL: &str = "https://github.com/rust-lang/crates.io-index";

/// Search packages on crates.io by version.
///
/// This is a version of `cargo search` that uses exact names rather than searching on prefixes. It
/// supports matching by semver ranges, hashing version numbers, and a number of output formats
/// including JSON and GitHub Actions.
///
/// `cargo search2` is optimized for cache keys and lookups on GitHub Actions and other CI
/// platforms.
#[derive(Debug, StructOpt)]
#[doc(hidden)]
pub struct Args {
    /// The index to use (default: same as Cargo's).
    #[structopt(long)]
    index_path: Option<PathBuf>,

    /// Version specification to match
    #[structopt(long = "req", default_value)]
    version_req: VersionReq,

    /// Bump this up to invalidate cache keys
    #[structopt(long, short = "c", default_value)]
    cache_version: u64,

    /// Output format
    #[structopt(long, possible_values = &MessageFormat::variants(), case_insensitive = true, default_value = "plain")]
    message_format: MessageFormat,

    /// The name of the package to look for.
    crate_name: String,
}

const BLAKE2B_PREFIX: &str = "blake2b24:";
const BLAKE2B_HASH_LEN: usize = 24;
const MAX_VERSIONS_IN_ERR: usize = 8;

arg_enum! {
    #[derive(Copy, Clone, Debug)]
    #[allow(non_camel_case_types)]
    enum MessageFormat {
        toml,
        plain,
        json,
        github,
    }
}

impl Args {
    /// Execute this and return
    pub fn exec(self) -> Result<()> {
        let index = match &self.index_path {
            Some(path) => BareIndex::with_path(path.clone(), DEFAULT_INDEX_URL),
            None => BareIndex::new_cargo_default(),
        };

        let mut repo = index.open_or_clone().wrap_err_with(|| {
            format!(
                "opening or cloning index at {} failed",
                index.path().display()
            )
        })?;

        repo.retrieve()
            .wrap_err_with(|| format!("updating index at {} failed", index.path().display()))?;

        let crate_ = repo
            .crate_(&self.crate_name)
            .ok_or_else(|| eyre!("crate {} not found", &self.crate_name))?;
        let mut matching_versions: Vec<Version> = crate_
            .versions()
            .iter()
            .filter_map(|version| {
                let version = match version.version().parse::<Version>() {
                    Ok(version) => version,
                    Err(_) => {
                        // Weird but ok, skip over this
                        return None;
                    }
                };

                if self.version_req.matches(&version) {
                    Some(version)
                } else {
                    None
                }
            })
            .collect();

        matching_versions.sort_unstable();
        let output = match matching_versions.last() {
            Some(found_version) => {
                let mut params = Params::new();
                let mut state = params.hash_length(BLAKE2B_HASH_LEN).to_state();

                state.update(crate_.name().as_bytes());
                state.update(b"\0");
                state.update(found_version.to_string().as_bytes());
                state.update(b"\0");
                state.update(&self.cache_version.to_be_bytes());
                let blake2b_hash = state.finalize();
                let blake2b_hex = blake2b_hash.to_hex();

                // Currently support blake2b only.

                let hash_str = BLAKE2B_PREFIX.to_owned() + blake2b_hex.as_str();
                Output {
                    crate_name: crate_.name().to_owned(),
                    version: found_version.clone(),
                    hash: hash_str,
                }
            }
            None => {
                let versions = crate_.versions();
                let latest_versions: Vec<_> = crate_
                    .versions()
                    .iter()
                    .rev()
                    .take(8)
                    .map(|v| v.version())
                    .collect();
                let versions_found_str = latest_versions.join(", ");

                let and_more = if versions.len() > MAX_VERSIONS_IN_ERR {
                    format!(" and {} more", versions.len() - MAX_VERSIONS_IN_ERR)
                } else {
                    String::new()
                };
                bail!(
                    "for crate {}, no matching versions for req {} (versions found: {}{})",
                    crate_.name(),
                    self.version_req,
                    versions_found_str,
                    and_more,
                );
            }
        };

        match self.message_format {
            MessageFormat::plain => {
                println!(
                    "{} {} (hash: {})",
                    output.crate_name, output.version, output.hash
                );
            }
            MessageFormat::toml => {
                println!(r#"{} = "{}""#, output.crate_name, output.version);
            }
            MessageFormat::json => {
                let json = serde_json::to_string(&output).wrap_err_with(|| {
                    format!("couldn't serialize serde output for {:?}", output)
                })?;
                println!("{}", json);
            }
            MessageFormat::github => {
                println!("::set-output name=crate-name::{}", output.crate_name);
                println!("::set-output name=version::{}", output.version);
                println!("::set-output name=hash::{}", output.hash);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
struct Output {
    crate_name: String,
    version: Version,
    hash: String,
}
