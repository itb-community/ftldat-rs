use std::fs::File;
use crate::error::PackageReadError;
use crate::Package;

/// A trait that describes how a [`Package`] object should be read from a specific file format.
///
/// The trait is defined by a single required method, [`read_package_from_file`](PackageReader::read_package_from_file),
/// which implements the conversion of the file's binary content to a [`Package`].
pub trait PackageReader {
    fn read_package_from_file(&self, file: File) -> Result<Package, PackageReadError>;
}
