use crate::errors::Error;

pub mod init;
pub mod responses;

mod errors;
mod routes;
mod services;

#[allow(dead_code)]
type AnyResult<T> = Result<T, Error>;
