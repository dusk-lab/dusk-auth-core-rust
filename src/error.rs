#[derive(Debug)]
pub enum AuthError {
    SessionNotFound,
    SessionExpired,
    SessionRevoked,
    InvalidToken,
    RefreshTokenReused
}