use crate::{
    AuthError,
    decision::AuthDecision,
    session::SessionId,
    store::SessionStore,
    time::Timestamp,
    token::{AccessToken, RefreshToken, RefreshTokenId},
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

    pub fn refresh_session(
        &self,
        refresh_token: &RefreshToken,
        now: Timestamp,
    ) -> Result<(AccessToken, RefreshToken), AuthError> {
        // 1. Load session
        let mut session = match self.store.load(&refresh_token.session_id) {
            Some(s) => s,
            None => return Err(AuthError::InvalidRefreshToken),
        };

        // 2. Check revoked / expired
        if session.is_revoked() {
            return Err(AuthError::SessionRevoked);
        }

        if session.is_expired(now) {
            return Err(AuthError::SessionExpired);
        }

        // 3. Detect refresh token reuse
        if refresh_token.refresh_token_id != session.current_refresh_token_id {
            // SECURITY EVENT: Revoke session
            self.store.revoke(&session.id);
            return Err(AuthError::RefreshTokenReused);
        }

        // 4. Rotate refresh token
        let new_refresh_token_id = RefreshTokenId(format!(
            "rt-{}",
            uuid::Uuid::new_v4()
        ));

        session.current_refresh_token_id = new_refresh_token_id.clone();

        // 5. Persiste update session
        self.store.save(session.clone());

        // 6. Issue new tokens
        let access_token = AccessToken(session.id.0.clone());

        let new_refresh_token = RefreshToken {
            session_id: session.id,
            refresh_token_id: new_refresh_token_id
        };

        Ok((access_token, new_refresh_token))
    }
}
