#[derive(Debug, Clone)]
pub struct AccessToken(pub String);

#[derive(Debug, Clone)]
pub struct RefreshToken(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshTokenId(pub String);

impl AccessToken {
    /// Temporary helper to extract a session id.
    /// In real systems, this would come from decoding a JWT or similar
    pub fn session_id(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.clone())
        }
    }
}