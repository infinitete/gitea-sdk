//! Gitea API client for Rust.

/// Re-export for convenience.
pub use serde::{Deserialize, Serialize};

mod error;
pub use error::{Error, Result};

mod response;
pub use response::{PageLinks, Response};

mod pagination;
pub use pagination::{ListOptions, PaginationOptions, QueryEncode};

mod client;
pub use client::{Client, ClientBuilder};

mod internal;
mod version;
