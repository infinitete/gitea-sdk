# gitea-sdk

An async Rust SDK for the [Gitea](https://about.gitea.com/) API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gitea-sdk = "0.1.0"
```

Feature flags:

| Feature       | Description                        |
|---------------|------------------------------------|
| `rustls-tls`  | Use rustls for TLS *(default)*     |
| `native-tls`  | Use the system native TLS backend  |
| `stream`      | Enable streaming response support  |

## Quick Start

```rust
use gitea_sdk::Client;

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

More complete examples are in the `examples/` directory:

```bash
cargo run --example basic_usage
cargo run --example authentication
```

## Features

- **Async API client** built on reqwest with async/await
- **15 API modules** covering repos, issues, pulls, orgs, users, admin, hooks, notifications, actions, releases, settings, OAuth2, packages, miscellaneous, ActivityPub, and status
- **6 authentication methods**: token, basic auth, OTP, sudo, SSH certificate signing, and SSH public key signing
- **Pagination** via `ListOptions` for all list endpoints
- **Version checking** with automatic server version detection
- **Thread-safe** client that can be cloned and shared across tasks
- **Full serde support** for serialization and deserialization

## Authentication

```rust
use gitea_sdk::Client;

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

The client stores credentials in a thread-safe interior and supports swapping them at runtime via `set_token()`, `set_basic_auth()`, `set_otp()`, and `set_sudo()`.

## Pagination

List endpoints accept option structs that embed pagination parameters:

```rust
use gitea_sdk::ListOptions;
use gitea_sdk::options::repo::ListReposOptions;

let opts = ListReposOptions {
    list_options: ListOptions {
        page: Some(1),
        page_size: Some(50),
    },
};
let (repos, _) = client.repos().list_my_repos(opts).await?;
```

Set `page` to `Some(0)` to disable pagination and fetch all results at once.

## Minimum Rust Version

Requires Rust **1.88** or later.

## License

Licensed under the [MIT License](LICENSE).

## Links

- [Repository](https://gitea.com/gitea/go-sdk)
- [Changelog](CHANGELOG.md)
