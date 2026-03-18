// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::repo::ListStargazersOptions;

use live::{CleanupRegistry, create_repo_fixture, live_client, load_live_env};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_watch_and_star_flow() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-watch-star")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (watching_before, check_watch_resp) = client
        .repos()
        .check_repo_watch(&owner, &repo)
        .await
        .expect("check repo watch before");
    assert!(
        check_watch_resp.status == 200 || check_watch_resp.status == 404,
        "expected 200 or 404 for watch probe, got {}",
        check_watch_resp.status
    );

    if watching_before {
        let unwatch_resp = client
            .repos()
            .unwatch_repo(&owner, &repo)
            .await
            .expect("normalize repo watch state");
        assert_success_status(unwatch_resp.status);
    }

    let watch_resp = client
        .repos()
        .watch_repo(&owner, &repo)
        .await
        .expect("watch repo");
    assert_success_status(watch_resp.status);

    let (watching_after, check_watch_resp) = client
        .repos()
        .check_repo_watch(&owner, &repo)
        .await
        .expect("check repo watch after");
    assert_success_status(check_watch_resp.status);
    assert!(watching_after);

    let (my_watched, my_watched_resp) = client
        .repos()
        .get_my_watched_repos()
        .await
        .expect("get my watched repos");
    assert_success_status(my_watched_resp.status);
    assert!(my_watched.iter().any(|entry| entry.name == repo));

    let (user_watched, user_watched_resp) = client
        .repos()
        .get_watched_repos(&env.user_name)
        .await
        .expect("get watched repos");
    assert_success_status(user_watched_resp.status);
    assert!(user_watched.iter().any(|entry| entry.name == repo));

    let unwatch_resp = client
        .repos()
        .unwatch_repo(&owner, &repo)
        .await
        .expect("unwatch repo");
    assert_success_status(unwatch_resp.status);

    let (watching_final, check_watch_resp) = client
        .repos()
        .check_repo_watch(&owner, &repo)
        .await
        .expect("check repo watch final");
    assert!(
        check_watch_resp.status == 200 || check_watch_resp.status == 404,
        "expected 200 or 404 for watch probe, got {}",
        check_watch_resp.status
    );
    assert!(!watching_final);

    let (starring_before, starring_resp) = client
        .repos()
        .is_repo_starring(&owner, &repo)
        .await
        .expect("check repo starring before");
    assert!(
        starring_resp.status == 204 || starring_resp.status == 404,
        "expected 204 or 404 for star probe, got {}",
        starring_resp.status
    );

    if starring_before {
        let unstar_resp = client
            .repos()
            .unstar_repo(&owner, &repo)
            .await
            .expect("normalize repo star state");
        assert_success_status(unstar_resp.status);
    }

    let star_resp = client
        .repos()
        .star_repo(&owner, &repo)
        .await
        .expect("star repo");
    assert_success_status(star_resp.status);

    let (starring_after, starring_resp) = client
        .repos()
        .is_repo_starring(&owner, &repo)
        .await
        .expect("check repo starring after");
    assert_success_status(starring_resp.status);
    assert!(starring_after);

    let (my_starred, my_starred_resp) = client
        .repos()
        .get_my_starred_repos()
        .await
        .expect("get my starred repos");
    assert_success_status(my_starred_resp.status);
    assert!(my_starred.iter().any(|entry| entry.name == repo));

    let (user_starred, user_starred_resp) = client
        .repos()
        .get_starred_repos(&env.user_name)
        .await
        .expect("get starred repos");
    assert_success_status(user_starred_resp.status);
    assert!(user_starred.iter().any(|entry| entry.name == repo));

    let (stargazers, stargazers_resp) = client
        .repos()
        .list_stargazers(&owner, &repo, ListStargazersOptions::default())
        .await
        .expect("list stargazers");
    assert_success_status(stargazers_resp.status);
    assert!(
        stargazers
            .iter()
            .any(|entry| entry.user_name == env.user_name)
    );

    let unstar_resp = client
        .repos()
        .unstar_repo(&owner, &repo)
        .await
        .expect("unstar repo");
    assert_success_status(unstar_resp.status);

    let (starring_final, starring_resp) = client
        .repos()
        .is_repo_starring(&owner, &repo)
        .await
        .expect("check repo starring final");
    assert!(
        starring_resp.status == 204 || starring_resp.status == 404,
        "expected 204 or 404 for star probe, got {}",
        starring_resp.status
    );
    assert!(!starring_final);

    cleanup.run_all().await;
}
