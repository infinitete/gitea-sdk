// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::options::oauth2::CreateOauth2Option;

use live::{CleanupRegistry, live_client, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_oauth2_application_lifecycle() {
    let client = live_client();
    let env = load_live_env();
    let redirect = format!("http://{}:{}/oauth2-callback", env.host, env.http_port);
    let mut cleanup = CleanupRegistry::new();

    let opt = CreateOauth2Option {
        name: unique_name("live-oauth2"),
        confidential_client: true,
        redirect_uris: vec![redirect.clone()],
    };

    let (created, create_resp) = client
        .oauth2()
        .create_application(opt)
        .await
        .expect("create oauth2 application");
    assert_success_status(create_resp.status);

    let app_id = created.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.oauth2().delete_application(app_id).await;
    });

    let (fetched, get_resp) = client
        .oauth2()
        .get_application(app_id)
        .await
        .expect("get oauth2 application");
    assert_success_status(get_resp.status);
    assert_eq!(fetched.id, app_id);

    let update_name = format!("{}-updated", fetched.name);
    let update_opt = CreateOauth2Option {
        name: update_name.clone(),
        confidential_client: fetched.confidential_client,
        redirect_uris: vec![redirect.clone()],
    };
    let (updated, update_resp) = client
        .oauth2()
        .update_application(app_id, update_opt)
        .await
        .expect("update oauth2 application");
    assert_success_status(update_resp.status);
    assert_eq!(updated.name, update_name);

    let (apps, list_resp) = client
        .oauth2()
        .list_applications(Default::default())
        .await
        .expect("list oauth2 applications");
    assert_success_status(list_resp.status);
    assert!(apps.iter().any(|app| app.id == app_id));

    cleanup.run_all().await;
}
