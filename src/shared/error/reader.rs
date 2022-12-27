use std::io::Error;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::shared::error::shared::InnerPathAlreadyExistsError;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageReadError(#[from] pub(crate) PackageReadErrorImpl);

impl PackageReadError {}

impl From<std::io::Error> for PackageReadError {
    fn from(error: Error) -> Self {
        Self(PackageReadErrorImpl::IOError(error))
    }
}

impl From<EntryReadError> for PackageReadError {
    fn from(error: EntryReadError) -> Self {
        Self(PackageReadErrorImpl::EntryReadError(error))
    }
}

impl From<InnerPathAlreadyExistsError> for PackageReadError {
    fn from(error: InnerPathAlreadyExistsError) -> Self {
        Self(PackageReadErrorImpl::InnerPathAlreadyExistsError(error))
    }
}

#[derive(Error, Debug)]
pub(crate) enum PackageReadErrorImpl {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    EntryReadError(#[from] EntryReadError),
    #[error(transparent)]
    InnerPathAlreadyExistsError(#[from] InnerPathAlreadyExistsError),
    #[error("corrupt package file - {0}")]
    PackageCorruptError(#[from] PackageCorruptErrorImpl),
}

#[derive(Error, Debug)]
pub(crate) enum PackageCorruptErrorImpl {
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
    #[error("header: expected entry header's size to be '{expected}', but found '{actual}'")]
    EntriesHeaderSizeMismatchError {
        expected: u16,
        actual: u16,
    },
}

impl<T> Into<Result<T, PackageReadError>> for PackageCorruptErrorImpl {
    fn into(self) -> Result<T, PackageReadError> {
        Err(PackageReadError(PackageReadErrorImpl::PackageCorruptError(self)))
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct EntryReadError(#[from] pub(crate) EntryReadErrorImpl);

impl EntryReadError {}

impl From<std::io::Error> for EntryReadError {
    fn from(error: Error) -> Self {
        Self(EntryReadErrorImpl::IOError(error))
    }
}

impl From<FromUtf8Error> for EntryReadError {
    fn from(error: FromUtf8Error) -> Self {
        Self(EntryReadErrorImpl::Utf8DecodeError(error))
    }
}

impl<T> Into<Result<T, EntryReadError>> for EntryReadErrorImpl {
    fn into(self) -> Result<T, EntryReadError> {
        Err(EntryReadError(self))
    }
}

#[derive(Error, Debug)]
pub(crate) enum EntryReadErrorImpl {
    #[error(transparent)]
    IOError(#[from]std::io::Error),
    #[error(transparent)]
    Utf8DecodeError(#[from] FromUtf8Error),
    #[error("deflated entries are not supported!")]
    UnsupportedDeflatedEntryError(),
    #[error("entry: expected inner path '{inner_path}' hash to match {expected}, but was {actual}")]
    PathHashMismatchError {
        inner_path: String,
        expected: u32,
        actual: u32,
    },
}
