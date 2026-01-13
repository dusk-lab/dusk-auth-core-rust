use std::collections::HashMap;
use std::sync::RwLock;

use super::SessionStore;
use crate::session::{Session, SessionId};

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
    fn load(&self, id: &SessionId) -> Option<Session> {
        let guard = self.sessions.read().expect("lock poisoned");
        guard.get(id).cloned()
    }

    fn save(&self, session: Session) {
        let mut guard = self.sessions.write().expect("lock poisoned");
        guard.insert(session.id.clone(), session);
    }

    fn revoke(&self, id: &SessionId) {
        let mut guard = self.sessions.write().expect("lock poisoned");
        if let Some(existing) = guard.get_mut(id) {
            // We do not set time here; the auth engine decides "when"
            existing.revoked_at = existing.revoked_at.or(Some(existing.expires_at));
        }
    }
}
