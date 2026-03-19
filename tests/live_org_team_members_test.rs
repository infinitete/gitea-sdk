// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk_rs::options::org::{CreateTeamOption, ListTeamMembersOptions};
use gitea_sdk_rs::types::enums::{AccessMode, RepoUnitType};

use live::{CleanupRegistry, create_org_fixture, live_client, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_team_members_flow() {
    let client = live_client();
    let env = load_live_env();
    let next_user = env
        .next_user_name
        .as_ref()
        .expect("missing GITEA_NEXT_USER_NAME in .env")
        .clone();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "live-org-members")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let team_name = unique_name("live-members-team");
    let (team, create_resp) = client
        .orgs()
        .create_team(
            &org_name,
            CreateTeamOption {
                name: team_name.clone(),
                description: Some("live fixtures for membership".to_string()),
                permission: Some(AccessMode::Write),
                can_create_org_repo: Some(false),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code],
            },
        )
        .await
        .expect("create team");
    assert_success_status(create_resp.status);

    let team_id = team.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_team(team_id).await;
    });

    let add_resp = client
        .orgs()
        .add_team_member(team_id, &next_user)
        .await
        .expect("add next user to team");
    assert_success_status(add_resp.status);

    let (members, members_resp) = client
        .orgs()
        .list_team_members(team_id, ListTeamMembersOptions::default())
        .await
        .expect("list team members");
    assert_success_status(members_resp.status);
    assert!(
        members.iter().any(|member| member.user_name == next_user),
        "expected second user among team members"
    );

    let (loaded_member, member_resp) = client
        .orgs()
        .get_team_member(team_id, &next_user)
        .await
        .expect("get team member");
    assert_success_status(member_resp.status);
    assert_eq!(loaded_member.user_name, next_user);

    let remove_resp = client
        .orgs()
        .remove_team_member(team_id, &next_user)
        .await
        .expect("remove team member");
    assert_success_status(remove_resp.status);

    let (members_after, after_resp) = client
        .orgs()
        .list_team_members(team_id, ListTeamMembersOptions::default())
        .await
        .expect("list team members after removal");
    assert_success_status(after_resp.status);
    assert!(
        members_after
            .iter()
            .all(|member| member.user_name != next_user),
        "second user should no longer be listed"
    );

    cleanup.run_all().await;
}
