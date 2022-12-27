use std::io::Error;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("inner path '{0}' already exists within the package")]
pub struct InnerPathAlreadyExistsError(pub(crate) String);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageReadError(#[from] pub(crate) Box<dyn std::error::Error>);

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageWriteError(#[from] pub(crate) Box<dyn std::error::Error>);

impl From<std::io::Error> for PackageReadError {
    fn from(error: Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<std::io::Error> for PackageWriteError {
    fn from(error: Error) -> Self {
        Self(Box::new(error))
    }
}

impl From<InnerPathAlreadyExistsError> for PackageReadError {
    fn from(error: InnerPathAlreadyExistsError) -> PackageReadError {
        PackageReadError(Box::new(error))
    }
}