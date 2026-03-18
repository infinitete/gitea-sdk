use std::fs;
use std::path::{Path, PathBuf};

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

#[test]
fn strict_phase3_api_coverage_2x_per_method() {
    let api_dir = rust_sdk_src().join("api");
    let mut files = Vec::new();
    collect_rs_files(&api_dir, &mut files);

    // Filter out mod.rs and keep only direct api/*.rs files
    let modules: Vec<_> = files
        .into_iter()
        .filter(|p| {
            p.extension().and_then(|ext| ext.to_str()) == Some("rs")
                && p.file_stem().and_then(|s| s.to_str()) != Some("mod")
                && p.parent() == Some(api_dir.as_path())
        })
        .collect();

    assert!(
        !modules.is_empty(),
        "Phase 3 coverage: no direct api/*.rs modules found under {}",
        api_dir.display()
    );

    let mut failures: Vec<String> = Vec::new();
    let mut total_pub_async_fn_count = 0usize;
    let mut total_tokio_test_count = 0usize;

    for path in &modules {
        let contents = fs::read_to_string(path).expect("api source file should be readable");

        let pub_async_fn_count = contents.matches("pub async fn").count();
        let tokio_test_count = contents.matches("#[tokio::test]").count();
        total_pub_async_fn_count += pub_async_fn_count;
        total_tokio_test_count += tokio_test_count;

        let required = pub_async_fn_count * 2;

        if tokio_test_count < required {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            failures.push(format!(
                "{}: need {} tests (2x {} methods), have {} tests (gap: {})",
                stem,
                required,
                pub_async_fn_count,
                tokio_test_count,
                required - tokio_test_count,
            ));
        }
    }

    assert!(
        total_pub_async_fn_count > 0,
        "Phase 3 coverage: discovered {} modules but found zero public async API methods",
        modules.len()
    );
    assert!(
        total_tokio_test_count > 0,
        "Phase 3 coverage: discovered {} modules but found zero #[tokio::test] cases",
        modules.len()
    );

    if failures.is_empty() {
        return;
    }

    let mut report = String::from("Phase 3 coverage: 2x tests per method FAILED\n\n");
    for f in &failures {
        report.push_str(&format!("  - {}\n", f));
    }
    report.push_str(&format!(
        "\n{} modules below threshold out of {} checked; totals: methods={}, tests={}\n",
        failures.len(),
        modules.len(),
        total_pub_async_fn_count,
        total_tokio_test_count,
    ));

    panic!("{}", report);
}
