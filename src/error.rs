#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    /// The referenced session does not exist.
    SessionNotFound,

    /// The session has expired and can no longer be used.
    SessionExpired,

    /// The session has been explicitly revoked.
    SessionRevoked,

    /// The token is malformed or invalid.
    InvalidToken,

    /// The refresh token is invalid or does not match the session.
    InvalidRefreshToken,

    /// A refresh token was reused. This is treated as a security breach.
    RefreshTokenReused,
}
