// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use gitea_sdk::options::repo::{
    CreateWikiPageOptions, ListWikiPageRevisionsOptions, ListWikiPagesOptions,
};

use live::{CleanupRegistry, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn encode_wiki_content(value: &str) -> String {
    STANDARD.encode(value.as_bytes())
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_wiki_page_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-wiki-repo")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let create_resp = client
        .repos()
        .create_wiki_page(
            &owner,
            &repo,
            CreateWikiPageOptions {
                title: "Home".to_string(),
                content_base64: encode_wiki_content("initial layout"),
                message: "initial wiki page".to_string(),
            },
        )
        .await
        .expect("create wiki page");
    assert_success_status(create_resp.status);

    let (page, get_resp) = client
        .repos()
        .get_wiki_page(&owner, &repo, "Home")
        .await
        .expect("get wiki page");
    assert_success_status(get_resp.status);
    assert_eq!(page.title, "Home");

    let edit_resp = client
        .repos()
        .edit_wiki_page(
            &owner,
            &repo,
            "Home",
            CreateWikiPageOptions {
                title: "Home".to_string(),
                content_base64: encode_wiki_content("updated layout"),
                message: "update wiki".to_string(),
            },
        )
        .await
        .expect("edit wiki page");
    assert_success_status(edit_resp.status);

    let (updated_page, updated_get_resp) = client
        .repos()
        .get_wiki_page(&owner, &repo, "Home")
        .await
        .expect("get edited wiki page");
    assert_success_status(updated_get_resp.status);
    assert_eq!(
        updated_page.content_base64,
        encode_wiki_content("updated layout")
    );

    let (pages, list_resp) = client
        .repos()
        .list_wiki_pages(&owner, &repo, ListWikiPagesOptions::default())
        .await
        .expect("list wiki pages");
    assert_success_status(list_resp.status);
    assert!(pages.iter().any(|entry| entry.title == "Home"));

    let (revisions, revisions_resp) = client
        .repos()
        .get_wiki_revisions(
            &owner,
            &repo,
            "Home",
            ListWikiPageRevisionsOptions::default(),
        )
        .await
        .expect("get wiki revisions");
    assert_success_status(revisions_resp.status);
    assert!(
        revisions.commits.len() >= 2,
        "expected at least two wiki revisions"
    );

    let delete_resp = client
        .repos()
        .delete_wiki_page(&owner, &repo, "Home")
        .await
        .expect("delete wiki page");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}
