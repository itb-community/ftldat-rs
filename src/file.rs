use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::error::Error;
use crate::package::Package;

/// Convenience struct to hold a reference to a file and exposing methods to
/// read/write a [Package].
#[derive(Debug)]
pub struct DatFile {
    path: String,
    file: File,
}

impl DatFile {
    pub fn new(path: &str) -> Result<DatFile, Error> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(DatFile {
            path: path.to_string(),
            file,
        })
    }

    pub fn from(path: &str) -> Result<DatFile, Error> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)?;

        Ok(DatFile {
            path: path.to_string(),
            file,
        })
    }

    pub fn read_package(&self) -> Result<Package, Error> {
        Package::from_reader(BufReader::new(&self.file))
    }

    pub fn write_package(&mut self, package: Package) -> Result<(), Error> {
        package.write(BufWriter::new(&self.file))
    }
}
