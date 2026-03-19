// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::{Engine as _, engine::general_purpose};
use gitea_sdk_rs::Error;
use gitea_sdk_rs::options::org::{
    CreateOrgActionVariableOption, CreateSecretOption, CreateTeamOption, EditOrgOption,
    ListOrgActionSecretOption, ListOrgActionVariableOption, ListOrgActivityFeedsOptions,
    ListOrgBlocksOptions, ListOrgMembershipOption, ListTeamActivityFeedsOptions, ListTeamsOptions,
    UpdateOrgActionVariableOption,
};
use gitea_sdk_rs::options::user::UpdateUserAvatarOption;
use gitea_sdk_rs::types::enums::{AccessMode, RepoUnitType};

use live::{
    CleanupRegistry, build_basic_auth_client, create_org_fixture, create_org_repo_fixture,
    live_client, load_live_env,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

const ONE_PIXEL_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x04, 0x00, 0x00, 0x00, 0xB5, 0x1C, 0x0C,
    0x02, 0x00, 0x00, 0x00, 0x0B, 0x49, 0x44, 0x41, 0x54, 0x78, 0xDA, 0x63, 0xFC, 0xFF, 0x1F, 0x00,
    0x03, 0x03, 0x02, 0x00, 0xEF, 0xBF, 0x95, 0x51, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
    0xAE, 0x42, 0x60, 0x82,
];

