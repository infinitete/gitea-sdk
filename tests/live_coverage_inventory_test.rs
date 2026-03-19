// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Ensures the Rust source baseline of live API methods matches the recorded artifact.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn rust_sdk_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    rust_sdk_root()
        .parent()
        .expect("rust-sdk lives under repository root")
        .to_path_buf()
}

fn baseline_path() -> PathBuf {
    repo_root()
        .join("docs")
        .join("plans")
        .join("live-api-method-baseline-2026-03-18.txt")
}

fn live_api_module_counts() -> io::Result<BTreeMap<String, usize>> {
    let api_dir = rust_sdk_root().join("src").join("api");
    let mut counts = BTreeMap::new();

    for entry in fs::read_dir(&api_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }

        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default();
        if stem == "mod" {
            continue;
        }

        let contents = fs::read_to_string(&path)?;
        let hits = contents.matches("pub async fn").count();
        counts.insert(stem.to_string(), hits);
    }

    Ok(counts)
}

struct Baseline {
    total_methods: usize,
    module_counts: BTreeMap<String, usize>,
}

impl Baseline {
    fn load(path: &Path) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let mut module_counts = BTreeMap::new();
        let mut total_methods = None;

        for line in contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let (key, value) = match trimmed.split_once(':') {
                Some(kv) => kv,
                None => continue,
            };

            let key = key.trim();
            let value = value.trim();

            if key.eq_ignore_ascii_case("total methods") {
                total_methods = Some(value.parse::<usize>().map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid total methods count: {err}"),
                    )
                })?);
            } else if key.eq_ignore_ascii_case("note") {
                continue;
            } else {
                let count = value.parse::<usize>().map_err(|err| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid count for {key}: {err}"),
                    )
                })?;
                module_counts.insert(key.to_string(), count);
            }
        }

        let total_methods = total_methods.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "baseline missing Total Methods entry",
            )
        })?;

        Ok(Self {
            total_methods,
            module_counts,
        })
    }
}

#[test]
fn live_api_inventory_matches_baseline() {
    let baseline = Baseline::load(&baseline_path()).expect(
        "baseline artifact should exist at docs/plans/live-api-method-baseline-2026-03-18.txt",
    );
    let modules = live_api_module_counts()
        .expect("should be able to read rust-sdk/src/api/*.rs source files");

    let computed_total: usize = modules.values().sum();
    for (module, count) in &modules {
        println!("live inventory: {} -> {}", module, count);
    }
    println!("live inventory: total methods -> {}", computed_total);

    assert_eq!(
        computed_total, baseline.total_methods,
        "sum of live API modules changed without updating the baseline"
    );
    assert_eq!(
        modules, baseline.module_counts,
        "per-module counts deviated from the documented baseline"
    );
}
