# dusk-auth-core-rust — Project Plan

## Purpose

`dusk-auth-core-rust` is an opinionated, framework-agnostic authentication core written in Rust.

Its goal is not to be a drop-in authentication solution, but to **encode correct authentication practices into enforceable rules**, so that common mistakes are difficult or impossible to make by design.

The library is intended to be:
- Educational (by structure, not tutorials)
- Reusable (via a clean public API)
- Honest about scope and trade-offs


## Core Problem Being Solved

Authentication is often implemented as a collection of loosely connected techniques (JWTs, refresh tokens, middleware) without a clear underlying model.

This leads to:
- JWTs being treated as “the session”
- Logout that does not actually revoke anything
- Refresh tokens reused indefinitely
- No auditability or explicit failure states
- Boolean auth checks (`true` / `false`)

While best practices exist, they are:
- Fragmented
- Optional
- Easy to misapply

`dusk-auth-core` encodes these best practices into **domain models and invariants** that are enforced by the library itself.



## Core Principles (Invariants)

These rules are **non-negotiable** and enforced by design.

### 1. JWT is not the session
- Tokens are derived artifacts
- Server-side sessions are the source of truth
- No session → no valid token

### 2. Refresh tokens rotate exactly once
- Each refresh invalidates the previous refresh token
- Reuse of a refresh token is detectable
- Reuse may result in session revocation

### 3. Logout revokes server-side state
- Logout mutates session state
- Revoked sessions cannot be validated or refreshed
- Client-only logout is impossible

### 4. Authentication outcomes are explicit
- No boolean auth results
- Validation returns a typed decision (e.g. valid, expired, revoked)

### 5. Sessions have a real lifecycle
- Sessions have creation and expiry times
- Sessions can be revoked
- Expired or revoked sessions cannot be resurrected



## High-Level Conceptual Model

### Sessions
- Represent authenticated server-side state
- Are the source of truth
- Have identity and lifecycle

### Tokens
- Represent proof of a session
- Are short-lived and replaceable
- Are validated against session state

**Sessions define whether authentication exists.  
Tokens define how authentication is proven.**



## Framework-Agnostic Design

The library:
- Does NOT depend on HTTP frameworks
- Does NOT handle cookies, headers, or requests
- Does NOT store sessions directly
- Does NOT choose databases or infrastructure

Applications handle IO and transport.  
`dusk-auth-core` handles **decisions and rules**.



## Storage Model

Session storage is **pluggable**.

The library defines a storage contract that applications must implement.
This allows sessions to be stored in:
- Memory
- SQL databases
- Key-value stores
- Custom backends

An in-memory session store is provided for:
- Learning
- Testing
- Examples

The library enforces **what must happen to sessions**, not **where they are stored**.


## Conceptual Public API

### Core Types
- `SessionId`
- `Session`
- `AccessToken`
- `RefreshToken`
- `AuthDecision`
- `AuthError`

### Storage Boundary
- `SessionStore` trait (pluggable)
- `InMemorySessionStore` (reference implementation)

### Core Engine
- `Authenticator<S: SessionStore>`

### Public Operations
- Create session (login)
- Validate access token
- Refresh session (with rotation)
- Revoke session (logout)

All operations:
- Take explicit inputs
- Return explicit outputs
- Avoid side effects beyond session state



## What This Library Does NOT Do

The following are explicitly out of scope:
- HTTP framework integration
- Database drivers
- OAuth providers
- Password hashing
- Cookie or header handling
- UI or frontend logic

These concerns belong to the application layer.



## Crate / Module Structure

dusk-auth-core/
├── Cargo.toml
└── src/
├── lib.rs
├── auth.rs
├── session.rs
├── token.rs
├── decision.rs
├── store/
│ ├── mod.rs
│ └── memory.rs
├── error.rs
├── time.rs
└── tests/
└── invariants.rs



### Module Responsibilities

- `lib.rs`  
  Public surface area and re-exports

- `auth.rs`  
  Core orchestration and invariant enforcement

- `session.rs`  
  Session domain model and lifecycle rules

- `token.rs`  
  Typed access and refresh tokens

- `decision.rs`  
  Explicit authentication outcomes

- `store/`  
  Storage boundary and reference implementation

- `error.rs`  
  Domain-level authentication errors

- `time.rs`  
  Explicit time handling for determinism and testing

- `tests/invariants.rs`  
  Tests that prove invariants cannot be violated



## Definition of “v1.0 Done”

`dusk-auth-core v1.0` is complete when:

- All five invariants are enforced
- Session storage is pluggable
- In-memory store is functional
- Core flows are implemented and tested
- README explains intent, design, and trade-offs

The following are explicitly excluded from v1.0:
- Framework adapters
- OAuth integrations
- Storage-specific implementations beyond memory


## Contribution Philosophy

- MIT licensed
- Contributions are welcome if they preserve core invariants
- PRs that weaken guarantees or blur responsibilities will be rejected

Correctness and clarity take priority over feature count.



## Guiding Principle

This project treats authentication as a **system**, not a feature.

The API, structure, and constraints are designed to teach correct thinking
by making incorrect usage difficult.