fn short_name(prefix: &str) -> String {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis()
        % 100_000;
    format!("{prefix}{millis}")
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_membership_permissions_and_activity_flow() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "lorgx")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let (members, members_resp) = client
        .orgs()
        .list_org_membership(&org_name, ListOrgMembershipOption::default())
        .await
        .expect("list org membership");
    assert_success_status(members_resp.status);
    assert!(members.iter().any(|entry| entry.user_name == env.user_name));

    let (is_member, member_resp) = client
        .orgs()
        .check_org_membership(&org_name, &env.user_name)
        .await
        .expect("check org membership");
    assert_eq!(member_resp.status, 204);
    assert!(is_member);

    let (_public_before, public_before_resp) = client
        .orgs()
        .check_public_org_membership(&org_name, &env.user_name)
        .await
        .expect("check public org membership before");
    assert!(public_before_resp.status == 204 || public_before_resp.status == 404);

    let publicize_resp = client
        .orgs()
        .set_public_org_membership(&org_name, &env.user_name, true)
        .await
        .expect("publicize org membership");
    assert_eq!(publicize_resp.status, 204);

    let (is_public, public_resp) = client
        .orgs()
        .check_public_org_membership(&org_name, &env.user_name)
        .await
        .expect("check public org membership after");
    assert_eq!(public_resp.status, 204);
    assert!(is_public);

    let (public_members, public_members_resp) = client
        .orgs()
        .list_public_org_membership(&org_name, ListOrgMembershipOption::default())
        .await
        .expect("list public org membership");
    assert_success_status(public_members_resp.status);
    assert!(
        public_members
            .iter()
            .any(|entry| entry.user_name == env.user_name)
    );

    let conceal_resp = client
        .orgs()
        .set_public_org_membership(&org_name, &env.user_name, false)
        .await
        .expect("conceal org membership");
    assert_eq!(conceal_resp.status, 204);

    let edit_resp = client
        .orgs()
        .edit_org(
            &org_name,
            EditOrgOption {
                description: Some("live edited org".to_string()),
                location: Some("Shanghai".to_string()),
                website: Some("https://example.com/org-live".to_string()),
                ..EditOrgOption::default()
            },
        )
        .await
        .expect("edit org");
    assert_success_status(edit_resp.status);

    let (edited_org, get_org_resp) = client
        .orgs()
        .get_org(&org_name)
        .await
        .expect("get edited org");
    assert_success_status(get_org_resp.status);
    assert_eq!(edited_org.description, "live edited org");

    let (permissions, permissions_resp) = client
        .orgs()
        .get_org_permissions(&org_name, &env.user_name)
        .await
        .expect("get org permissions");
    assert_success_status(permissions_resp.status);
    assert!(permissions.can_read);
    assert!(permissions.is_owner || permissions.is_admin);

    let repo_fixture = create_org_repo_fixture(&client, &mut cleanup, &org_name, "lorga")
        .await
        .expect("create org repo fixture");
    let _repo_name = repo_fixture.repository.name.clone();

    let (org_feeds, org_feeds_resp) = client
        .orgs()
        .list_org_activity_feeds(&org_name, ListOrgActivityFeedsOptions::default())
        .await
        .expect("list org activity feeds");
    assert_success_status(org_feeds_resp.status);
    let _ = org_feeds;

    let (team, create_team_resp) = client
        .orgs()
        .create_team(
            &org_name,
            CreateTeamOption {
                name: short_name("tm"),
                description: Some("live activity team".to_string()),
                permission: Some(AccessMode::Write),
                can_create_org_repo: Some(false),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code],
            },
        )
        .await
        .expect("create team");
    assert_success_status(create_team_resp.status);
    let team_id = team.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_team(team_id).await;
    });

    let (teams, teams_resp) = client
        .orgs()
        .list_org_teams(&org_name, ListTeamsOptions::default())
        .await
        .expect("list org teams");
    assert_success_status(teams_resp.status);
    assert!(teams.iter().any(|entry| entry.id == team_id));

    let (team_feeds, team_feeds_resp) = client
        .orgs()
        .list_team_activity_feeds(team_id, ListTeamActivityFeedsOptions::default())
        .await
        .expect("list team activity feeds");
    assert_success_status(team_feeds_resp.status);
    let _ = team_feeds;

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_actions_blocks_and_avatar_flow() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "lorgy")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let variable_name = short_name("OV");
    let create_var_resp = client
        .orgs()
        .create_org_action_variable(
            &org_name,
            CreateOrgActionVariableOption {
                name: variable_name.clone(),
                value: "alpha".to_string(),
                description: Some("live org variable".to_string()),
            },
        )
        .await
        .expect("create org action variable");
    assert_success_status(create_var_resp.status);

    let (variables, variables_resp) = client
        .orgs()
        .list_org_action_variables(&org_name, ListOrgActionVariableOption::default())
        .await
        .expect("list org action variables");
    assert_success_status(variables_resp.status);
    assert!(variables.iter().any(|entry| entry.name == variable_name));

    let (variable, variable_resp) = client
        .orgs()
        .get_org_action_variable(&org_name, &variable_name)
        .await
        .expect("get org action variable");
    assert_success_status(variable_resp.status);
    assert_eq!(variable.name, variable_name);

    let update_var_resp = client
        .orgs()
        .update_org_action_variable(
            &org_name,
            &variable_name,
            UpdateOrgActionVariableOption {
                value: "beta".to_string(),
                description: Some("updated".to_string()),
            },
        )
        .await
        .expect("update org action variable");
    assert_success_status(update_var_resp.status);

    let (updated_variable, updated_variable_resp) = client
        .orgs()
        .get_org_action_variable(&org_name, &variable_name)
        .await
        .expect("get updated org action variable");
    assert_success_status(updated_variable_resp.status);
    assert_eq!(updated_variable.data, "beta");

    let secret_name = short_name("OS");
    let create_secret_resp = client
        .orgs()
        .create_org_action_secret(
            &org_name,
            CreateSecretOption {
                name: secret_name.clone(),
                data: "super-secret".to_string(),
                description: Some("live org secret".to_string()),
            },
        )
        .await
        .expect("create org action secret");
    assert_success_status(create_secret_resp.status);

    let (secrets, secrets_resp) = client
        .orgs()
        .list_org_action_secrets(&org_name, ListOrgActionSecretOption::default())
        .await
        .expect("list org action secrets");
    assert_success_status(secrets_resp.status);
    assert!(secrets.iter().any(|entry| entry.name == secret_name));

    let (blocks_before, blocks_before_resp) = client
        .orgs()
        .list_org_blocks(&org_name, ListOrgBlocksOptions::default())
        .await
        .expect("list org blocks before");
    assert_success_status(blocks_before_resp.status);
    assert!(
        !blocks_before
            .iter()
            .any(|entry| entry.user_name == env.next_user_name.clone().unwrap_or_default())
    );

    if let Some(next_user_name) = env.next_user_name.clone() {
        let (blocked_before, blocked_before_resp) = client
            .orgs()
            .check_org_block(&org_name, &next_user_name)
            .await
            .expect("check org block before");
        assert!(blocked_before_resp.status == 204 || blocked_before_resp.status == 404);
        assert!(!blocked_before);

        let block_resp = client
            .orgs()
            .block_org_user(&org_name, &next_user_name)
            .await
            .expect("block org user");
        assert_eq!(block_resp.status, 204);

        let (blocked_after, blocked_after_resp) = client
            .orgs()
            .check_org_block(&org_name, &next_user_name)
            .await
            .expect("check org block after");
        assert_eq!(blocked_after_resp.status, 204);
        assert!(blocked_after);

        let (blocks_after, blocks_after_resp) = client
            .orgs()
            .list_org_blocks(&org_name, ListOrgBlocksOptions::default())
            .await
            .expect("list org blocks after");
        assert_success_status(blocks_after_resp.status);
        assert!(
            blocks_after
                .iter()
                .any(|entry| entry.user_name == next_user_name)
        );

        let unblock_resp = client
            .orgs()
            .unblock_org_user(&org_name, &next_user_name)
            .await
            .expect("unblock org user");
        assert_eq!(unblock_resp.status, 204);
    }

    let avatar = UpdateUserAvatarOption {
        image: general_purpose::STANDARD.encode(ONE_PIXEL_PNG),
    };
    match client.orgs().update_org_avatar(&org_name, &avatar).await {
        Ok(update_avatar_resp) => {
            assert_eq!(update_avatar_resp.status, 204);
            let delete_avatar_resp = client
                .orgs()
                .delete_org_avatar(&org_name)
                .await
                .expect("delete org avatar");
            assert_eq!(delete_avatar_resp.status, 204);
        }
        Err(Error::UnknownApi { status, body }) => {
            panic!("update org avatar unexpected status {status}: {body}");
        }
        Err(other) => panic!("update org avatar: {other}"),
    }

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_my_teams_delete_membership_and_rename_flow() {
    let client = live_client();
    let env = load_live_env();
    let basic_client = build_basic_auth_client(env);
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_org_fixture(&client, &mut cleanup, "lorgz")
        .await
        .expect("create org fixture");
    let org_name = fixture.organization.user_name.clone();

    let (team, create_team_resp) = client
        .orgs()
        .create_team(
            &org_name,
            CreateTeamOption {
                name: short_name("tmx"),
                description: Some("live my teams team".to_string()),
                permission: Some(AccessMode::Write),
                can_create_org_repo: Some(false),
                includes_all_repositories: Some(false),
                units: vec![RepoUnitType::Code],
            },
        )
        .await
        .expect("create team");
    assert_success_status(create_team_resp.status);
    let team_id = team.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_team(team_id).await;
    });

    let add_self_resp = client
        .orgs()
        .add_team_member(team_id, &env.user_name)
        .await
        .expect("add self to team");
    assert_success_status(add_self_resp.status);

    let (my_teams, my_teams_resp) = basic_client
        .orgs()
        .list_my_teams(ListTeamsOptions::default())
        .await
        .expect("list my teams via basic auth");
    assert_success_status(my_teams_resp.status);
    assert!(my_teams.iter().any(|entry| entry.id == team_id));

    if let Some(next_user_name) = env.next_user_name.clone() {
        let add_next_resp = client
            .orgs()
            .add_team_member(team_id, &next_user_name)
            .await
            .expect("add next user to team");
        assert_success_status(add_next_resp.status);

        let (is_member_before_delete, member_before_delete_resp) = client
            .orgs()
            .check_org_membership(&org_name, &next_user_name)
            .await
            .expect("check next user membership before delete");
        assert_eq!(member_before_delete_resp.status, 204);
        assert!(is_member_before_delete);

        let delete_membership_resp = client
            .orgs()
            .delete_org_membership(&org_name, &next_user_name)
            .await
            .expect("delete org membership");
        assert_success_status(delete_membership_resp.status);

        let (is_member_after_delete, member_after_delete_resp) = client
            .orgs()
            .check_org_membership(&org_name, &next_user_name)
            .await
            .expect("check next user membership after delete");
        assert_eq!(member_after_delete_resp.status, 404);
        assert!(!is_member_after_delete);
    }

    cleanup.run_all().await;

    let original_name = short_name("rnorg");
    let (renamed_fixture, create_resp) = client
        .orgs()
        .create_org(gitea_sdk_rs::options::org::CreateOrgOption {
            name: original_name.clone(),
            full_name: None,
            email: None,
            description: Some("rename fixture".to_string()),
            website: None,
            location: None,
            visibility: None,
            repo_admin_change_team_access: None,
        })
        .await
        .expect("create org for rename");
    assert_success_status(create_resp.status);

    let new_name = short_name("rnnew");
    let rename_resp = client
        .orgs()
        .rename_org(
            &original_name,
            gitea_sdk_rs::options::org::RenameOrgOption {
                new_name: new_name.clone(),
            },
        )
        .await
        .expect("rename org");
    assert_eq!(rename_resp.status, 204);

    let (renamed_org, get_renamed_resp) = client
        .orgs()
        .get_org(&new_name)
        .await
        .expect("get renamed org");
    assert_success_status(get_renamed_resp.status);
    assert_eq!(renamed_org.user_name, new_name);

    let delete_renamed_resp = client
        .orgs()
        .delete_org(&new_name)
        .await
        .expect("delete renamed org");
    assert_success_status(delete_renamed_resp.status);

    let _ = renamed_fixture;
}
