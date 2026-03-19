// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Entry {
    module: String,
    method: String,
    status: String,
    test: Option<String>,
    reason: Option<String>,
}

fn manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("live_coverage_manifest.json")
}

fn api_modules() -> Vec<(String, Vec<String>)> {
    let api_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("api");
    let mut modules = Vec::new();

    for entry in std::fs::read_dir(&api_dir).expect("api dir readable") {
        let path = entry.expect("entry readable").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        if path.file_stem().and_then(|stem| stem.to_str()) == Some("mod") {
            continue;
        }

        let module = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("module must have name")
            .to_string();
        let contents = fs::read_to_string(&path).expect("api file readable");
        let mut methods = Vec::new();
        for line in contents.lines() {
            if let Some(name) = line.trim().strip_prefix("pub async fn ") {
                let name = name
                    .split(|ch: char| ch == '(' || ch.is_whitespace())
                    .next()
                    .unwrap_or_default()
                    .to_string();
                if !name.is_empty() {
                    methods.push(name);
                }
            }
        }
        modules.push((module, methods));
    }

    modules.sort_by(|a, b| a.0.cmp(&b.0));
    modules
}

#[test]
fn manifest_matches_source() {
    let content = fs::read_to_string(manifest_path()).expect("manifest readable");
    let entries: Vec<Entry> =
        serde_json::from_str(&content).expect("manifest should parse as JSON array");
    let mut by_module: BTreeMap<&str, Vec<&Entry>> = BTreeMap::new();
    let mut seen = HashSet::new();

    for entry in &entries {
        assert!(
            seen.insert((&entry.module, &entry.method)),
            "duplicate manifest entry for {}::{}",
            entry.module,
            entry.method
        );
        by_module.entry(&entry.module).or_default().push(entry);

        if entry.status == "covered" {
            let test = entry.test.as_ref().expect("covered entry needs test field");
            let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join(test);
            assert!(
                test_path.exists(),
                "covered entry {}::{} references missing {}",
                entry.module,
                entry.method,
                test
            );
            assert!(
                entry.reason.is_none(),
                "covered entry {}::{} should not include blocked reason",
                entry.module,
                entry.method
            );
        } else {
            assert!(
                entry
                    .reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "blocked entry {}::{} must include reason",
                entry.module,
                entry.method
            );
        }
        assert!(
            matches!(
                entry.status.as_str(),
                "covered"
                    | "blocked_by_permissions"
                    | "blocked_by_server_capability"
                    | "blocked_by_missing_seed_data"
                    | "blocked_by_sdk_bug"
            ),
            "invalid status {}",
            entry.status
        );
    }

    let modules = api_modules();
    let mut total_methods = 0;
    for (module, methods) in modules {
        let manifest_methods = by_module
            .get(module.as_str())
            .map(|vec| vec.iter().map(|e| e.method.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();
        assert_eq!(
            methods.len(),
            manifest_methods.len(),
            "module {} method count mismatch",
            module
        );
        total_methods += methods.len();
        for method in methods {
            assert!(
                manifest_methods.iter().any(|m| *m == method),
                "missing manifest entry for {}::{}",
                module,
                method
            );
        }
    }

    assert_eq!(total_methods, entries.len(), "manifest total mismatch");
}
