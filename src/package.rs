use std::collections::btree_map::Keys;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::slice::Iter;

use crate::dat_reader::DatReader;
use crate::dat_writer::DatWriter;
use crate::entry::Entry;
use crate::error::*;

/// Represents the internal structure of an FTLDat package.
///
/// These packages consist of a list of [Entries](Entry).
#[derive(Debug)]
pub struct Package {
    /// Use a Vec as main [Entry] storage; this way we retain the order in which the source
    /// FtlDat file originally stored its entries.
    entries: Vec<Entry>,
    inner_path_to_entry_index: BTreeMap<String, usize>,
}

impl Package {
    //region <Constructors>
    /// Constructs a new empty [Package] with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Package {
        Package {
            entries: Vec::with_capacity(capacity),
            inner_path_to_entry_index: BTreeMap::new(),
        }
    }

    /// Constructs a new empty [Package] instance, initialized with a capacity for 2048 entries.
    pub fn new() -> Package {
        Package::with_capacity(2048)
    }
    //endregion

    //region <Input/Output>
    /// Reads and creates a [Package] instance out of the specified [File].
    pub fn from_file(source_path: &str) -> Result<Package, Error> {
        let file = File::options()
            .read(true)
            .open(source_path)
            .expect("Failed to open the file for reading");
        Package::from_reader(BufReader::new(file))
    }

    /// Constructs a [Package] instance from data in the given `input',
    /// consuming it in the process.
    pub fn from_reader(input: (impl Read + Seek)) -> Result<Package, Error> {
        DatReader::read_package(input)
    }

    /// Writes out this [Package] in binary FtlDat format to a file at the specified `target_path`.
    pub fn to_file(&self, target_path: &str) -> Result<(), Error> {
        let file = File::options()
            .write(true)
            .create(true)
            .open(target_path)
            .expect("Failed to open the file for writing");

        self.write(BufWriter::new(file))
    }

    /// Writes out this [Package] in binary FtlDat format to the given `output`,
    /// consuming it in the process.
    pub fn write(&self, output: (impl Write + Seek)) -> Result<(), Error> {
        DatWriter::write_package(self, output)
    }
    //endregion

    //region <API>
    /// Adds the specified `entry` into this [Package]. Returns an [Error] if this [Package]
    /// already contains an [Entry] under the same `inner_path` as the specified `entry`.
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Error> {
        if self.inner_path_to_entry_index.contains_key(&entry.inner_path) {
            return Err(Error::inner_path_already_exists(entry.inner_path));
        }

        self.append_new_entry(entry);
        Ok(())
    }

    /// Puts the specified [Entry] into this [Package], overwriting any [Entry] that may have
    /// been previously stored under the same `inner_path`.
    pub fn put_entry(&mut self, entry: Entry) {
        let maybe_index = self.inner_path_to_entry_index.get(&entry.inner_path);
        match maybe_index {
            Some(index) => {
                self.entries.remove(*index);
                self.entries.insert(*index, entry);
            }
            None => {
                self.append_new_entry(entry);
            }
        }
    }

    fn append_new_entry(&mut self, entry: Entry) {
        let index = self.entries.len();
        self.inner_path_to_entry_index.insert(entry.inner_path.to_string(), index);
        self.entries.push(entry);
    }

    /// Removes the [Entry] under the specified `inner_path` from this [Package].
    ///
    /// Returns `true` if the entry was removed, `false` if no entry was found under the
    /// specified path.
    pub fn remove_entry(&mut self, inner_path: &str) -> bool {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path);
        match maybe_index {
            Some(index) => {
                self.entries.remove(*index);
                self.inner_path_to_entry_index.remove(inner_path);
                true
            }
            None => false
        }
    }

    /// Checks if this [Package] has any [Entry] associated with the given `inner_path`.
    ///
    /// Returns `true` if an [Entry] is found, `false` otherwise.
    pub fn entry_exists(&mut self, inner_path: &str) -> bool {
        self.inner_path_to_entry_index.contains_key(inner_path)
    }

    /// Retrieves an entry by the `inner_path` under which it is stored in this [Package].
    ///
    /// Returns a reference to the [Entry] if found, or `None` if the `inner_path` doesn't
    /// have any entry associated with it.
    pub fn entry_by_path(&self, inner_path: &str) -> Option<&Entry> {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path);
        match maybe_index {
            Some(index) => self.entries.get(*index),
            None => None
        }
    }

    /// Removes all [Entries](Entry) from this [Package].
    pub fn clear(&mut self) {
        self.entries.clear();
        self.inner_path_to_entry_index.clear();
    }

    /// Returns an iterator over this [Package]'s [Entries](Entry).
    pub fn iter(&self) -> Iter<Entry> {
        self.entries.iter()
    }

    /// Alias for [Package::iter]
    pub fn entries(&self) -> Iter<Entry> {
        self.iter()
    }

    /// Returns an iterator over `inner_path`s in this [Package].
    pub fn inner_paths(&self) -> Keys<'_, String, usize> {
        self.inner_path_to_entry_index.keys().into_iter()
    }

    /// Returns the number of [Entries](Entry) in this [Package].
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    //endregion
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entry [entries: '{}']", self.len())
    }
}
