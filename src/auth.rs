use crate::store::SessionStore;

pub struct Authenticator<S: SessionStore> {
    pub store: S,
}

impl<S: SessionStore> Authenticator<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }
}