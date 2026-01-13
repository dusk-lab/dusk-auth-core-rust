use crate::time::TimeStamp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(pub String);

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub subject:String,
    pub created_at: TimeStamp,
    pub expires_at: TimeStamp,
    pub revoked_at: Option<TimeStamp>,
}