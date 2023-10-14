#![allow(unused)]

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectError(surrealdb::Error),    
    CreateRecordError(surrealdb::Error),
    ReadRecordError(surrealdb::Error),
    UpdateRecordError(surrealdb::Error),
    DeleteRecordError(surrealdb::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConnectError(err) => write!(f, "{}", err),
            Error::CreateRecordError(err) => write!(f, "{}", err),
            Error::ReadRecordError(err) => write!(f, "{}", err),
            Error::UpdateRecordError(err) => write!(f, "{}", err),
            Error::DeleteRecordError(err) => write!(f, "{}", err),
        }
    }
}
