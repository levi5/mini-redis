use crate::domain::repository::DatabaseRepository;
use bytes::Bytes;
use std::sync::Arc;

pub struct SetValueUseCase {
    db: Arc<dyn DatabaseRepository>,
}

impl SetValueUseCase {
    pub fn new(db: Arc<dyn DatabaseRepository>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, key: String, value: Bytes) {
        println!("[UseCase] Save key: {}", key);
        self.db.set(key, value).await;
    }
}
