use thiserror::Error;
use crate::shared::error::{PackageReadError, PackageWriteError};

#[derive(Error, Debug)]
#[error("package file is corrupt")]
pub(super) enum FileCorruptError {
    #[error("signature: expected bytes '{expected}', but found '{actual}'")]
    SignatureMismatchError {
        expected: u8,
        actual: u8,
    },
    #[error("header: expected header size '{expected}', but found '{actual}'")]
    HeaderSizeMismatchError {
        expected: u16,
        actual: u16,
    },
    #[error("header: expected entry header's size to be '{expected}', but found '{actual}'")]
    EntriesHeaderSizeMismatchError {
        expected: u16,
        actual: u16,
    },
}

impl Into<PackageReadError> for FileCorruptError {
    fn into(self) -> PackageReadError {
        PackageReadError(Box::new(self))
    }
}

#[derive(Error, Debug)]
pub(super) enum EntryReadError {
    #[error("deflated entries are not supported!")]
    UnsupportedDeflatedEntryError(),
    #[error("entry: expected inner path '{inner_path}' hash to match {expected}, but was {actual}")]
    PathHashMismatchError {
        inner_path: String,
        expected: u32,
        actual: u32,
    },
}

impl From<EntryReadError> for PackageReadError {
    fn from(error: EntryReadError) -> PackageReadError {
        PackageReadError(Box::new(error))
    }
}

#[derive(Error, Debug)]
pub(super) enum PkgWriteError {
    #[error("package contains more than {} entries", u32::MAX)]
    EntryCountExceededError(),
    #[error("total size of all inner paths in package ({0}) is larger than {}", u32::MAX)]
    PathAreaSizeExceededError(usize)
}

impl Into<PackageWriteError> for PkgWriteError {
    fn into(self) -> PackageWriteError {
        PackageWriteError(Box::new(self))
    }
}

