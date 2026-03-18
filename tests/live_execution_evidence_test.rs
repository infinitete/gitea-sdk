// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Entry {
    status: String,
}

fn manifest_entries() -> Vec<Entry> {
    let content = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("live_coverage_manifest.json"),
    )
    .expect("manifest readable");
    serde_json::from_str(&content).expect("manifest parseable")
}

fn baseline_total() -> usize {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .join("docs")
        .join("plans")
        .join("live-api-method-baseline-2026-03-18.txt");
    let contents = fs::read_to_string(path).expect("baseline readable");
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with("total methods") {
            return trimmed
                .split_once(':')
                .and_then(|(_, value)| value.trim().parse().ok())
                .expect("total methods parseable");
        }
    }
    panic!("Total Methods line missing in baseline");
}

#[test]
fn manifest_counts_match_baseline() {
    let entries = manifest_entries();
    let mut counts = BTreeMap::new();
    for entry in entries {
        *counts.entry(entry.status).or_insert(0usize) += 1;
    }
    let sum: usize = counts.values().sum();
    let expected = baseline_total();
    assert_eq!(
        sum, expected,
        "manifest entries ({sum}) != baseline ({expected})"
    );
}
