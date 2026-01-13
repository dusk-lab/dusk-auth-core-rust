use std::collections::HashMap;
use std::sync::RwLock;

use crate::session::{Session, SessionId};
use super::SessionStore;

pub struct InMemorySessionStore {
    sessions: RwLock<HashMap<SessionId, Session>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl SessionStore for InMemorySessionStore {
    fn load(&self, _id: &SessionId) -> Option<Session> {
        unimplemented!()
    }

    fn save(&self, _session: Session) {
        unimplemented!()
    }

    fn revoke(&self, _id: &SessionId) {
        unimplemented!()
    }
}