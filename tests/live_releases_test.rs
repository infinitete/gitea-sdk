// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::release::{
    CreateReleaseOption, EditAttachmentOption, EditReleaseOption, ListReleaseAttachmentsOptions,
    ListReleasesOptions,
};

use live::{
    CleanupRegistry, create_release_fixture, create_repo_fixture, live_client, unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_release_and_attachment_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-release-repo")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo_name = repo_fixture.repository.name.clone();

    let release_fixture =
        create_release_fixture(&client, &mut cleanup, &owner, &repo_name, "live-release")
            .await
            .expect("create release fixture");
    let release_id = release_fixture.release.id;
    let release_tag = release_fixture.release.tag_name.clone();

    let (releases, list_response) = client
        .releases()
        .list(&owner, &repo_name, ListReleasesOptions::default())
        .await
        .expect("list releases");
    assert_success_status(list_response.status);
    assert!(releases.iter().any(|entry| entry.id == release_id));

    let (loaded, get_response) = client
        .releases()
        .get(&owner, &repo_name, release_id)
        .await
        .expect("get release");
    assert_success_status(get_response.status);
    assert_eq!(loaded.id, release_id);

    let (latest, latest_response) = client
        .releases()
        .get_latest(&owner, &repo_name)
        .await
        .expect("get latest release");
    assert_success_status(latest_response.status);
    assert_eq!(latest.id, release_id);

    let (by_tag, tag_response) = client
        .releases()
        .get_by_tag(&owner, &repo_name, &release_tag)
        .await
        .expect("get release by tag");
    assert_success_status(tag_response.status);
    assert_eq!(by_tag.tag_name, release_tag);

    let (edited, edit_response) = client
        .releases()
        .edit(
            &owner,
            &repo_name,
            release_id,
            EditReleaseOption {
                title: Some(format!("{}-edited", release_tag)),
                note: Some("updated release".to_string()),
                is_draft: Some(false),
                is_prerelease: Some(false),
                tag_name: None,
                target: None,
            },
        )
        .await
        .expect("edit release");
    assert_success_status(edit_response.status);
    assert!(edited.title.contains("edited"));

    let attachment_name = unique_name("live-release-asset");
    let (attachment, attach_resp) = client
        .releases()
        .create_attachment(
            &owner,
            &repo_name,
            release_id,
            format!("asset content for {attachment_name}").into_bytes(),
            &format!("{attachment_name}.txt"),
        )
        .await
        .expect("create release attachment");
    assert_success_status(attach_resp.status);

    let (attachments, list_attachment_resp) = client
        .releases()
        .list_attachments(
            &owner,
            &repo_name,
            release_id,
            ListReleaseAttachmentsOptions::default(),
        )
        .await
        .expect("list release attachments");
    assert_success_status(list_attachment_resp.status);
    assert!(attachments.iter().any(|entry| entry.id == attachment.id));

    let (_, get_attachment_resp) = client
        .releases()
        .get_attachment(&owner, &repo_name, release_id, attachment.id)
        .await
        .expect("get attachment");
    assert_success_status(get_attachment_resp.status);

    let (edited_attachment, edit_attachment_resp) = client
        .releases()
        .edit_attachment(
            &owner,
            &repo_name,
            release_id,
            attachment.id,
            EditAttachmentOption {
                name: format!("{attachment_name}-updated.txt"),
            },
        )
        .await
        .expect("edit attachment");
    assert_success_status(edit_attachment_resp.status);
    assert!(edited_attachment.name.contains("updated"));

    let delete_attachment_resp = client
        .releases()
        .delete_attachment(&owner, &repo_name, release_id, attachment.id)
        .await
        .expect("delete attachment");
    assert_success_status(delete_attachment_resp.status);

    let delete_resp = client
        .releases()
        .delete(&owner, &repo_name, release_id)
        .await
        .expect("delete release");
    assert_success_status(delete_resp.status);

    let new_release_pref = unique_name("live-release-2");
    let (_new_release, new_create_resp) = client
        .releases()
        .create(
            &owner,
            &repo_name,
            CreateReleaseOption {
                tag_name: new_release_pref.clone(),
                target: None,
                title: Some(new_release_pref.clone()),
                note: Some("live release 2".to_string()),
                is_draft: false,
                is_prerelease: false,
            },
        )
        .await
        .expect("create second release");
    assert_success_status(new_create_resp.status);

    let delete_by_tag_resp = client
        .releases()
        .delete_by_tag(&owner, &repo_name, &new_release_pref)
        .await
        .expect("delete release by tag");
    assert_success_status(delete_by_tag_resp.status);

    cleanup.run_all().await;
}
