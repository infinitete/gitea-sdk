// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::{Engine as _, engine::general_purpose};
use gitea_sdk::options::repo::{
    CreateWikiPageOptions, ListRepoTopicsOptions, ListWikiPagesOptions,
};

use live::{CleanupRegistry, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn encode_content(content: &str) -> String {
    general_purpose::STANDARD.encode(content)
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_wiki_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-wiki")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let title = "Live-Test-Page";
    let initial_content = encode_content("## Live header");
    let create_response = client
        .repos()
        .create_wiki_page(
            &owner,
            &repo,
            CreateWikiPageOptions {
                title: title.to_string(),
                content_base64: initial_content.clone(),
                message: "create live page".to_string(),
            },
        )
        .await
        .expect("create wiki page");
    assert_success_status(create_response.status);

    let updated_content = encode_content("## Live header\n\nUpdated content");
    let edit_response = client
        .repos()
        .edit_wiki_page(
            &owner,
            &repo,
            title,
            CreateWikiPageOptions {
                title: "Live-Test-Page".to_string(),
                content_base64: updated_content,
                message: "update live page".to_string(),
            },
        )
        .await
        .expect("edit wiki page");
    assert_success_status(edit_response.status);

    let (reviewed, get_response) = client
        .repos()
        .get_wiki_page(&owner, &repo, "Live-Test-Page")
        .await
        .expect("get wiki page");
    assert_success_status(get_response.status);
    assert_eq!(reviewed.title, "Live-Test-Page");
    assert_ne!(reviewed.content_base64, initial_content);

    let (list, list_response) = client
        .repos()
        .list_wiki_pages(&owner, &repo, ListWikiPagesOptions::default())
        .await
        .expect("list wiki pages");
    assert_success_status(list_response.status);
    assert!(list.iter().any(|meta| meta.title == "Live-Test-Page"));

    let delete_response = client
        .repos()
        .delete_wiki_page(&owner, &repo, "Live-Test-Page")
        .await
        .expect("delete wiki page");
    assert_success_status(delete_response.status);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_topics_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-topic-repo")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let base_topics = vec!["rust-sdk".to_string()];
    let set_response = client
        .repos()
        .set_topics(&owner, &repo, base_topics.clone())
        .await
        .expect("set topics");
    assert_success_status(set_response.status);

    let (topics, list_response) = client
        .repos()
        .list_topics(&owner, &repo, ListRepoTopicsOptions::default())
        .await
        .expect("list topics");
    assert_success_status(list_response.status);
    assert!(topics.contains(&"rust-sdk".to_string()));

    let add_response = client
        .repos()
        .add_topic(&owner, &repo, "live-topic")
        .await
        .expect("add topic");
    assert_success_status(add_response.status);

    let (topics_after_add, _) = client
        .repos()
        .list_topics(&owner, &repo, ListRepoTopicsOptions::default())
        .await
        .expect("list topics after add");
    assert!(topics_after_add.contains(&"live-topic".to_string()));

    let delete_response = client
        .repos()
        .delete_topic(&owner, &repo, "rust-sdk")
        .await
        .expect("delete topic");
    assert_success_status(delete_response.status);

    cleanup.run_all().await;
}
