// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::package::ListPackagesOptions;

use live::{CleanupRegistry, create_generic_package_fixture, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_generic_package_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-package-repo")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo_name = repo_fixture.repository.name.clone();
    let package_fixture = create_generic_package_fixture(&client, &mut cleanup, "live-pkg")
        .await
        .expect("create generic package fixture");
    assert_eq!(package_fixture.owner, owner);

    let (packages, list_resp) = client
        .packages()
        .list_packages(&owner, ListPackagesOptions::default())
        .await
        .expect("list packages");
    assert_success_status(list_resp.status);
    assert!(packages.iter().any(|pkg| {
        pkg.package_type == package_fixture.package_type
            && pkg.name == package_fixture.name
            && pkg.version == package_fixture.version
    }));

    let (latest, latest_resp) = client
        .packages()
        .get_latest_package(&owner, &package_fixture.package_type, &package_fixture.name)
        .await
        .expect("get latest package");
    assert_success_status(latest_resp.status);
    assert_eq!(latest.name, package_fixture.name);
    assert_eq!(latest.version, package_fixture.version);

    let (package, package_resp) = client
        .packages()
        .get_package(
            &owner,
            &package_fixture.package_type,
            &package_fixture.name,
            &package_fixture.version,
        )
        .await
        .expect("get package");
    assert_success_status(package_resp.status);
    assert_eq!(package.name, package_fixture.name);
    assert_eq!(package.id, package_fixture.package.id);

    let (files, files_resp) = client
        .packages()
        .list_package_files(
            &owner,
            &package_fixture.package_type,
            &package_fixture.name,
            &package_fixture.version,
        )
        .await
        .expect("list package files");
    assert_success_status(files_resp.status);
    assert!(
        files
            .iter()
            .any(|file| file.name == package_fixture.file_name)
    );

    let link_resp = client
        .packages()
        .link_package(
            &owner,
            &package_fixture.package_type,
            &package_fixture.name,
            &repo_name,
        )
        .await
        .expect("link package");
    assert_success_status(link_resp.status);

    let unlink_resp = client
        .packages()
        .unlink_package(&owner, &package_fixture.package_type, &package_fixture.name)
        .await
        .expect("unlink package");
    assert_success_status(unlink_resp.status);

    let delete_resp = client
        .packages()
        .delete_package(
            &owner,
            &package_fixture.package_type,
            &package_fixture.name,
            &package_fixture.version,
        )
        .await
        .expect("delete package");
    assert_success_status(delete_resp.status);

    let (packages_after_delete, list_after_delete_resp) = client
        .packages()
        .list_packages(&owner, ListPackagesOptions::default())
        .await
        .expect("list packages after delete");
    assert_success_status(list_after_delete_resp.status);
    assert!(!packages_after_delete.iter().any(|pkg| {
        pkg.package_type == package_fixture.package_type
            && pkg.name == package_fixture.name
            && pkg.version == package_fixture.version
    }));

    cleanup.run_all().await;
}
