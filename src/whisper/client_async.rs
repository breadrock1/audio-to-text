use crate::whisper::client::WhisperClient;
use crate::whisper::config::WhisperClientConfig;

use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};

#[derive(Clone)]
pub struct WhisperAsyncClient {
    client: Arc<RwLock<WhisperClient>>,
}

impl WhisperAsyncClient {
    pub fn new(cfg: &WhisperClientConfig) -> Self {
        let whisper_client = WhisperClient::new(cfg);

        WhisperAsyncClient {
            client: Arc::new(RwLock::new(whisper_client)),
        }
    }
    pub async fn get_client(&self) -> RwLockWriteGuard<'_, WhisperClient> {
        self.client.write().await
    }
}
