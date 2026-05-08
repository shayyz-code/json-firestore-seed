// Copyright (c) 2025 RustRaccoon Software Company Ltd.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeedError {
    #[error("File not found: {path}")]
    FileNotFound {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid JSON structure: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("JSON must be an array of objects")]
    NotAJsonArray,

    #[error("Firestore authentication failed: {0}")]
    AuthError(String),

    #[error("Firestore write error for collection '{collection}': {source}")]
    FirestoreWrite {
        collection: String,
        #[source]
        source: firestore::errors::FirestoreError,
    },

    #[error("Timestamp parsing failed for '{input}': {details}")]
    TimestampParse { input: String, details: String },

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, SeedError>;
