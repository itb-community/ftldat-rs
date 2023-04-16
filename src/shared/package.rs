use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Seek, Write};
use std::path::Path;
use std::slice::Iter;

use crate::{PackageReader, PackageWriter};
use crate::dat::{DatReader, DatWriter};
use crate::error::{InnerPathAlreadyExistsError, PackageReadError, PackageWriteError};
use crate::pkg::{PkgReader, PkgWriter};
use crate::shared::entry::PackageEntry;

/// Represents the internal structure of a package.
///
/// These packages consist of a list of [entries](PackageEntry).
#[derive(Debug)]
pub struct Package {
    /// Use a Vec as main [PackageEntry] storage; this way we retain the order in which the source
    /// file originally stored its entries.
    entries: Vec<PackageEntry>,
    inner_path_to_entry_index: BTreeMap<String, usize>,
}

impl Package {
    // region <Constructors>
    /// Creates a new empty [Package].
    pub fn new() -> Package {
        Package {
            entries: Vec::new(),
            inner_path_to_entry_index: BTreeMap::new(),
        }
    }

    /// Creates a new empty [Package], with a backing vector having at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Package {
        Package {
            entries: Vec::with_capacity(capacity),
            inner_path_to_entry_index: BTreeMap::new(),
        }
    }

    /// Reads the file at the specified path using DAT format, and creates a [Package] instance.
    ///
    /// This function memory-maps the file, whose lifetime is as long as the longest-lived entry
    /// read from this file.
    /// If the [Package] instance created by this function goes out of scope, and its entries are
    /// not referenced anywhere, the memory map will be correctly disposed.
    pub fn from_path_dat<P: AsRef<Path>>(source_path: P) -> Result<Package, PackageReadError> {
        Package::from_path(source_path, DatReader())
    }

    /// Reads the specified file using DAT format, and creates a [Package] instance.
    ///
    /// This function memory-maps the file, whose lifetime is as long as the longest-lived entry
    /// read from this file.
    /// If the [Package] instance created by this function goes out of scope, and its entries are
    /// not referenced anywhere, the memory map will be correctly disposed.
    pub fn from_file_dat(file: File) -> Result<Package, PackageReadError> {
        Package::from_file(file, DatReader())
    }

    /// Reads the file at the specified path using PKG format, and creates a [Package] instance.
    ///
    /// This function memory-maps the file, whose lifetime is as long as the longest-lived entry
    /// read from this file.
    /// If the [Package] instance created by this function goes out of scope, and its entries are
    /// not referenced anywhere, the memory map will be correctly disposed.
    pub fn from_path_pkg<P: AsRef<Path>>(source_path: P) -> Result<Package, PackageReadError> {
        Package::from_path(source_path, PkgReader())
    }

    /// Reads the specified file using PKG format, and creates a [Package] instance.
    ///
    /// This function memory-maps the file, whose lifetime is as long as the longest-lived entry
    /// read from this file.
    /// If the [Package] instance created by this function goes out of scope, and its entries are
    /// not referenced anywhere, the memory map will be correctly disposed.
    pub fn from_file_pkg(file: File) -> Result<Package, PackageReadError> {
        Package::from_file(file, PkgReader())
    }

    /// Reads the file at the specified path using format provided by the specified [PackageReader],
    /// and creates a [Package] instance.
    pub fn from_path<P: AsRef<Path>, T: PackageReader>(source_path: P, reader: T) -> Result<Package, PackageReadError> {
        let file = File::options()
            .read(true)
            .open(source_path)?;
        Package::from_file(file, reader)
    }

    /// Reads the specified file using format provided by the specified [PackageReader], and creates
    /// a [Package] instance.
    pub fn from_file<T: PackageReader>(file: File, reader: T) -> Result<Package, PackageReadError> {
        reader.read_package_from_file(file)
    }
    // endregion

    // region <Output>
    // region <DAT>
    /// Consumes and writes this [Package] in DAT format to file at the specified path.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_path_dat] instead.
    pub fn into_path_dat<P: AsRef<Path>>(self, destination_path: P) -> Result<(), PackageWriteError> {
        self.into_path(destination_path, DatWriter())
    }

    /// Consumes and writes this [Package] in DAT format to the specified output.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_output_dat] instead.
    pub fn into_output_dat<O: Write + Seek>(self, output: O) -> Result<(), PackageWriteError> {
        self.into_output(output, DatWriter())
    }

    /// Writes this [Package] in DAT format to file at the specified path.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_path_dat] instead.
    pub fn to_path_dat<P: AsRef<Path>>(&self, destination_path: P) -> Result<(), PackageWriteError> {
        self.to_path(destination_path, DatWriter())
    }

    /// Writes this [Package] in DAT format to the specified output.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_output_dat] instead.
    pub fn to_output_dat<O: Write + Seek>(&self, output: O) -> Result<(), PackageWriteError> {
        self.to_output(output, DatWriter())
    }
    // endregion

    // region <PKG>
    /// Consumes and writes this [Package] in PKG format to file at the specified path.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_path_pkg] instead.
    pub fn into_path_pkg<P: AsRef<Path>>(self, destination_path: P) -> Result<(), PackageWriteError> {
        self.into_path(destination_path, PkgWriter())
    }

    /// Consumes and writes this [Package] in PKG format to the specified output.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_output_pkg] instead.
    pub fn into_output_pkg<O: Write + Seek>(self, output: O) -> Result<(), PackageWriteError> {
        self.into_output(output, PkgWriter())
    }

    /// Writes this [Package] in PKG format to file at the specified path.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_path_pkg] instead.
    pub fn to_path_pkg<P: AsRef<Path>>(&self, destination_path: P) -> Result<(), PackageWriteError> {
        self.to_path(destination_path, PkgWriter())
    }

    /// Writes this [Package] in PKG format to the specified output.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_output_pkg] instead.
    pub fn to_output_pkg<O: Write + Seek>(&self, output: O) -> Result<(), PackageWriteError> {
        self.to_output(output, PkgWriter())
    }
    // endregion

    /// Consumes and writes this [Package] using format provided by the specified [PackageWriter],
    /// to file at the specified path.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_output] instead.
    pub fn into_path<P: AsRef<Path>, T: PackageWriter>(self, destination_path: P, writer: T) -> Result<(), PackageWriteError> {
        let destination_path = destination_path.as_ref();
        let destination_path_tmp = destination_path.with_extension("tmp");

        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&destination_path_tmp)?;

        println!("exists: {}", destination_path_tmp.exists());

        self.into_output(BufWriter::new(file), writer)?;

        println!("exists: {}", destination_path_tmp.exists());

        std::fs::remove_file(destination_path)?;
        std::fs::rename(destination_path_tmp, destination_path)?;

        Ok(())
    }

    /// Consumes and writes this [Package] using format provided by the specified [PackageWriter],
    /// to the specified output.
    ///
    /// This method consumes the [Package], therefore this method can overwrite the file from which
    /// the [Package] was originally created, even if the [PackageWriter] implementation locks file
    /// system resources.
    ///
    /// For a non-consuming variant, see [Package::to_output] instead.
    pub fn into_output<O: Write + Seek, T: PackageWriter>(self, output: O, writer: T) -> Result<(), PackageWriteError> {
        writer.write_package_to_output(&self, output)
    }

    /// Writes this [Package] using format provided by the specified [PackageWriter], to file at
    /// the specified path.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_path] instead.
    pub fn to_path<P: AsRef<Path>, T: PackageWriter>(&self, destination_path: P, writer: T) -> Result<(), PackageWriteError> {
        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(destination_path)?;

        self.to_output(BufWriter::new(file), writer)
    }

    /// Writes this [Package] using format provided by the specified [PackageWriter], to the
    /// specified output.
    ///
    /// This method does not consume the [Package], so if the [PackageWriter] implementation locks
    /// file system resources, this method will not be able to overwrite the file from which the
    /// [Package] was originally created.
    ///
    /// If this is what you want to do, use [Package::into_output] instead.
    pub fn to_output<O: Write + Seek, T: PackageWriter>(&self, output: O, writer: T) -> Result<(), PackageWriteError> {
        writer.write_package_to_output(self, output)
    }
    // endregion

    /// Adds the specified entry to this [Package].
    /// Returns an [InnerPathAlreadyExistsError] if this [Package] already contains an entry under
    /// the specified entry's `inner_path`.
    pub fn add_entry(&mut self, entry: PackageEntry) -> Result<(), InnerPathAlreadyExistsError> {
        let inner_path = entry.inner_path().to_string();
        if self.inner_path_to_entry_index.contains_key(&inner_path) {
            return Err(InnerPathAlreadyExistsError(inner_path));
        }

        self.append_new_entry(entry);
        Ok(())
    }

    /// Puts the specified entry into this [Package],
    /// overwriting any entry that may have been previously stored under that entry's `inner_path`.
    pub fn put_entry(&mut self, entry: PackageEntry) {
        let inner_path = entry.inner_path();
        let maybe_index = self.inner_path_to_entry_index.get(inner_path);
        match maybe_index {
            Some(index) => {
                let index = *index;
                self.entries.remove(index);
                self.entries.insert(index, entry);
            }
            None => {
                self.append_new_entry(entry);
            }
        }
    }

    /// Retrieves content under the `inner_path` in this [Package].
    ///
    /// Returns a copy of the content if found, or `None` if the `inner_path` doesn't
    /// have any entry associated with it.
    pub fn content_by_path<S: AsRef<str>>(&self, inner_path: S) -> Option<Vec<u8>> {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path.as_ref());
        match maybe_index {
            Some(index) => {
                self.entries.get(*index)
                    .map(|entry| entry.content().unwrap())
            }
            None => None
        }
    }

    /// Removes the entry under the specified `inner_path` from this [Package].
    ///
    /// Returns `true` if the entry was removed, `false` if no entry was found under the
    /// specified path.
    pub fn remove_entry<S: AsRef<str>>(&mut self, inner_path: S) -> bool {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path.as_ref());
        match maybe_index {
            Some(index) => {
                self.entries.remove(*index);
                true
            }
            None => false
        }
    }

    /// Checks if this [Package] has any entry associated with the given `inner_path`.
    ///
    /// Returns `true` if an entry is found, `false` otherwise.
    pub fn entry_exists<S: AsRef<str>>(&self, inner_path: S) -> bool {
        self.inner_path_to_entry_index.get(inner_path.as_ref()).is_some()
    }

    /// Removes all entries from this [Package].
    pub fn clear(&mut self) {
        self.inner_path_to_entry_index.clear();
        self.entries.clear();
    }

    /// Returns a view of `inner_path`s in this [Package], reflecting the internal order of
    /// entries within the package.
    pub fn inner_paths(&self) -> Vec<String> {
        self.entries.iter()
            .map(|e| e.inner_path().to_string())
            .collect()
    }

    /// Returns the number of entries in this [Package].
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    fn append_new_entry(&mut self, entry: PackageEntry) {
        let index = self.entries.len();
        self.inner_path_to_entry_index.insert(entry.inner_path().to_string(), index);
        self.entries.push(entry);
    }

    /// Returns an iterator over this [Package]'s [entries](PackageEntry).
    pub fn iter(&self) -> Iter<'_, PackageEntry> {
        self.entries.iter()
    }

    /// Extracts all [entries](PackageEntry) in this [Package] into the specified directory.
    /// The complete directory structure will be created if it doesn't exist yet.
    pub fn extract<P: AsRef<Path>>(&self, destination_path: P) -> Result<(), std::io::Error> {
        let destination_path = destination_path.as_ref();

        for entry in self.iter() {
            let entry_dest_path = destination_path.join(entry.inner_path());

            std::fs::create_dir_all(&entry_dest_path.parent().unwrap())?;
            let mut file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&entry_dest_path)?;

            let result = file.write(entry.content()?.as_ref());

            if let Err(error) = result {
                return Err(error);
            }
        }

        Ok(())
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Package [entries: '{}']", self.entry_count())
    }
}
