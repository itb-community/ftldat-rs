use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use memmap2::Mmap;

// Documentation imports
#[allow(unused)]
use crate::Package;

/// Represents a file entry in a [`Package`].
///
/// These entries consist basically only of the file's path within the package (here called an
/// `inner_path`), and a reference to the file's content.
#[derive(Debug)]
pub struct PackageEntry {
    inner_path: String,
    source: DataSource,
}

/// Represents the source of a [`PackageEntry`]'s content.
/// Where possible, each entry's content is not stored in-memory, but rather sourced from
/// the source package from which the entry was originally read, or from a file on disk,
/// and read only when this data is actually needed.
#[derive(Debug)]
enum DataSource {
    FileOnDisk(PathBuf),
    MemoryMappedFile(Rc<Mmap>, u64, u64),
    InMemoryByteArray(Vec<u8>),
}

impl PackageEntry {
    /// Constructs an [`PackageEntry`] from the given `inner_path` and file at the specified `path`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [`Package`].
    /// * `path` - path to the file that will be read to populate this entry's content.
    pub fn from_file<S: AsRef<str>, P: AsRef<Path>>(
        inner_path: S,
        path: P,
    ) -> PackageEntry {
        PackageEntry {
            inner_path: inner_path.as_ref().to_string(),
            source: DataSource::FileOnDisk(PathBuf::from(path.as_ref())),
        }
    }

    /// Constructs an [`PackageEntry`] from the given `inner_path`, memory map, offset and length
    ///
    /// * `inner_path` - path under which the file will be stored within the [`Package`].
    /// * `mmap` - memory map of the file from which the file's content will be read.
    /// * `offset` - offset to the file's content within the memory mapped file.
    /// * `length` - length of the file's content within the memory mapped file.
    pub fn from_memory_mapped_file<S: AsRef<str>>(
        inner_path: S,
        mmap: Rc<Mmap>,
        offset: u64,
        length: u64,
    ) -> PackageEntry {
        PackageEntry {
            inner_path: inner_path.as_ref().to_string(),
            source: DataSource::MemoryMappedFile(
                mmap,
                offset,
                length,
            ),
        }
    }

    /// Constructs an [`PackageEntry`] from the given `inner_path` and text `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [`Package`].
    /// * `content` - textual content of the file.
    pub fn from_string<S: AsRef<str>, C: AsRef<str> + Into<Vec<u8>>>(
        inner_path: S,
        content: C,
    ) -> PackageEntry {
        PackageEntry::from_byte_array(
            inner_path,
            content.into(),
        )
    }

    /// Constructs an [`PackageEntry`] from the given `inner_path` and binary `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [`Package`].
    /// * `content` - binary content of the file.
    pub fn from_byte_array<S: AsRef<str>>(
        inner_path: S,
        content: Vec<u8>,
    ) -> PackageEntry {
        PackageEntry {
            inner_path: inner_path.as_ref().to_string(),
            source: DataSource::InMemoryByteArray(content),
        }
    }

    /// Returns the `inner_path` of this entry.
    pub fn inner_path(&self) -> &str {
        &self.inner_path
    }

    /// Returns a view of this entry's content as bytes.
    pub fn content(&self) -> Result<Vec<u8>, std::io::Error> {
        match &self.source {
            DataSource::InMemoryByteArray(slice) => {
                Ok(slice.to_vec())
            }
            DataSource::FileOnDisk(path) => {
                let file = File::options()
                    .read(true)
                    .open(path)
                    .expect("Failed to open file for reading");
                let mut reader = BufReader::new(file);

                let mut buffer = Vec::with_capacity(1024);
                reader.read_to_end(&mut buffer)?;

                Ok(buffer)
            }
            DataSource::MemoryMappedFile(mmap, offset, length) => {
                let offset = *offset as usize;
                let length = *length as usize;
                let slice = mmap[offset..offset + length].to_vec();
                Ok(slice)
            }
        }
    }
}

impl Display for PackageEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Entry [inner_path: '{}', source: {}]",
            self.inner_path, self.source
        )
    }
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "DataSource [{:?}]",
            self
        )
    }
}