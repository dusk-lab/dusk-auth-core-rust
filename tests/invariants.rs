use dusk_auth_core_rust::{
    Authenticator,
    AuthDecision,
    InMemorySessionStore,
    Session,
    SessionId,
    AccessToken,
    SessionStore
};
use dusk_auth_core_rust::time::TimeStamp;
use std::time::{SystemTime, Duration};

#[test]
fn revoked_session_never_validates() {
    // Arrange
    let store = InMemorySessionStore::new();
    let auth = Authenticator::new(store);

    let now = TimeStamp(SystemTime::now());
    let expires_at = TimeStamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-123".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-1".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
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

    let now = TimeStamp(SystemTime::now());
    let expired_at = TimeStamp(SystemTime::now() - Duration::from_secs(60));

    let session_id = SessionId("session-expired".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-2".to_string(),
        created_at: expired_at,
        expires_at: expired_at,
        revoked_at: None,
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

    let now = TimeStamp(SystemTime::now());

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

    let now = TimeStamp(SystemTime::now());
    let expires_at = TimeStamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-active".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-active".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
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

    let now = TimeStamp(SystemTime::now());
    let expires_at = TimeStamp(SystemTime::now() + Duration::from_secs(3600));

    let session_id = SessionId("session-logout".to_string());

    let session = Session {
        id: session_id.clone(),
        subject: "user-logout".to_string(),
        created_at: now,
        expires_at,
        revoked_at: None,
    };

    auth.store.save(session);

    // Act: logout
    auth.revoke_session(&session_id);

    let token = AccessToken("session-logout".to_string());
    let decision = auth.validate_access_token(&token, now);

    // Assert
    match decision {
        AuthDecision::Revoked => {}
        other => panic!(
            "Expected Revoked decision after logout, got {:?}",
            other
        ),
    }
}
