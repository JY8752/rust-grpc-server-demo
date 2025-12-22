use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type ID = String;
pub struct Store<T>(Arc<RwLock<HashMap<ID, T>>>);

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }

    pub async fn write(&self, id: ID, value: T) -> () {
        self.0.write().await.insert(id, value);
    }
}

impl<T: Clone> Store<T> {
    pub async fn read(&self, id: &ID) -> Option<T> {
        self.0.read().await.get(id).cloned()
    }
}
