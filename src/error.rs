use thiserror::Error;

use crate::executor::ExecutorError;
use crate::nlp::NlpError;
use crate::planner::PlannerError;
use crate::storage::StorageError;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("NLP error: {0}")]
    Nlp(NlpError),
    #[error("Planner error: {0}")]
    Planner(PlannerError),
    #[error("Executor error: {0}")]
    Executor(ExecutorError),
    #[error("Storage error: {0}")]
    Storage(StorageError),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<NlpError> for CoreError {
    fn from(err: NlpError) -> Self {
        CoreError::Nlp(err)
    }
}

impl From<PlannerError> for CoreError {
    fn from(err: PlannerError) -> Self {
        CoreError::Planner(err)
    }
}

impl From<ExecutorError> for CoreError {
    fn from(err: ExecutorError) -> Self {
        CoreError::Executor(err)
    }
}

impl From<StorageError> for CoreError {
    fn from(err: StorageError) -> Self {
        CoreError::Storage(err)
    }
}
