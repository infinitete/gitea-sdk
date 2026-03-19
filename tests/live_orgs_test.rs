// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::Error;
use gitea_sdk::options::hook::{CreateHookOption, EditHookOption, ListHooksOptions};
use gitea_sdk::options::org::{
    CreateOrgLabelOption, CreateTeamOption, EditOrgLabelOption, EditTeamOption,
    ListOrgLabelsOptions, ListOrgsOptions, ListTeamRepositoriesOptions, ListTeamsOptions,
    SearchTeamsOptions,
};
use gitea_sdk::types::enums::{AccessMode, HookType, RepoUnitType};

use live::{
    CleanupRegistry, create_org_fixture, create_org_repo_fixture, live_client, load_live_env,
    unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_read_and_lifecycle() {
    let client = live_client();
    let env = live::load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "live-org-domain")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let (my_orgs, my_orgs_response) = client
        .orgs()
        .list_my_orgs(ListOrgsOptions::default())
        .await
        .expect("list my orgs");
    assert_success_status(my_orgs_response.status);
    assert!(my_orgs.iter().any(|org| org.user_name == org_name));

    let (user_orgs, user_orgs_response) = client
        .orgs()
        .list_user_orgs(&env.user_name, ListOrgsOptions::default())
        .await
        .expect("list user orgs");
    assert_success_status(user_orgs_response.status);
    assert!(user_orgs.iter().any(|org| org.user_name == org_name));

    let (loaded, get_response) = client.orgs().get_org(&org_name).await.expect("get org");
    assert_success_status(get_response.status);
    assert_eq!(loaded.user_name, org_name);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_team_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "live-team-org")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let team_name = unique_name("live-team");
    let (team, create_response) = client
        .orgs()
        .create_team(
            &org_name,
            CreateTeamOption {
                name: team_name.clone(),
                description: Some("live team fixture".to_string()),
                permission: Some(AccessMode::Write),
                can_create_org_repo: Some(false),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code],
            },
        )
        .await
        .expect("create team");
    assert_success_status(create_response.status);

    let team_id = team.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_team(team_id).await;
    });

    let (loaded, get_response) = client.orgs().get_team(team_id).await.expect("get team");
    assert_success_status(get_response.status);
    assert_eq!(loaded.name, team_name);

    let (teams, list_response) = client
        .orgs()
        .list_org_teams(&org_name, ListTeamsOptions::default())
        .await
        .expect("list org teams");
    assert_success_status(list_response.status);
    assert!(teams.iter().any(|item| item.id == team_id));

    cleanup.run_all().await;
}

