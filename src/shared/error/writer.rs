use std::io::Error;

use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct PackageWriteError(#[from] pub(crate) PackageWriteErrorImpl);

impl PackageWriteError {}

impl From<std::io::Error> for PackageWriteError {
    fn from(error: Error) -> Self {
        Self(PackageWriteErrorImpl::IOError(error))
    }
}

impl From<EntryWriteError> for PackageWriteError {
    fn from(error: EntryWriteError) -> Self {
        Self(PackageWriteErrorImpl::EntryWriteError(error))
    }
}

#[derive(Error, Debug)]
pub(crate) enum PackageWriteErrorImpl {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("package contains more than u32::MAX entries")]
    EntryCountExceededError(),
    #[error("total size of all inner paths in package ({0}) is larger than {}", u32::MAX)]
    PathAreaSizeExceededError(usize),
    #[error(transparent)]
    EntryWriteError(#[from] EntryWriteError),
}

impl<T> Into<Result<T, PackageWriteError>> for PackageWriteErrorImpl {
    fn into(self) -> Result<T, PackageWriteError> {
        Err(PackageWriteError(self))
    }
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct EntryWriteError(#[from] pub(crate) EntryWriteErrorImpl);

impl EntryWriteError {}

#[derive(Error, Debug)]
pub(crate) enum EntryWriteErrorImpl {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl From<std::io::Error> for EntryWriteError {
    fn from(error: Error) -> Self {
        Self(EntryWriteErrorImpl::IOError(error))
    }
}