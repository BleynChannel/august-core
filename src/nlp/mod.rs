use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct NlpRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NlpResponse {}

#[derive(Error, Debug)]
pub enum NlpError {
    #[error("Other error: {0}")]
    Other(String),
}

#[async_trait]
pub trait NlpProcessor: Send + Sync {
    async fn process(&self, request: NlpRequest) -> Result<NlpResponse, NlpError>;
}

pub struct MockNlpProcessor;

#[async_trait]
impl NlpProcessor for MockNlpProcessor {
    async fn process(&self, request: NlpRequest) -> Result<NlpResponse, NlpError> {
        info!("Mock NLP processing request: {:?}", request);
        // Simulate some NLP processing
        Ok(NlpResponse {})
    }
}
