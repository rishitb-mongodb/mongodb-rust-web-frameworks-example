use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    
    #[error("Invalid object ID: {0}")]
    InvalidObjectId(#[from] bson::oid::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Handler error: {0}")]
    HandlerError(#[from] tokio::task::JoinError),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}