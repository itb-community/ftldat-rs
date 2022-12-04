use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("inner path already exists within the package: '{0}'")]
pub struct InnerPathAlreadyExistsError(pub(crate) String);

#[derive(Error, Debug)]
pub enum ReadPackageError {
    #[error("failed to read bytes")]
    ReadByteError(
        #[from]
        #[source] std::io::Error
    ),
    #[error("failed to read package entry")]
    ReadEntryError(
        #[from]
        #[source] ReadEntryError
    ),
    #[error("failed to process package during read")]
    ProcessPackageError(
        #[source] InnerPathAlreadyExistsError
    ),
}

#[derive(Error, Debug)]
pub enum ReadEntryError {
    #[error("failed to read bytes")]
    ReadByteError(
        #[from]
        #[source] std::io::Error
    ),
    #[error("failed to decode bytes as utf8 string")]
    DecodeByteError(
        #[from]
        #[source] FromUtf8Error
    ),
}
