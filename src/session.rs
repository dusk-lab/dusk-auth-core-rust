use crate::{
    token:: RefreshTokenId,
    time::Timestamp
};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub subject:String,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,
    pub revoked_at: Option<Timestamp>,

    /// The currently valid refresh token identifier.
    /// Any mismatch indicates reuse or forgery.
    pub current_refresh_token_id: RefreshTokenId,
}

impl Session {
    /// Returns true if the session has been explicitly revoked
    pub fn is_revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    /// Returns true if the session has expired at the given time
    pub fn is_expired(&self, now: Timestamp) -> bool {
        now >= self.expires_at
    }

    /// Returns true if the session is both active and valid at the given time.
    ///
    /// Note: This does NOT mean authenticated — only that the session
    /// has not expired and has not been revoked.
    pub fn is_active(&self, now: Timestamp) -> bool {
        !self.is_revoked() && !self.is_expired(now)
    }
    
}