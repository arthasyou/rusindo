use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message:} ({line:}, {column})")]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("auth error: {0}")]
    AuthError(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;