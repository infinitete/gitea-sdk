use std::future::Future;

fn assert_bool_result_future<F>(_: F)
where
    F: Future<Output = gitea_sdk::Result<bool>>,
{
}

#[test]
fn users_follow_checks_return_bool_results() {
    let client = gitea_sdk::Client::builder("https://example.com")
        .gitea_version("")
        .build()
        .expect("builder should succeed");
    let users = client.users();

    assert_bool_result_future(users.is_following("target"));
    assert_bool_result_future(users.is_user_following("user", "target"));
}
