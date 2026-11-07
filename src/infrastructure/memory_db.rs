use crate::domain::repository::DatabaseRepository;
use async_trait::async_trait;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MemoryDatabase {
    state: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl MemoryDatabase {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DatabaseRepository for MemoryDatabase {
    async fn set(&self, key: String, value: Bytes) {
        let mut db = self.state.lock().unwrap();
        db.insert(key, value);
    }

    async fn get(&self, key: &str) -> Option<Bytes> {
        let db = self.state.lock().unwrap();
        db.get(key).cloned()
    }
}
