//! Basic usage example for the Gitea SDK.
//!
//! Run with:
//!   cargo run --example basic_usage

use gitea_rs::options::repo::ListReposOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client pointing at your Gitea instance.
    let client = gitea_rs::Client::builder("https://gitea.example.com")
        .token("your-token-here")
        .build()?;

    // Get current user info.
    let (user, _) = client.users().get_my_info().await?;
    println!("Logged in as: {}", user.user_name);

    // List the authenticated user's repositories (first page, default limit).
    let (repos, _) = client
        .repos()
        .list_my_repos(ListReposOptions::default())
        .await?;
    for repo in repos {
        println!("  - {} ({})", repo.full_name, repo.html_url);
    }

    Ok(())
}
