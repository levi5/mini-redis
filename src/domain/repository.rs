use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
pub trait DatabaseRepository: Send + Sync {
    async fn set(&self, key: String, value: Bytes);
    async fn get(&self, key: &str) -> Option<Bytes>;
}
