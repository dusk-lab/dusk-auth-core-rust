use dusk_auth_core::{
    Authenticator,
    AuthDecision,
    InMemorySessionStore,
    Session,
    SessionId,
    AccessToken,
};
use dusk_auth_core::time::Timestamp;
use std::time::{SystemTime, Duration};

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
