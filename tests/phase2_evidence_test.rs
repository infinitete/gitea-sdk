use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust-sdk should live under repo root")
        .to_path_buf()
}

#[test]
fn phase2_plan_evidence_files_exist() {
    let root = repo_root();
    let required = [
        ".sisyphus/evidence/task-1-deps-resolve.txt",
        ".sisyphus/evidence/task-1-no-ssh-deps.txt",
        ".sisyphus/evidence/task-2-serde-zero-time.txt",
        ".sisyphus/evidence/task-2-serde-roundtrip.txt",
        ".sisyphus/evidence/task-3-enums-compile.txt",
        ".sisyphus/evidence/task-3-unknown-enum.txt",
        ".sisyphus/evidence/task-4-user-serde.txt",
        ".sisyphus/evidence/task-4-types-compile.txt",
        ".sisyphus/evidence/task-5-test-helpers.txt",
        ".sisyphus/evidence/task-6-api-scaffold.txt",
        ".sisyphus/evidence/task-7-reexports.txt",
        ".sisyphus/evidence/task-8-users-api.txt",
        ".sisyphus/evidence/task-8-follow-api.txt",
        ".sisyphus/evidence/task-9-orgs-api.txt",
        ".sisyphus/evidence/task-9-membership.txt",
        ".sisyphus/evidence/task-10-admin-api.txt",
        ".sisyphus/evidence/task-11-issues-create.txt",
        ".sisyphus/evidence/task-11-milestones.txt",
        ".sisyphus/evidence/task-12-merge.txt",
        ".sisyphus/evidence/task-12-patch.txt",
        ".sisyphus/evidence/task-13-releases.txt",
        ".sisyphus/evidence/task-14-hooks.txt",
        ".sisyphus/evidence/task-15-notifications.txt",
        ".sisyphus/evidence/task-16-settings.txt",
        ".sisyphus/evidence/task-17-oauth2.txt",
        ".sisyphus/evidence/task-18-miscellaneous.txt",
        ".sisyphus/evidence/task-19-actions.txt",
        ".sisyphus/evidence/task-20-activitypub.txt",
        ".sisyphus/evidence/task-21-status.txt",
        ".sisyphus/evidence/task-22-repo-types.txt",
        ".sisyphus/evidence/task-22-zero-time.txt",
        ".sisyphus/evidence/task-23-repo-options.txt",
        ".sisyphus/evidence/task-23-validate.txt",
        ".sisyphus/evidence/task-24-repos-get.txt",
        ".sisyphus/evidence/task-24-collaborator.txt",
        ".sisyphus/evidence/task-24-languages.txt",
        ".sisyphus/evidence/task-24-version-gated.txt",
        ".sisyphus/evidence/task-f1-api-parity.txt",
        ".sisyphus/evidence/task-f2-code-quality.txt",
        ".sisyphus/evidence/task-f3-wiremock-coverage.txt",
        ".sisyphus/evidence/task-f4-scope-fidelity.txt",
    ];

    let missing: Vec<_> = required
        .into_iter()
        .filter(|path| !root.join(path).is_file())
        .collect();

    assert!(
        missing.is_empty(),
        "missing phase2 evidence files: {missing:?}"
    );
}
