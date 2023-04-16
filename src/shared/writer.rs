use std::io::{Seek, Write};
use crate::error::PackageWriteError;
use crate::Package;

pub trait PackageWriter {
    fn write_package_to_output<T: Write + Seek>(&self, package: &Package, output: T) -> Result<(), PackageWriteError>;
}
