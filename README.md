# dusk-auth-core

<div align="center">

![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)
![Status](https://img.shields.io/badge/status-stable_core-green.svg?style=flat-square)
![Language](https://img.shields.io/badge/language-rust-orange.svg?style=flat-square)

**An opinionated, framework-agnostic authentication core that enforces correct session-based auth practices.**

[Design Philosophy](#design-philosophy) • [Core Invariants](#core-invariants) • [API Preview](#api-preview) • [Status](#status)

</div>

---

## Why This Library Exists

In modern web development, authentication is often implemented as a grab-bag of tools—JWTs, refresh tokens, middleware—without a cohesive underlying model. This "token-first" mentality leads to dangerous implementations:

- **JWTs as Sessions**: Treating stateless tokens as the source of truth, making revocation impossible.
- **Zombie Sessions**: "Logouts" that only delete cookies on the client, leaving tokens valid on the server.
- **Infinite Refresh**: Long-lived refresh tokens that never rotate, creating permanent backdoors.
- **Boolean Security**: Reducing complex auth states to simple `true/false` checks, hiding the difference between "expired" and "revoked".

**dusk-auth-core exists to close this gap.**

It encodes authentication best practices into **enforced domain invariants**. Instead of providing helpers to "build your own auth," it provides a strict state machine where common security mistakes are compile-time or runtime impossibilities.

---

## Core Invariants

This library is not a toolkit; it is a system. It enforces five non-negotiable rules:

### 1. The Token is Not the Session
JSON Web Tokens are merely *derived proofs* of identity. They are not the identity itself.
- **Rule**: All tokens must be validated against a server-side session.
- **Effect**: If the session is deleted, all tokens—even valid ones—immediately stop working.

### 2. Refresh Tokens Rotate Exactly Once
A refresh token is a single-use ticket.
- **Rule**: Every time a refresh token is used, it is invalidated.
- **Effect**: If a refresh token is used twice (e.g., by an attacker), the system detects a replay attack and revokes the entire session chain.

### 3. Logout is Destructive
Logout is a server-side state mutation, not a client-side cleanup.
- **Rule**: Calling `logout` explicitly transitions the session to a `Revoked` state.
- **Effect**: Revoked sessions cannot be "un-revoked" or refreshed. They are dead.

### 4. Auth Outcomes are Explicit
Authentication never returns a boolean.
- **Rule**: Validation returns a typed `Outcome` enum (`Valid`, `Expired`, `Revoked`, `Invalid`).
- **Effect**: Developers are forced to handle each case explicitly. Logging a "Revoked" token attempt is different from handling an "Expired" one.

### 5. Sessions Have a Real Lifecycle
Sessions are mortal.
- **Rule**: Every session has a hard creation time, expiry time, and optional revocation time.
- **Effect**: No session lives forever.

---

## API Preview

*Note: This design reflects the v1.0 core architecture.*

### 1. Login & Session Creation
We don't just "sign a token." We create a session, then issue tokens for it.

```rust
// Create a new session for a user (persistence is pluggable)
let session = Authenticator::login(&store, user_id).await?;

// Receive a typed bundle of tokens
let tokens = session.issue_tokens(); 
// tokens.access_token  (short-lived)
// tokens.refresh_token (long-lived, strict rotation)
```

### 2. Validation (The "Guard")
We validate the token *and* the underlying session state in one atomic operation.

```rust
let decision = Authenticator::validate(&store, access_token).await;

match decision {
    Outcome::Valid(session) => {
        // Safe to proceed. `session` contains up-to-date user context.
        process_request(session);
    },
    Outcome::Expired => {
        // Token is old. Client should attempt refresh.
        return http::Response::unauthorized("Token expired");
    },
    Outcome::Revoked(reason) => {
        // CRITICAL: Token signature is valid, but session was killed.
        // potentially malicious activity.
        security_log::alert("Attempt to use revoked session", reason);
        return http::Response::forbidden("Session revoked");
    },
    Outcome::Invalid => {
        // Bad signature or malformed token.
        return http::Response::unauthorized("Invalid token");
    }
}
```

### 3. Secure Refresh
Refresh checks the chain of custody.

```rust
// Rotates the refresh token and updates the session heartbeat
let new_tokens = Authenticator::refresh(&store, old_refresh_token).await?;
```

---

## Architecture

This library is **framework-agnostic**. It handles the logic; you handle the plumbing.

- **IO-Free Core**: No HTTP, no headers, no cookies.
- **Pluggable Storage**: Comes with an `InMemoryStore` for testing. Implement the `SessionStore` trait to persist to Redis, Postgres, or SQLx.
- **Zero Magic**: No hidden background threads or global state.

## Status

**Current Version**: `v1.0 Design Protocol`

This project is currently in the strict design phase. The architecture described above is the finalized specification for the Rust implementation.

## License

Licensed under the **MIT License**.

Contributions are welcome, provided they strictly adhere to the [Core Invariants](#core-invariants).
