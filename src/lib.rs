//! Gitea API client for Rust.
//!
//! This crate provides a typed client for the Gitea API,
//! supporting both synchronous and asynchronous operations.
//!
//! # Quick Start
//!
//! ```ignore
//! use gitea_sdk::Client;
//!
//! let client = Client::builder("https://gitea.example.com")
//!     .token("your-token")
//!     .build()?;
//! ```

// Re-export serde macros for convenience.
pub use serde::{Deserialize, Serialize};

mod client;
mod error;
mod response;
mod pagination;
mod version;
mod internal;

pub mod auth;

// Empty module stubs for Phase 1 types/options/API.
#[allow(dead_code)]
mod types;
#[allow(dead_code)]
mod options;
#[allow(dead_code)]
mod api;

// Public API re-exports.
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use response::{PageLinks, Response};
pub use pagination::{ListOptions, PaginationOptions, QueryEncode};

#[cfg(test)]
mod tests {
    #[test]
    fn test_public_api_accessible() {
        let _: fn() -> crate::Client = || unimplemented!();
        let _: fn() -> crate::ClientBuilder<'static> = || unimplemented!();
        let _: fn() -> crate::Error = || unimplemented!();
        let _: fn() -> crate::Result<String> = || unimplemented!();
        let _: fn() -> crate::Response = || unimplemented!();
        let _: fn() -> crate::PageLinks = || unimplemented!();
        let _: fn() -> crate::ListOptions = || unimplemented!();
    }
}
