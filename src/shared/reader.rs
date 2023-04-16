use std::fs::File;
use crate::error::PackageReadError;
use crate::Package;

pub trait PackageReader {
    fn read_package_from_file(&self, file: File) -> Result<Package, PackageReadError>;
}

