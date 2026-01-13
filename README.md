

# dusk-auth-core-rust

<div align="center">

![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)
![Status](https://img.shields.io/badge/status-v1.0_core_complete-green.svg?style=flat-square)
![Language](https://img.shields.io/badge/language-rust-orange.svg?style=flat-square)

**A synchronous, framework-agnostic authentication core that enforces correct session-based auth practices.**

[Design Philosophy](#design-philosophy) • [Core Invariants](#core-invariants) • [How to Use](#how-to-use) • [Architecture](#architecture)

📖 **[Read the Full Documentation & Manifesto](./DOCUMENTATION.md)**

</div>

---

## Design Philosophy

Modern authentication systems often suffer from a *token-first* mindset: JWTs are treated as sessions, refresh tokens never rotate, and logout is reduced to deleting client-side state.

These approaches are convenient—but dangerous.

**dusk-auth-core-rust takes the opposite approach.**

It is a **pure domain engine** that enforces authentication correctness through **explicit session state and non-negotiable invariants**. It does not handle HTTP, async IO, JWT encoding, or databases. Instead, it provides a strict and test-proven core that other layers must respect.

This crate is intentionally:

* Synchronous
* IO-free
* Framework-agnostic
* Deterministic
* Opinionated

---

## Why This Library Exists

Common authentication failures include:

* **JWTs as Sessions**
  Stateless tokens treated as the source of truth, making revocation impossible.

* **Zombie Logouts**
  “Logout” that only clears cookies while server-side state remains valid.

* **Infinite Refresh Tokens**
  Long-lived refresh tokens that never rotate, enabling permanent access if stolen.

* **Boolean Auth**
  Reducing authentication to `true / false`, hiding critical distinctions like *expired vs revoked*.

**dusk-auth-core-rust encodes the correct model directly into code.**

If a mistake is common, this library makes it hard—or impossible—to implement.

---

## Core Invariants

This library is not a toolkit. It is a system with enforced rules.

### 1. The Token Is Not the Session

Tokens are derived proofs, not the source of truth.

* All validation is performed against **server-side session state**
* If the session is gone or revoked, *all tokens immediately stop working*

---

### 2. Refresh Tokens Rotate Exactly Once

Refresh tokens are single-use credentials.

* Every successful refresh rotates the token
* Any reuse is detected deterministically
* Reuse automatically **revokes the entire session**

---

### 3. Logout Is Destructive

Logout is a server-side state transition.

* Calling logout revokes the session permanently
* Revoked sessions cannot be refreshed or revalidated

---

### 4. Auth Outcomes Are Explicit

Authentication never returns a boolean.

Validation produces a typed decision:

* `Valid`
* `Expired`
* `Revoked`
* `Invalid`

This forces correct handling and meaningful security responses.

---

### 5. Sessions Have a Real Lifecycle

Sessions are mortal.

* Creation time
* Expiry time
* Optional revocation time

No session lives forever.

---

## How to Use

`dusk-auth-core-rust` is used **inside your application**, not as a standalone service.

### 1. Provide a Session Store

You choose how sessions are persisted by implementing `SessionStore`.

```rust
let store = InMemorySessionStore::new();
let auth = Authenticator::new(store);
```

(In production, this would be backed by a database or Redis.)

---

### 2. Validate Access Tokens

At request boundaries, validate tokens **and session state together**.

```rust
let decision = auth.validate_access_token(&access_token, now);

match decision {
    AuthDecision::Valid(session) => {
        // Safe to proceed
    }
    AuthDecision::Expired => {
        // Client may attempt refresh
    }
    AuthDecision::Revoked => {
        // Security event: token was valid but session was killed
    }
    AuthDecision::Invalid => {
        // Malformed or unknown token
    }
}
```

---

### 3. Refresh Tokens Securely

Refresh enforces rotation and reuse detection.

```rust
let (new_access, new_refresh) =
    auth.refresh_session(&refresh_token, now)?;
```

If a refresh token is reused:

* The session is revoked
* All tokens become invalid
* A `RefreshTokenReused` error is returned

---

### 4. Logout (Server-Side)

Logout is explicit and destructive.

```rust
auth.revoke_session(&session_id);
```

After this:

* Validation returns `Revoked`
* Refresh is impossible

---

## Architecture

**This crate is the core, not the whole system.**

* **No HTTP**
* **No async**
* **No JWT encoding**
* **No databases**
* **No framework dependencies**

Those concerns belong in **adapter crates** built on top of this core.

Typical layering:

```
dusk-auth-core-rust        ← this crate (rules & invariants)
dusk-auth-adapter-*  ← HTTP / async / framework glue
application code     ← routing, storage, identity
```

---

## Status

**Current Version**: `v1.0 (core complete)`

The core authentication engine is complete and fully covered by invariant tests.
Future work will focus on adapters and integrations, not changes to the core logic.

---

## License

Licensed under the **MIT License**.

Contributions are welcome **only if they preserve the core invariants**.

---


