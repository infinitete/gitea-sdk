# gitea-sdk-rs

[![Crates.io](https://img.shields.io/crates/v/gitea-sdk-rs.svg)](https://crates.io/crates/gitea-sdk-rs)
[![docs.rs](https://docs.rs/gitea-sdk-rs/badge.svg)](https://docs.rs/gitea-sdk-rs)
[![CI](https://github.com/infinitete/gitea-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/infinitete/gitea-sdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-orange.svg)](https://blog.rust-lang.org/)

[中文文档](README_zh.md)

An async Rust SDK for the [Gitea](https://about.gitea.com/) REST API, with full coverage of repositories, issues, pull requests, organizations, users, admin operations, and more.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gitea-sdk-rs = "0.1.0"
```

### Feature Flags

| Feature       | Description                        |
|---------------|------------------------------------|
| `rustls-tls`  | Use rustls for TLS *(default)*     |
| `native-tls`  | Use the system native TLS backend  |
| `stream`      | Enable streaming response support  |

## Quick Start

```rust
use gitea_sdk_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("https://gitea.example.com")
        .token("your-token-here")
        .build()?;

    let (user, _) = client.users().get_my_info().await?;
    println!("Logged in as: {}", user.user_name);

    Ok(())
}
```

More examples are in the [`examples/`](examples/) directory:

```bash
cargo run --example basic_usage
cargo run --example authentication
```

## API Overview

### Repository Management

**[`ReposApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ReposApi.html)** — Full repository lifecycle and contents management.

- **CRUD** — `create_repo`, `get_repo`, `edit_repo`, `delete_repo`, `search_repos`
- **Branches** — `list_branches`, `create_branch`, `delete_branch`, `list_branch_protections`
- **Tags** — `list_tags`, `create_tag`, `delete_tag`, `list_tag_protections`
- **Files** — `get_contents`, `create_file`, `update_file`, `delete_file`, `get_raw_file`
- **Collaborators** — `list_collaborators`, `add_collaborator`, `get_collaborator_permission`
- **Commits** — `list_commits`, `get_single_commit`, `compare_commits`
- **Actions** — `list_action_secrets`, `list_action_variables`, `create_action_secret`
- **Wiki** — `create_wiki_page`, `get_wiki_page`, `edit_wiki_page`, `list_wiki_pages`
- **Misc** — `list_forks`, `create_fork`, `mirror_sync`, `transfer_repo`, `get_archive`

**[`ReleasesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ReleasesApi.html)** — Releases and attachments.

- `list`, `create`, `edit`, `delete`, `get_by_tag`, `list_attachments`, `create_attachment`

### Issues & Pull Requests

**[`IssuesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.IssuesApi.html)** — Full issue tracking with rich metadata.

- **CRUD** — `list_repo_issues`, `create_issue`, `edit_issue`, `delete_issue`
- **Comments** — `list_issue_comments`, `create_issue_comment`, `edit_issue_comment`
- **Labels** — `get_issue_labels`, `add_issue_labels`, `replace_issue_labels`
- **Milestones** — `list_repo_milestones`, `create_milestone`, `edit_milestone`
- **Reactions** — `post_issue_reaction`, `post_issue_comment_reaction`
- **Dependencies** — `list_issue_dependencies`, `create_issue_dependency`
- **Time Tracking** — `add_time`, `list_issue_tracked_times`, `list_my_tracked_times`
- **Subscriptions** — `list_issue_subscribers`, `add_issue_subscription`
- **Pins** — `list_repo_pinned_issues`, `pin_issue`, `unpin_issue`

**[`PullsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.PullsApi.html)** — Pull request workflow.

- **CRUD** — `list`, `get`, `create`, `edit`
- **Merge** — `merge`, `is_merged`, `patch`, `diff`
- **Reviews** — `list_reviews`, `create_review`, `submit_review`, `dismiss_review`
- **Files** — `list_commits`, `list_files`

### Users & Organizations

**[`UsersApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.UsersApi.html)** — User profiles, keys, and social features.

- **Profile** — `get`, `get_my_info`, `search`, `get_settings`, `update_settings`
- **Keys** — `list_public_keys`, `create_public_key`, `list_gpg_keys`, `create_gpg_key`
- **Social** — `follow`, `unfollow`, `list_followers`, `block_user`, `unblock_user`
- **Email** — `list_emails`, `add_email`, `delete_email`
- **Tokens** — `list_access_tokens`, `create_access_token`, `delete_access_token`

**[`OrgsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.OrgsApi.html)** — Organization and team management.

- **CRUD** — `list_orgs`, `create_org`, `edit_org`, `delete_org`
- **Teams** — `list_org_teams`, `create_team`, `add_team_member`, `add_team_repo`
- **Members** — `list_org_membership`, `set_public_org_membership`
- **Labels** — `list_org_labels`, `create_org_label`
- **Actions** — `list_org_action_secrets`, `list_org_action_variables`
- **Blocks** — `list_org_blocks`, `block_org_user`, `unblock_org_user`

### System & Administration

**[`AdminApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.AdminApi.html)** — Server administration (requires admin privileges).

- **Users** — `list_users`, `create_user`, `edit_user`, `delete_user`
- **Orgs** — `list_orgs`, `create_org_for_user`
- **Cron** — `list_cron_tasks`, `run_cron_task`
- **Hooks** — `list_hooks`, `create_hook`, `edit_hook`
- **Repos** — `list_unadopted_repos`, `adopt_unadopted_repo`
- **Badges** — `list_user_badges`, `add_user_badges`

**[`SettingsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.SettingsApi.html)** — `get_api_settings`, `get_repo_settings`, `get_ui_settings`

**[`MiscApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.MiscApi.html)** — `get_version`, `render_markdown`, `list_gitignore_templates`, `list_license_templates`, `get_signing_key_gpg`

### Other Modules

| Module | Description |
|--------|-------------|
| [`HooksApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.HooksApi.html) | Webhook management for repos, orgs, and users |
| [`NotificationsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.NotificationsApi.html) | Notification inbox and read status |
| [`ActionsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ActionsApi.html) | Gitea Actions workflow runs and jobs |
| [`PackagesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.PackagesApi.html) | Package registry management |
| [`Oauth2Api`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.Oauth2Api.html) | OAuth2 application management |
| [`StatusApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.StatusApi.html) | Commit status (CI/CD integration) |
| [`ActivityPubApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ActivityPubApi.html) | ActivityPub federation endpoints |

## Authentication

```rust
use gitea_sdk_rs::Client;

// Personal access token
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .build()?;

// Username and password
let client = Client::builder("https://gitea.example.com")
    .basic_auth("username", "password")
    .build()?;

// Token with two-factor OTP
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .otp("123456")
    .build()?;

// Act as another user (sudo)
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .sudo("target-username")
    .build()?;
```

The client is thread-safe and supports swapping credentials at runtime via `set_token()`, `set_basic_auth()`, `set_otp()`, and `set_sudo()`.

## Pagination

List endpoints accept option structs that embed pagination parameters:

```rust
use gitea_sdk_rs::ListOptions;
use gitea_sdk_rs::options::repo::ListReposOptions;

let opts = ListReposOptions {
    list_options: ListOptions {
        page: Some(1),
        page_size: Some(50),
    },
};
let (repos, _) = client.repos().list_my_repos(opts).await?;
```

Set `page` to `Some(0)` to disable pagination and fetch all results at once.

## Error Handling

All API methods return `Result<(T, Response)>` where `T` is the deserialized response body and `Response` contains HTTP status, headers, and pagination links.

```rust
use gitea_sdk_rs::{Client, Error};

async fn example(client: &Client) {
    match client.repos().get_repo("owner", "repo").await {
        Ok((repo, response)) => {
            println!("Repo: {}", repo.name);
            println!("Status: {}", response.status);
        }
        Err(Error::Http(status, msg)) => {
            eprintln!("HTTP {status}: {msg}");
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
```

## Minimum Rust Version

Requires Rust **1.88** or later (edition 2024).

## License

Licensed under the [MIT License](LICENSE).

## Links

- [API Documentation (docs.rs)](https://docs.rs/gitea-sdk-rs)
- [Repository (GitHub)](https://github.com/infinitete/gitea-sdk)
- [Changelog](CHANGELOG.md)
- [Gitea API Reference](https://gitea.com/api/swagger)
