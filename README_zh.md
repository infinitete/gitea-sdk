# gitea-sdk-rs

[![Crates.io](https://img.shields.io/crates/v/gitea-sdk-rs.svg)](https://crates.io/crates/gitea-sdk-rs)
[![docs.rs](https://docs.rs/gitea-sdk-rs/badge.svg)](https://docs.rs/gitea-sdk-rs)
[![CI](https://github.com/infinitete/gitea-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/infinitete/gitea-sdk/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.88-orange.svg)](https://blog.rust-lang.org/)

[English](README.md)

异步 Rust SDK，用于访问 [Gitea](https://about.gitea.com/) REST API，全面覆盖仓库、Issue、Pull Request、组织、用户、管理员操作等功能。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
gitea-sdk-rs = "0.1.0"
```

### Feature Flags

| Feature       | 说明                          |
|---------------|-------------------------------|
| `rustls-tls`  | 使用 rustls 作为 TLS 后端（默认）|
| `native-tls`  | 使用系统原生 TLS 后端           |
| `stream`      | 启用流式响应支持                |

## 快速开始

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

更多示例请参考 [`examples/`](examples/) 目录：

```bash
cargo run --example basic_usage
cargo run --example authentication
```

## API 概览

### 仓库管理

**[`ReposApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ReposApi.html)** — 仓库全生命周期及内容管理。

- **增删改查** — `create_repo`, `get_repo`, `edit_repo`, `delete_repo`, `search_repos`
- **分支** — `list_branches`, `create_branch`, `delete_branch`, `list_branch_protections`
- **标签** — `list_tags`, `create_tag`, `delete_tag`, `list_tag_protections`
- **文件** — `get_contents`, `create_file`, `update_file`, `delete_file`, `get_raw_file`
- **协作者** — `list_collaborators`, `add_collaborator`, `get_collaborator_permission`
- **提交** — `list_commits`, `get_single_commit`, `compare_commits`
- **Actions** — `list_action_secrets`, `list_action_variables`, `create_action_secret`
- **Wiki** — `create_wiki_page`, `get_wiki_page`, `edit_wiki_page`, `list_wiki_pages`
- **其他** — `list_forks`, `create_fork`, `mirror_sync`, `transfer_repo`, `get_archive`

**[`ReleasesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ReleasesApi.html)** — Release 及附件管理。

- `list`, `create`, `edit`, `delete`, `get_by_tag`, `list_attachments`, `create_attachment`

### Issue 与 Pull Request

**[`IssuesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.IssuesApi.html)** — 完整的 Issue 跟踪与丰富的元数据支持。

- **增删改查** — `list_repo_issues`, `create_issue`, `edit_issue`, `delete_issue`
- **评论** — `list_issue_comments`, `create_issue_comment`, `edit_issue_comment`
- **标签** — `get_issue_labels`, `add_issue_labels`, `replace_issue_labels`
- **里程碑** — `list_repo_milestones`, `create_milestone`, `edit_milestone`
- **Reaction** — `post_issue_reaction`, `post_issue_comment_reaction`
- **依赖关系** — `list_issue_dependencies`, `create_issue_dependency`
- **工时追踪** — `add_time`, `list_issue_tracked_times`, `list_my_tracked_times`
- **订阅** — `list_issue_subscribers`, `add_issue_subscription`
- **置顶** — `list_repo_pinned_issues`, `pin_issue`, `unpin_issue`

**[`PullsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.PullsApi.html)** — Pull Request 工作流。

- **增删改查** — `list`, `get`, `create`, `edit`
- **合并** — `merge`, `is_merged`, `patch`, `diff`
- **审查** — `list_reviews`, `create_review`, `submit_review`, `dismiss_review`
- **文件** — `list_commits`, `list_files`

### 用户与组织

**[`UsersApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.UsersApi.html)** — 用户资料、密钥及社交功能。

- **资料** — `get`, `get_my_info`, `search`, `get_settings`, `update_settings`
- **密钥** — `list_public_keys`, `create_public_key`, `list_gpg_keys`, `create_gpg_key`
- **社交** — `follow`, `unfollow`, `list_followers`, `block_user`, `unblock_user`
- **邮箱** — `list_emails`, `add_email`, `delete_email`
- **令牌** — `list_access_tokens`, `create_access_token`, `delete_access_token`

**[`OrgsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.OrgsApi.html)** — 组织与团队管理。

- **增删改查** — `list_orgs`, `create_org`, `edit_org`, `delete_org`
- **团队** — `list_org_teams`, `create_team`, `add_team_member`, `add_team_repo`
- **成员** — `list_org_membership`, `set_public_org_membership`
- **标签** — `list_org_labels`, `create_org_label`
- **Actions** — `list_org_action_secrets`, `list_org_action_variables`
- **屏蔽** — `list_org_blocks`, `block_org_user`, `unblock_org_user`

### 系统与管理

**[`AdminApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.AdminApi.html)** — 服务器管理（需要管理员权限）。

- **用户** — `list_users`, `create_user`, `edit_user`, `delete_user`
- **组织** — `list_orgs`, `create_org_for_user`
- **定时任务** — `list_cron_tasks`, `run_cron_task`
- **Webhook** — `list_hooks`, `create_hook`, `edit_hook`
- **仓库** — `list_unadopted_repos`, `adopt_unadopted_repo`
- **徽章** — `list_user_badges`, `add_user_badges`

**[`SettingsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.SettingsApi.html)** — `get_api_settings`, `get_repo_settings`, `get_ui_settings`

**[`MiscApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.MiscApi.html)** — `get_version`, `render_markdown`, `list_gitignore_templates`, `list_license_templates`, `get_signing_key_gpg`

### 其他模块

| 模块 | 说明 |
|------|------|
| [`HooksApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.HooksApi.html) | 仓库、组织和用户的 Webhook 管理 |
| [`NotificationsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.NotificationsApi.html) | 通知收件箱和已读状态 |
| [`ActionsApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ActionsApi.html) | Gitea Actions 工作流运行和任务 |
| [`PackagesApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.PackagesApi.html) | 包注册表管理 |
| [`Oauth2Api`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.Oauth2Api.html) | OAuth2 应用管理 |
| [`StatusApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.StatusApi.html) | 提交状态（CI/CD 集成） |
| [`ActivityPubApi`](https://docs.rs/gitea-sdk-rs/latest/gitea_sdk_rs/api/struct.ActivityPubApi.html) | ActivityPub 联邦协议端点 |

## 认证方式

```rust
use gitea_sdk_rs::Client;

// 个人访问令牌
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .build()?;

// 用户名和密码
let client = Client::builder("https://gitea.example.com")
    .basic_auth("username", "password")
    .build()?;

// 令牌 + 两步验证 OTP
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .otp("123456")
    .build()?;

// 以其他用户身份操作（sudo）
let client = Client::builder("https://gitea.example.com")
    .token("your-token-here")
    .sudo("target-username")
    .build()?;
```

客户端是线程安全的，支持在运行时通过 `set_token()`、`set_basic_auth()`、`set_otp()` 和 `set_sudo()` 动态切换凭据。

## 分页

列表接口通过内嵌分页参数的选项结构体进行分页：

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

将 `page` 设为 `Some(0)` 可禁用分页，一次获取所有结果。

## 错误处理

所有 API 方法返回 `Result<(T, Response)>`，其中 `T` 是反序列化后的响应体，`Response` 包含 HTTP 状态码、响应头和分页链接。

```rust
use gitea_sdk_rs::{Client, Error};

async fn example(client: &Client) {
    match client.repos().get_repo("owner", "repo").await {
        Ok((repo, response)) => {
            println!("仓库: {}", repo.name);
            println!("状态码: {}", response.status);
        }
        Err(Error::Http(status, msg)) => {
            eprintln!("HTTP {status}: {msg}");
        }
        Err(e) => {
            eprintln!("错误: {e}");
        }
    }
}
```

## 最低 Rust 版本

需要 Rust **1.88** 或更高版本（edition 2024）。

## 许可证

本项目使用 [MIT 许可证](LICENSE)。

## 链接

- [API 文档 (docs.rs)](https://docs.rs/gitea-sdk-rs)
- [代码仓库 (GitHub)](https://github.com/infinitete/gitea-sdk)
- [变更日志](CHANGELOG.md)
- [Gitea API 参考](https://gitea.com/api/swagger)
