# Documentation & Design Manifesto

> **"I built this not just to be used, but to be read."**

`dusk-auth-core-rust` is an educational implementation of a secure authentication state machine. While it is production-ready code, its primary purpose is to demonstrate **correctness** in a domain often plagued by shortcuts and "magic" frameworks.

This document explains the *why* and *how* of the library, serving as both a manual and a lesson in authentication design.

---

## 1. The Vision

Authentication is often taught as "how to set up a JWT middleware" or "how to use a library." Rarely is it taught as a **concept**.

This causes developers to learn *mechanisms* (tokens, cookies, headers) without understanding the *models* (sessions, identity, trust).

**My goal is to prevent developers from taking a detour.**
I want to show you a robust, defensible method for handling sessions that avoids common pitfalls. It is not the *only* way, but it is a *good* way—one that prioritizes security and explicitness over convenience.

---

## 2. The Core Philosophy

### "The Session is the Source of Truth"
Many modern apps treat the **JWT** as the session. This is a mistake.
If a token contains everything needed to validate itself, you have lost control. You cannot revoke it. You cannot see who is logged in.

In `dusk-auth-core`, the **Session** (stored in your database/memory) is the only thing that matters.
- Tokens are just **pointers** to the session.
- If the session is dead, the token is useless.

### "No Magic, No Async, No IO"
You will notice this library does not:
- Connect to a database
- Speak HTTP
- Use `async/await`

**Why?**
Because those are *application* details. Core business logic should be pure. By keeping IO out of the core, we make the logic verifiable, testable, and theoretically portable to any system—even embedded ones.

---

## 3. How It Works (The State Machine)

The library implements a strict state machine for a user session.

### The Actors

#### `Session`
The central object. It has:
- `id`: The unique handle.
- `expires_at`: When it dies naturally.
- `revoked_at`: When it was killed explicitly (logout).
- `current_refresh_token_id`: The **only** valid refresh token allowed to touch this session.

#### `Authenticator`
The engine. It takes a **Store** (where sessions live) and performs operations.

### The Lifecycle

#### A. Login (Creation)
**You** are responsibly for verifying credentials (password, MFA).
Once you trust the user, you construct a `Session`.
* *Design Note*: There is no `Authenticator::login` function. This is intentional. I don't know how your user logs in. I only care about managing the session *after* they satisfy you.

#### B. Validation (`validate_access_token`)
When a user makes a request with an Access Token:
1. We parse the Session ID from the token.
2. We load the `Session` from the store.
3. We check: **Is it revoked?** (Security check)
4. We check: **Is it expired?** (Time check)

If any check fails, the result is an explicit `AuthDecision` (`Revoked`, `Expired`). We never return a boolean `false`. We tell you *why*.

#### C. Refresh (`refresh_session`)
This is where the magic happens.
Refresh tokens are **single-use**.
1. Client sends `RefreshToken`.
2. We check if the token matches `session.current_refresh_token_id`.
3. **If it matches**: We rotate it. We create a NEW refresh token, save it to the session, and give it to the user.
4. **If it does NOT match**: This is a **theft attempt**. Someone is trying to use an old ticket. We immediately **revoke** the session, locking out both the attacker and the real user.

#### D. Logout (`revoke_session`)
We set `revoked_at` to `now`.
We do NOT delete the session immediately.
* *Why?* Keeping a record of revoked sessions allows for auditing ("User X logged out at Time Y").

---

## 4. API Reference

### `Authenticator`

#### `new(store: S) -> self`
Creates the engine. You must provide a struct that implements `SessionStore`.

#### `validate_access_token(token, now) -> AuthDecision`
The workhorse. Call this on every protected request.
* Returns `Valid(Session)` if you are good to go.
* Returns `Revoked` if the user is banned or logged out.

#### `refresh_session(refresh_token, now) -> Result<(AccessToken, RefreshToken), AuthError>`
Exchanges an old credential for a fresh pair.
* **Critical**: If this returns `RefreshTokenReused`, you should alert your security team. It means a token was stolen.

#### `revoke_session(session_id)`
The kill switch. Idempotent (can be called multiple times).

---

## 5. FAQ

**Q: Why do I have to construct `Session` manually?**
A: Because "Logging In" is an application concern. You might need to add custom fields, check IP addresses, or trigger webhooks. My library shouldn't get in your way.

**Q: Why doesn't it handle JWT signing?**
A: Because signing is a *serialization* detail. This library manages the *lifecycle*. You can wrap these `AccessToken` structs in JWTs, PASETOs, or cookies.

**Q: Why is `revoked_at` an Option?**
A: To distinguish between "died of old age" (`expires_at`) and "was murdered" (`revoked_at`). This distinction matters for security auditing.

---

## 6. Installation & Usage Guide

To use `dusk-auth-core-rust` in your project:

### Step 1: Add Dependency

Add the library to your `Cargo.toml`. Since this is a core library, you probably also want `uuid` or a similar crate to generate IDs.

```toml
[dependencies]
dusk-auth-core-rust = "0.1.0"  # Check crates.io used latest version
```

### Step 2: Implement Storage

You must tell the library where to keep sessions. Create a struct that wraps your database (Postgres, Redis, whatever) and implement `SessionStore`.

```rust,ignore
use dusk_auth_core_rust::{SessionStore, Session, SessionId, Timestamp};

struct MyDatabase {
    // db_pool: ...
}

impl SessionStore for MyDatabase {
    fn load(&self, id: &SessionId) -> Option<Session> {
        // Fetch from DB, map to Session struct
        None
    }

    fn save(&self, session: Session) {
        // Upsert session into DB
    }

    fn revoke(&self, id: &SessionId) {
        // Update `revoked_at` column in DB
    }
}
```

### Step 3: Initialize & Use

Wire it up in your application startup.

```rust,ignore
let store = MyDatabase::new();
let auth = Authenticator::new(store);

// Use `auth` in your request handlers
```

---

## Final Words

If you are a new developer, I hope this codebase shows you that authentication isn't checking `if token.isValid()`. It is a lifecycle of trust.

Trust is created, maintained, checked, and eventually revoked.

Treat your sessions with respect, and they will keep your users safe.
