use dusk_auth_core_rust::{
    time::Timestamp,
    token::RefreshTokenId,
    AccessToken,
    AuthDecision,
    Authenticator,
    InMemorySessionStore,
    Session,
    SessionId,
    SessionStore,
    RefreshToken,
    AuthError,
};
// use dusk_auth_core_rust::time::Timestamp;
use std::time::{Duration, SystemTime};

#[test]
fn revoked_session_never_validates() {
    // Arrange
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());
    let expires_at = Timestamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-123".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-1".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
        current_refresh_token_id: RefreshTokenId("Intial".to_string()),
    };

    // Save session
    auth.store.save(session);

    // Revoke session (logout)
    auth.store.revoke(&session_id);

    let token = AccessToken("session-123".to_string());

    // Act
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Revoked => {}
        other => panic!(
            "Expected Revoked decision for revoked session, got {:?}",
            other
        ),
    }
}

#[test]
fn expired_session_never_validates() {
    // Arrange
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());
    let expired_at = Timestamp(SystemTime::now() - Duration::from_secs(60));

    let session_id = SessionId("session-expired".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-2".to_string(),
        created_at: expired_at,
        expires_at: expired_at,
        revoked_at: None,
        current_refresh_token_id: RefreshTokenId("Intial".to_string()),
    };

    // Save expired session
    auth.store.save(session);

    let token = AccessToken("session-expired".to_string());

    // Act
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Expired => {}
        other => panic!(
            "Expected Expired decision for expired session, got {:?}",
            other
        ),
    }
}

#[test]
fn missing_session_returns_invalid() {
    // Arrange
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());

    // Token refers to a session that does not exist
    let token = AccessToken("non-existent-session".to_string());

    // Act
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Invalid => {}
        other => panic!(
            "Expected Invalid decision for missing session, got {:?}",
            other
        ),
    }
}

#[test]
fn active_session_validates() {
    // Arrange
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());
    let expires_at = Timestamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-active".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-active".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
        current_refresh_token_id: RefreshTokenId("Intial".to_string()),
    };

    // Save active session
    auth.store.save(session.clone());

    let token = AccessToken("session-active".to_string());

    // Act
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Valid(returned_session) => {
            assert_eq!(returned_session.id, session.id);
        }
        other => panic!(
            "Expected Valid decision for active session, got {:?}",
            other
        ),
    }
}

#[test]
fn logout_revokes_session() {
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());
    let expires_at = Timestamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-logout".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-logout".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
        current_refresh_token_id: RefreshTokenId("Intial".to_string()),
    };

    auth.store.save(session);

    // Act: logout
    auth.revoke_session(&session_id);

    let token = AccessToken("session-logout".to_string());
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Revoked => {}
        other => panic!("Expected Revoked decision after logout, got {:?}", other),
    }
}


#[test]
fn refresh_token_reuse_revokes_session() {
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = Timestamp(SystemTime::now());
    let expires_at = Timestamp(SystemTime::now() + Duration::from_secs(3600));

    // Initial refresh token id
    let refresh_token_id = "refresh-1".to_string();

    let session_id = SessionId("session-refresh".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-refresh".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
        current_refresh_token_id: RefreshTokenId(refresh_token_id.clone()),
    };

    auth.store.save(session);

    // First refresh token (valid)
    let refresh_token = RefreshToken {
        session_id: session_id.clone(),
        refresh_token_id: RefreshTokenId(refresh_token_id.clone()),
    };

    // Act 1: First refresh should succeed
    let result = auth.refresh_session(&refresh_token, now);
    assert!(result.is_ok(), "first refresh should succeed");

    // Act 2: Reuse the same refresh token
    let reuse_result = auth.refresh_session(&refresh_token, now);

    // Assert: reuse revokes the session
    match reuse_result {
        Err(AuthError::RefreshTokenReused) => {}
        other => panic!("Expected RefreshTokenReused error, got {:?}", other),
    }

    // Assert: session is now revoked
    let access_token = AccessToken("session-refresh".to_string());
    let decision = auth.validate_access_token(&access_token, now);

    match decision {
        AuthDecision::Revoked => {}
        other => panic!(
            "Expected session to be revoked after refresh token reuse, got {:?}",
            other
        ),
    }
}
