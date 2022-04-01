use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum UserError {}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("An internal error occurred. Please try again later.")]
    InternalError(#[from] anyhow::Error),
    #[error("{0}")]
    UserError(#[from] UserError),
}

impl actix_web::error::ResponseError for Error {}
