use async_trait::async_trait;
use bytes::Bytes;
use mini_redis::domain::repository::DatabaseRepository;
use mini_redis::use_cases::set_value::SetValueUseCase;
use std::sync::{Arc, Mutex};

struct IntegrationMockDb {
    storage: Arc<Mutex<Option<(String, Bytes)>>>,
}

#[async_trait]
impl DatabaseRepository for IntegrationMockDb {
    async fn set(&self, key: String, value: Bytes) {
        let mut store = self.storage.lock().unwrap();
        *store = Some((key, value));
    }

    async fn get(&self, _key: &str) -> Option<Bytes> {
        None
    }
}

#[tokio::test]
async fn test_complete_set_workflow() {
    let mock_db = Arc::new(IntegrationMockDb {
        storage: Arc::new(Mutex::new(None)),
    });

    let use_case = SetValueUseCase::new(mock_db.clone());

    use_case
        .execute("sessao_integracao".into(), Bytes::from("dados_123"))
        .await;

    let store = mock_db.storage.lock().unwrap();
    assert!(store.is_some(), "O fluxo de integração falhou ao salvar");

    let (key, value) = store.as_ref().unwrap();
    assert_eq!(key, "sessao_integracao");
    assert_eq!(value, &Bytes::from("dados_123"));
}
