use async_trait::async_trait;
use bytes::Bytes;
use mini_redis::domain::repository::DatabaseRepository;
use mini_redis::use_cases::set_value::SetValueUseCase;
use std::sync::{Arc, Mutex};

struct MockDb {
    state: Arc<Mutex<Option<(String, Bytes)>>>,
}

#[async_trait]
impl DatabaseRepository for MockDb {
    async fn set(&self, key: String, value: Bytes) {
        let mut s = self.state.lock().unwrap();
        *s = Some((key, value));
    }
    async fn get(&self, _key: &str) -> Option<Bytes> {
        None
    }
}

#[tokio::test]
async fn test_set_use_case_isolated() {
    let mock = Arc::new(MockDb {
        state: Arc::new(Mutex::new(None)),
    });
    let use_case = SetValueUseCase::new(mock.clone());

    use_case
        .execute("key_test".into(), Bytes::from("key_test"))
        .await;

    let res = mock.state.lock().unwrap();
    assert!(res.is_some());
}
