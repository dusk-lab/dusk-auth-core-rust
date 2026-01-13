use crate::session::{Session, SessionId};

pub trait SessionStore {
    fn load(&self, id: &SessionId) -> Option<Session>;
    fn save(&self, session: Session);
    fn revoke (&self, id: &SessionId);
}

pub mod memory;

pub use memory::InMemorySessionStore;