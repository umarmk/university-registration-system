use thiserror::Error;
use diesel::result::Error as DieselError;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DieselError),
    
    #[error("Record not found")]
    NotFound,
    
    #[error("Duplicate record")]
    DuplicateRecord,
    
    #[error("Invalid input: {0}")]
    ValidationError(String),
    
    #[error("Connection pool error: {0}")]
    PoolError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

impl From<r2d2::Error> for DbError {
    fn from(err: r2d2::Error) -> Self {
        DbError::PoolError(err.to_string())
    }
}

impl From<diesel::result::Error> for DbError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            DieselError::NotFound => DbError::NotFound,
            DieselError::DatabaseError(_, info) => DbError::DatabaseError(error),
            _ => DbError::DatabaseError(error),
        }
    }
} 