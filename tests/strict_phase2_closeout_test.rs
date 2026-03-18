use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust-sdk should live under repo root")
        .to_path_buf()
}

fn rust_sdk_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("source directory should be readable")
        .map(|entry| entry.expect("directory entry should be readable").path())
        .collect();
    entries.sort();

    for path in entries {
        if path.is_dir() {
            collect_rs_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

fn item_name(line: &str) -> Option<&str> {
    for marker in ["pub async fn ", "pub fn ", "pub struct ", "pub enum "] {
        if let Some(rest) = line.trim_start().strip_prefix(marker) {
            return rest
                .split(|ch: char| ch == '(' || ch == '{' || ch == '<' || ch.is_whitespace())
                .next()
                .filter(|name| !name.is_empty());
        }
    }
    None
}

fn requires_doc_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("pub(crate)") || trimmed.starts_with("pub use ") {
        return false;
    }

    trimmed.starts_with("pub async fn ")
        || trimmed.starts_with("pub fn ")
        || trimmed.starts_with("pub struct ")
        || trimmed.starts_with("pub enum ")
}

#[test]
fn strict_phase2_public_items_have_rustdoc() {
    let src_root = rust_sdk_src();
    let mut files = vec![src_root.join("client.rs")];
    collect_rs_files(&src_root.join("api"), &mut files);
    collect_rs_files(&src_root.join("options"), &mut files);
    collect_rs_files(&src_root.join("types"), &mut files);
    files.sort();
    files.dedup();

    let mut missing = Vec::new();

    for path in files {
        let contents = fs::read_to_string(&path).expect("source file should be readable");
        let lines: Vec<_> = contents.lines().collect();

        for (index, line) in lines.iter().enumerate() {
            if !requires_doc_comment(line) {
                continue;
            }

            let mut cursor = index;
            let mut found_doc = false;
            while cursor > 0 {
                cursor -= 1;
                let previous = lines[cursor].trim();
                if previous.is_empty() {
                    continue;
                }
                if previous.starts_with("#[") {
                    continue;
                }
                found_doc = previous.starts_with("///");
                break;
            }

            if !found_doc {
                let name = item_name(line).unwrap_or("<unknown>");
                let rel = path
                    .strip_prefix(repo_root())
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                missing.push(format!("{rel}:{}:{name}", index + 1));
            }
        }
    }

    assert!(
        missing.is_empty(),
        "missing rustdoc on public Phase 2 items: {missing:#?}"
    );
}

#[test]
fn strict_phase2_evidence_files_exist() {
    let root = repo_root();
    let required = [
        ".sisyphus/evidence/task-16-settings.txt",
        ".sisyphus/evidence/task-17-oauth2.txt",
        ".sisyphus/evidence/task-18-miscellaneous.txt",
        ".sisyphus/evidence/task-19-actions.txt",
        ".sisyphus/evidence/task-20-activitypub.txt",
        ".sisyphus/evidence/task-21-status.txt",
    ];

    let missing: Vec<_> = required
        .into_iter()
        .filter(|path| !root.join(path).is_file())
        .collect();

    assert!(
        missing.is_empty(),
        "missing strict closeout evidence files: {missing:?}"
    );
}

#[test]
fn strict_phase2_no_unwrap_in_production_code() {
    let src_root = rust_sdk_src();
    let mut files = vec![
        src_root.join("client.rs"),
        src_root.join("pagination.rs"),
        src_root.join("auth/mod.rs"),
    ];
    collect_rs_files(&src_root.join("api"), &mut files);
    collect_rs_files(&src_root.join("options"), &mut files);
    collect_rs_files(&src_root.join("types"), &mut files);
    files.sort();
    files.dedup();

    let mut offenders = Vec::new();

    for path in files {
        let contents = fs::read_to_string(&path).expect("source file should be readable");
        let production_prefix = contents
            .split("\n#[cfg(test)]")
            .next()
            .expect("split always returns at least one segment");

        for (index, line) in production_prefix.lines().enumerate() {
            if line.contains("unwrap(") {
                let rel = path
                    .strip_prefix(repo_root())
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                offenders.push(format!("{rel}:{}", index + 1));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "found unwrap() in production code paths: {offenders:#?}"
    );
}

#[test]
fn strict_phase2_wiremock_coverage_artifact_matches_source() {
    let root = repo_root();
    let artifact =
        fs::read_to_string(root.join(".sisyphus/evidence/task-f3-wiremock-coverage.txt"))
            .expect("task-f3 artifact should be readable");

    let mut actual = std::collections::BTreeMap::new();
    for entry in fs::read_dir(rust_sdk_src().join("api")).expect("api dir should be readable") {
        let path = entry.expect("entry should be readable").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if stem == "mod" {
            continue;
        }
        let count = fs::read_to_string(&path)
            .expect("api source should be readable")
            .matches("#[tokio::test]")
            .count();
        actual.insert(stem.to_string(), count);
    }

    let actual_total: usize = actual.values().sum();
    assert!(
        artifact.contains(&format!("Total Tests [{actual_total}]")),
        "task-f3 total count is stale; expected Total Tests [{actual_total}]"
    );

    for (module, count) in actual {
        let needle = format!("- {module}: tests={count}, pattern=ok");
        assert!(
            artifact.contains(&needle),
            "task-f3 missing or stale module count: {needle}"
        );
    }
}

#[test]
fn strict_phase2_parity_artifact_has_correct_pull_and_release_mappings() {
    let root = repo_root();
    let artifact = fs::read_to_string(root.join(".sisyphus/evidence/task-f1-api-parity.txt"))
        .expect("task-f1 artifact should be readable");

    let required = [
        "- ListRepoPullRequests (pull.go) -> list (pulls.rs) [doc-alias]",
        "- GetPullRequest (pull.go) -> get (pulls.rs) [doc-alias]",
        "- CreatePullRequest (pull.go) -> create (pulls.rs) [doc-alias]",
        "- EditPullRequest (pull.go) -> edit (pulls.rs) [doc-alias]",
        "- ListPullRequestCommits (pull.go) -> list_commits (pulls.rs) [doc-alias]",
        "- GetRelease (release.go) -> get (releases.rs) [doc-alias]",
    ];

    for line in required {
        assert!(
            artifact.contains(line),
            "task-f1 missing required mapping line: {line}"
        );
    }

    let forbidden = [
        "- ListRepoPullRequests (pull.go) -> list (releases.rs) [doc-alias]",
        "- GetPullRequest (pull.go) -> get (users.rs) [doc-alias]",
        "- CreatePullRequest (pull.go) -> create (releases.rs) [doc-alias]",
        "- EditPullRequest (pull.go) -> edit (releases.rs) [doc-alias]",
        "- ListPullRequestCommits (pull.go) -> list_commits (repos.rs) [doc-alias]",
        "- GetRelease (release.go) -> get (users.rs) [doc-alias]",
    ];

    for line in forbidden {
        assert!(
            !artifact.contains(line),
            "task-f1 still contains known-bad mapping line: {line}"
        );
    }
}
