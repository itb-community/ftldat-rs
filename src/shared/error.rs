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
        #[from]
        #[source] InnerPathAlreadyExistsError
    ),
    #[error("corrupt package file")]
    PackageCorruptError(
        #[from]
        #[source] PackageCorruptError
    ),
}

#[derive(Error, Debug)]
pub enum PackageCorruptError {
    #[error("signature: expected byte '{expected}', but found '{actual}'")]
    SignatureMismatchError {
        expected: u8,
        actual: u8,
    },
    #[error("header: expected header size '{expected}', but found '{actual}'")]
    HeaderSizeMismatchError {
        expected: u16,
        actual: u16,
    },
    #[error("header: expected entries size to be '{expected}', but found '{actual}'")]
    EntriesCountSizeMismatchError {
        expected: u16,
        actual: u16,
    },
    #[error("header: header claims combined size of all entries ('{0}') to be greater than the entire file")]
    EntriesTotalSizeError(usize),
    #[error("header: header claims combined size of all paths ('{0}') to be greater than the entire file")]
    PathStringsTotalSizeError(usize),
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
    #[error("deflated entries are not supported!")]
    UnsupportedDeflatedEntryError(),
    #[error("entry: expected inner path '{inner_path}' hash to match {expected}, but was {actual}")]
    PathHashMismatchError {
        inner_path: String,
        expected: u32,
        actual: u32
    }
}
