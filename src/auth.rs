use crate::{
    decision::AuthDecision,
    session::SessionId,
    store::SessionStore,
    time::Timestamp,
    token::AccessToken,
};

pub struct Authenticator<S: SessionStore> {
    pub store: S,
}

impl<S: SessionStore> Authenticator<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn validate_access_token(&self, token: &AccessToken, now: Timestamp) -> AuthDecision {
        let session_id = match token.session_id() {
            Some(id) => SessionId(id),
            None => return AuthDecision::Invalid,
        };

        let session = match self.store.load(&session_id) {
            Some(session) => session,
            None => return AuthDecision::Invalid,
        };

        if session.is_revoked() {
            return AuthDecision::Revoked;
        }

        if session.is_expired(now) {
            return AuthDecision::Expired;
        }

        AuthDecision::Valid(session)
    }

    /// Revoke a session (server-side logout).
    ///
    /// This operation is idempotent.
    pub fn revoke_session(&self, session_id: &SessionId) {
        self.store.revoke(session_id);
    }
}
