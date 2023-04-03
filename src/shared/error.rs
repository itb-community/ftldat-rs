use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("inner path '{0}' already exists within the package")]
pub struct InnerPathAlreadyExistsError(pub(crate) String);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageReadError(#[from] pub(crate) Box<dyn Error>);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageWriteError(#[from] pub(crate) Box<dyn Error>);

impl From<std::io::Error> for PackageReadError {
    fn from(error: std::io::Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<std::io::Error> for PackageWriteError {
    fn from(error: std::io::Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<InnerPathAlreadyExistsError> for PackageReadError {
    fn from(error: InnerPathAlreadyExistsError) -> Self {
        Self(Box::new(error))
    }
}