fn org_hook_config(prefix: &str) -> std::collections::HashMap<String, String> {
    let mut config = std::collections::HashMap::new();
    config.insert(
        "url".to_string(),
        format!("https://example.com/hooks/{}", unique_name(prefix)),
    );
    config.insert("content_type".to_string(), "json".to_string());
    config
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_hooks_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "live-hooks-org")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let config = org_hook_config("org-hooks");
    let (hook, create_resp) = client
        .hooks()
        .create_org_hook(
            &org_name,
            CreateHookOption {
                hook_type: HookType::Gitea,
                config,
                events: vec!["push".to_string()],
                branch_filter: None,
                active: true,
                authorization_header: None,
            },
        )
        .await
        .expect("create org hook");
    assert_success_status(create_resp.status);

    let hook_id = hook.id;
    let cleanup_client = client.clone();
    let org_clone = org_name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .hooks()
            .delete_org_hook(&org_clone, hook_id)
            .await;
    });

    let (_, list_resp) = client
        .hooks()
        .list_org_hooks(&org_name, ListHooksOptions::default())
        .await
        .expect("list org hooks");
    assert_success_status(list_resp.status);

    let (_, get_resp) = client
        .hooks()
        .get_org_hook(&org_name, hook_id)
        .await
        .expect("get org hook");
    assert_success_status(get_resp.status);

    let edit_resp = client
        .hooks()
        .edit_org_hook(
            &org_name,
            hook_id,
            EditHookOption {
                active: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("edit org hook");
    assert_success_status(edit_resp.status);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_team_and_labels_workflow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "live-org-team")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();
    let team_name = unique_name("live-team");

    let (team, create_resp) = client
        .orgs()
        .create_team(
            &org_name,
            CreateTeamOption {
                name: team_name.clone(),
                description: Some("team for hooks coverage".to_string()),
                permission: Some(AccessMode::Write),
                can_create_org_repo: Some(false),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code],
            },
        )
        .await
        .expect("create team for org");
    assert_success_status(create_resp.status);

    let team_id = team.id;
    client
        .orgs()
        .add_team_member(team_id, &load_live_env().user_name)
        .await
        .expect("add self to team");
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_team(team_id).await;
    });

    let (orgs, orgs_resp) = client
        .orgs()
        .list_orgs(ListOrgsOptions::default())
        .await
        .expect("list orgs");
    assert_success_status(orgs_resp.status);
    assert!(orgs.iter().any(|entry| entry.user_name == org_name));

    match client
        .orgs()
        .list_my_teams(ListTeamsOptions::default())
        .await
    {
        Ok((my_teams, my_teams_resp)) => {
            assert_success_status(my_teams_resp.status);
            assert!(my_teams.iter().any(|entry| entry.id == team_id));
        }
        Err(Error::Api {
            status, message, ..
        }) => {
            assert!(status == 500 && message.contains("user does not exist"));
        }
        Err(other) => panic!("unexpected list_my_teams error: {other}"),
    }

    let (search_results, search_resp) = client
        .orgs()
        .search_org_teams(&org_name, SearchTeamsOptions::default())
        .await
        .expect("search org teams");
    assert_success_status(search_resp.status);
    assert!(search_results.iter().any(|entry| entry.id == team_id));

    let edit_resp = client
        .orgs()
        .edit_team(
            team_id,
            EditTeamOption {
                name: team_name.clone(),
                description: Some("updated team".to_string()),
                permission: Some(AccessMode::Admin),
                can_create_org_repo: Some(true),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code, RepoUnitType::Issues],
            },
        )
        .await
        .expect("edit team");
    assert_success_status(edit_resp.status);

    let repo_fixture = create_org_repo_fixture(&client, &mut cleanup, &org_name, "live-team-repo")
        .await
        .expect("create repo for team");
    let repo_name = repo_fixture.repository.name.clone();

    let link_resp = client
        .orgs()
        .add_team_repo(team_id, &org_name, &repo_name)
        .await
        .expect("link repo to team");
    assert_success_status(link_resp.status);

    let (team_repos, team_repos_resp) = client
        .orgs()
        .list_team_repositories(team_id, ListTeamRepositoriesOptions::default())
        .await
        .expect("list team repos");
    assert_success_status(team_repos_resp.status);
    assert!(team_repos.iter().any(|repo| repo.name == repo_name));

    let unlink_resp = client
        .orgs()
        .remove_team_repo(team_id, &org_name, &repo_name)
        .await
        .expect("unlink repo from team");
    assert_success_status(unlink_resp.status);

    let repo_side_fixture =
        create_org_repo_fixture(&client, &mut cleanup, &org_name, "live-repo-team-api")
            .await
            .expect("create repo for repo-team apis");
    let repo_side_name = repo_side_fixture.repository.name.clone();

    let repo_link_resp = client
        .repos()
        .add_repo_team(&org_name, &repo_side_name, &team_name)
        .await
        .expect("link repo team via repos api");
    assert_success_status(repo_link_resp.status);

    let (repo_teams, repo_teams_resp) = client
        .repos()
        .get_repo_teams(&org_name, &repo_side_name)
        .await
        .expect("get repo teams");
    assert_success_status(repo_teams_resp.status);
    assert!(repo_teams.iter().any(|team| team.name == team_name));

    let (linked_team, linked_team_resp) = client
        .repos()
        .check_repo_team(&org_name, &repo_side_name, &team_name)
        .await
        .expect("check linked repo team");
    assert_success_status(linked_team_resp.status);
    assert_eq!(
        linked_team.expect("linked team should exist").name,
        team_name
    );

    let repo_unlink_resp = client
        .repos()
        .remove_repo_team(&org_name, &repo_side_name, &team_name)
        .await
        .expect("unlink repo team via repos api");
    assert_success_status(repo_unlink_resp.status);

    let (missing_team, missing_team_resp) = client
        .repos()
        .check_repo_team(&org_name, &repo_side_name, &team_name)
        .await
        .expect("check repo team after removal");
    assert_eq!(missing_team_resp.status, 404);
    assert!(missing_team.is_none(), "team should no longer be linked");

    let label_name = unique_name("live-org-label");
    let (label, label_resp) = client
        .orgs()
        .create_org_label(
            &org_name,
            CreateOrgLabelOption {
                name: label_name.clone(),
                color: "00AA00".to_string(),
                description: Some("live org label".to_string()),
                exclusive: Some(false),
            },
        )
        .await
        .expect("create org label");
    assert_success_status(label_resp.status);

    let (labels, list_labels_resp) = client
        .orgs()
        .list_org_labels(&org_name, ListOrgLabelsOptions::default())
        .await
        .expect("list org labels");
    assert_success_status(list_labels_resp.status);
    assert!(labels.iter().any(|entry| entry.id == label.id));

    let (loaded_label, get_label_resp) = client
        .orgs()
        .get_org_label(&org_name, label.id)
        .await
        .expect("get org label");
    assert_success_status(get_label_resp.status);
    assert_eq!(loaded_label.name, label_name);

    let (edited_label, edit_label_resp) = client
        .orgs()
        .edit_org_label(
            &org_name,
            label.id,
            EditOrgLabelOption {
                name: Some(format!("{label_name}-updated")),
                color: Some("FFAA00".to_string()),
                description: Some("updated".to_string()),
                exclusive: Some(false),
            },
        )
        .await
        .expect("edit org label");
    assert_success_status(edit_label_resp.status);
    assert_eq!(edited_label.name, format!("{label_name}-updated"));

    let delete_label_resp = client
        .orgs()
        .delete_org_label(&org_name, label.id)
        .await
        .expect("delete org label");
    assert_success_status(delete_label_resp.status);

    cleanup.run_all().await;
}
