use crate::session::Session;

#[derive(Debug, Clone)]
pub enum AuthDecision {
    Valid(Session),
    Expired,
    Revoked,
    Invalid,
}