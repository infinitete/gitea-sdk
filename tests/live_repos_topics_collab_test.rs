// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::repo::{ListCollaboratorsOptions, ListRepoTopicsOptions};

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_topics_and_collab_reads() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-topic-collab")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (topics_before, list_topics_resp) = client
        .repos()
        .list_topics(&owner, &repo, ListRepoTopicsOptions::default())
        .await
        .expect("list topics before");
    assert_success_status(list_topics_resp.status);
    assert!(topics_before.is_empty());

    let topic_a = unique_name("topic-a").replace('_', "-");
    let topic_b = unique_name("topic-b").replace('_', "-");
    let set_topics_resp = client
        .repos()
        .set_topics(&owner, &repo, vec![topic_a.clone()])
        .await
        .expect("set topics");
    assert_success_status(set_topics_resp.status);

    let add_topic_resp = client
        .repos()
        .add_topic(&owner, &repo, &topic_b)
        .await
        .expect("add topic");
    assert_success_status(add_topic_resp.status);

    let (topics_after_add, list_topics_resp) = client
        .repos()
        .list_topics(&owner, &repo, ListRepoTopicsOptions::default())
        .await
        .expect("list topics after add");
    assert_success_status(list_topics_resp.status);
    assert!(topics_after_add.iter().any(|topic| topic == &topic_a));
    assert!(topics_after_add.iter().any(|topic| topic == &topic_b));

    let delete_topic_resp = client
        .repos()
        .delete_topic(&owner, &repo, &topic_b)
        .await
        .expect("delete topic");
    assert_success_status(delete_topic_resp.status);

    let (topics_after_delete, list_topics_resp) = client
        .repos()
        .list_topics(&owner, &repo, ListRepoTopicsOptions::default())
        .await
        .expect("list topics after delete");
    assert_success_status(list_topics_resp.status);
    assert!(topics_after_delete.iter().any(|topic| topic == &topic_a));
    assert!(!topics_after_delete.iter().any(|topic| topic == &topic_b));

    let (collaborators, collaborators_resp) = client
        .repos()
        .list_collaborators(&owner, &repo, ListCollaboratorsOptions::default())
        .await
        .expect("list collaborators");
    assert_success_status(collaborators_resp.status);
    let _ = collaborators;

    let fake_user = unique_name("missing-user");
    let (is_collaborator, collab_resp) = client
        .repos()
        .is_collaborator(&owner, &repo, &fake_user)
        .await
        .expect("probe missing collaborator");
    assert!(collab_resp.status >= 200);
    assert!(!is_collaborator);

    let (permission, permission_resp) = client
        .repos()
        .get_collaborator_permission(&owner, &repo, &fake_user)
        .await
        .expect("probe missing collaborator permission");
    assert!(permission_resp.status == 200 || permission_resp.status == 404);
    assert!(permission.is_none());

    let (reviewers, reviewers_resp) = client
        .repos()
        .get_reviewers(&owner, &repo)
        .await
        .expect("get reviewers");
    assert_success_status(reviewers_resp.status);
    let _ = reviewers;

    let (assignees, assignees_resp) = client
        .repos()
        .get_assignees(&owner, &repo)
        .await
        .expect("get assignees");
    assert_success_status(assignees_resp.status);
    assert!(!assignees.is_empty());

    cleanup.run_all().await;
}
