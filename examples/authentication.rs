//! Authentication examples for the Gitea SDK.
//!
//! Run with:
//!   cargo run --example authentication

use gitea_sdk_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Token authentication ---
    let client = Client::builder("https://gitea.example.com")
        .token("your-token-here")
        .build()?;
    let (user, _) = client.users().get_my_info().await?;
    println!("Token auth: {}", user.user_name);

    // --- Basic authentication ---
    let client = Client::builder("https://gitea.example.com")
        .basic_auth("username", "password")
        .build()?;
    let (user, _) = client.users().get_my_info().await?;
    println!("Basic auth: {}", user.user_name);

    // --- Token + OTP (two-factor authentication) ---
    let client = Client::builder("https://gitea.example.com")
        .token("your-token-here")
        .otp("123456")
        .build()?;
    let (user, _) = client.users().get_my_info().await?;
    println!("Token+OTP: {}", user.user_name);

    // --- Token + Sudo (act as another user) ---
    let client = Client::builder("https://gitea.example.com")
        .token("your-token-here")
        .sudo("admin-username")
        .build()?;
    let (user, _) = client.users().get_my_info().await?;
    println!("Sudo: {}", user.user_name);

    Ok(())
}
