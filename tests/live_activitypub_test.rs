// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::Error;

use live::{CleanupRegistry, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn route_unavailable(err: &Error) -> Option<String> {
    match err {
        Error::Api {
            status, message, ..
        } if *status == 404 => Some(message.clone()),
        Error::UnknownApi { status, body } if *status == 404 => Some(body.clone()),
        _ => None,
    }
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_activitypub_person_followers_and_inbox() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();

    let (me, me_resp) = client
        .users()
        .get_my_info()
        .await
        .expect("get current user");
    assert_success_status(me_resp.status);

    match client.activitypub().get_person(me.id).await {
        Ok((person, person_resp)) => {
            assert_success_status(person_resp.status);
            assert_eq!(person.get("type").and_then(|v| v.as_str()), Some("Person"));
            assert_eq!(
                person.get("preferredUsername").and_then(|v| v.as_str()),
                Some(me.user_name.as_str())
            );
        }
        Err(err) => {
            if let Some(message) = route_unavailable(&err) {
                println!("[activitypub person] endpoint unavailable: {message}");
            } else {
                panic!("get activitypub person: {err}");
            }
        }
    }

    match client.activitypub().get_person_response(me.id).await {
        Ok((raw, raw_resp)) => {
            assert_success_status(raw_resp.status);
            let person: serde_json::Value =
                serde_json::from_slice(&raw).expect("parse raw activitypub person");
            assert_eq!(person.get("type").and_then(|v| v.as_str()), Some("Person"));
        }
        Err(err) => {
            if let Some(message) = route_unavailable(&err) {
                println!("[activitypub person raw] endpoint unavailable: {message}");
            } else {
                panic!("get raw activitypub person: {err}");
            }
        }
    }

    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-activitypub")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    match client.activitypub().get_followers(&owner, &repo).await {
        Ok((followers, followers_resp)) => {
            assert_success_status(followers_resp.status);
            assert!(
                followers.get("type").and_then(|v| v.as_str()).is_some(),
                "followers response should include an ActivityPub type"
            );
        }
        Err(err) => {
            if let Some(message) = route_unavailable(&err) {
                println!("[activitypub followers] endpoint unavailable: {message}");
            } else {
                panic!("get activitypub followers: {err}");
            }
        }
    }

    match client
        .activitypub()
        .send_inbox(
            me.id,
            serde_json::json!({
                "@context": "https://www.w3.org/ns/activitystreams",
                "type": "Follow",
                "actor": format!("{}/api/v1/activitypub/user-id/{}", client.base_url(), me.id),
                "object": format!("{}/api/v1/activitypub/user-id/{}", client.base_url(), me.id),
            }),
        )
        .await
    {
        Ok(resp) => assert_eq!(resp.status, 204),
        Err(Error::Api {
            status, message, ..
        }) => {
            println!("[activitypub inbox] endpoint blocked: status {status} message {message}");
        }
        Err(Error::UnknownApi { status, body }) => {
            println!("[activitypub inbox] endpoint blocked: status {status} body {body}");
        }
        Err(err) => panic!("send activitypub inbox: {err}"),
    }

    cleanup.run_all().await;
}
