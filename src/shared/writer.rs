use std::io::{Seek, Write};
use crate::error::PackageWriteError;
use crate::Package;

/// A trait that describes how a [`Package`] object should be written into a specific file format.
///
/// The trait is defined by a single required method, [`write_package_to_output`](PackageWriter::write_package_to_output),
/// which implements the conversion of the [`Package`] to a binary representation.
pub trait PackageWriter {
    fn write_package_to_output<T: Write + Seek>(&self, package: &Package, output: T) -> Result<(), PackageWriteError>;
}
