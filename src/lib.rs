#![doc = include_str!("../DOCUMENTATION.md")]
//! dusk-auth-core-rust
//!
//! An opinionated, framework-agnostic authentication core.
pub mod auth;
pub mod session;
pub mod token;
pub mod decision;
pub mod error;
pub mod store;
pub mod time;

pub use auth::Authenticator;
pub use decision::AuthDecision;
pub use error::AuthError;
pub use session::{Session, SessionId};
pub use token::{AccessToken, RefreshToken};
pub use store::{SessionStore, InMemorySessionStore};