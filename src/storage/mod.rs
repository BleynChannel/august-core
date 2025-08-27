use serde::{Deserialize, Serialize};
use sled::Db;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredData {
    pub key: String,
    pub value: String,
}

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: PathBuf) -> Result<Self, StorageError> {
        let db = sled::open(path).map_err(|e| StorageError::DbError(e.to_string()))?;
        Ok(Storage { db })
    }

    pub async fn insert(&self, key: &str, value: String) -> Result<(), StorageError> {
        let data = StoredData {
            key: key.to_string(),
            value,
        };
        let serialized = serde_json::to_vec(&data)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.db
            .insert(key, serialized)
            .map_err(|e| StorageError::DbError(e.to_string()))?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let result = self
            .db
            .get(key)
            .map_err(|e| StorageError::DbError(e.to_string()))?;
        match result {
            Some(ivec) => {
                let data: StoredData = serde_json::from_slice(&ivec)
                    .map_err(|e| StorageError::DeserializationError(e.to_string()))?;
                Ok(Some(data.value))
            }
            None => Ok(None),
        }
    }

    pub async fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.db
            .remove(key)
            .map_err(|e| StorageError::DbError(e.to_string()))?;
        Ok(())
    }
}
