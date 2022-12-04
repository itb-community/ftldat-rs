use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::slice::Iter;

use crate::shared::entry::Entry;
use crate::shared::error::*;

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

    //region <API>
    /// Adds a new entry to this [Package] with the specified `inner_path` and text `content`.
    /// Returns an [InnerPathAlreadyExistsError] if this [Package] already contains an [Entry] under
    /// the specified `inner_path`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    pub fn add_entry_from_string<S: AsRef<str>, C: AsRef<str> + Into<Vec<u8>>>(&mut self, inner_path: S, content: C) -> Result<(), InnerPathAlreadyExistsError> {
        let ref_inner_path = inner_path.as_ref();
        if self.inner_path_to_entry_index.contains_key(ref_inner_path) {
            return Err(InnerPathAlreadyExistsError(ref_inner_path.to_owned()));
        }

        self.append_new_entry(Entry::from_string(ref_inner_path, content));
        Ok(())
    }

    pub fn add_entry_from_byte_array<S: AsRef<str>>(&mut self, inner_path: S, content: Vec<u8>) -> Result<(), InnerPathAlreadyExistsError> {
        let ref_inner_path = inner_path.as_ref();
        if self.inner_path_to_entry_index.contains_key(ref_inner_path) {
            return Err(InnerPathAlreadyExistsError(ref_inner_path.to_owned()));
        }

        self.append_new_entry(Entry::from_byte_array(ref_inner_path, content));
        Ok(())
    }

    /// Puts the specified text `content` into this [Package] under the specified `inner_path`,
    /// overwriting any entry that may have been previously stored under that `inner_path`.
    pub fn put_entry_from_string<S: AsRef<str>, C: AsRef<str> + Into<Vec<u8>>>(&mut self, inner_path: S, content: C) {
        let ref_inner_path = inner_path.as_ref();
        let maybe_index = self.inner_path_to_entry_index.get(ref_inner_path);
        let entry = Entry::from_string(ref_inner_path, content);
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

    /// Puts the specified binary `content` into this [Package] under the specified `inner_path`,
    /// overwriting any entry that may have been previously stored under that `inner_path`.
    pub fn put_entry_from_byte_array<S: AsRef<str>>(&mut self, inner_path: S, content: Vec<u8>) {
        let ref_inner_path = inner_path.as_ref();
        let maybe_index = self.inner_path_to_entry_index.get(ref_inner_path);
        let entry = Entry::from_byte_array(ref_inner_path, content);
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

    /// Adds the specified `entry` into this [Package]. Returns an [Error] if this [Package]
    /// already contains an [Entry] under the same `inner_path` as the specified `entry`.
    pub(crate) fn add_entry_internal(&mut self, entry: Entry) -> Result<(), InnerPathAlreadyExistsError> {
        if self.inner_path_to_entry_index.contains_key(&entry.inner_path) {
            return Err(InnerPathAlreadyExistsError(entry.inner_path));
        }

        self.append_new_entry(entry);
        Ok(())
    }

    fn append_new_entry(&mut self, entry: Entry) {
        let index = self.entries.len();
        self.inner_path_to_entry_index.insert(entry.inner_path.to_string(), index);
        self.entries.push(entry);
    }

    /// Retrieves content under the `inner_path` in this [Package].
    ///
    /// Returns a copy of the text content if found, or `None` if the `inner_path` doesn't
    /// have any entry associated with it.
    pub fn string_content_by_path<S: AsRef<str>>(&self, inner_path: S) -> Option<String> {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path.as_ref());
        match maybe_index {
            Some(index) => {
                self.entries.get(*index)
                    .map(|entry| entry.content_string())
            }
            None => None
        }
    }

    /// Retrieves content under the `inner_path` in this [Package].
    ///
    /// Returns a copy of the binary content if found, or `None` if the `inner_path` doesn't
    /// have any entry associated with it.
    pub fn byte_array_content_by_path<S: AsRef<str>>(&self, inner_path: S) -> Option<Vec<u8>> {
        let maybe_index = self.inner_path_to_entry_index.get(inner_path.as_ref());
        match maybe_index {
            Some(index) => {
                self.entries.get(*index)
                    .map(|entry| entry.content_bytes().to_vec())
            }
            None => None
        }
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
    pub fn entry_exists(&self, inner_path: &str) -> bool {
        self.inner_path_to_entry_index.contains_key(inner_path)
    }

    /// Removes all [Entries](Entry) from this [Package].
    pub fn clear(&mut self) {
        self.entries.clear();
        self.inner_path_to_entry_index.clear();
    }

    /// Returns an iterator over this [Package]'s [Entries](Entry).
    pub(crate) fn iter(&self) -> Iter<Entry> {
        self.entries.iter()
    }

    /// Alias for [Package::iter]
    pub(crate) fn entries(&self) -> Iter<Entry> {
        self.iter()
    }

    /// Returns a view of `inner_path`s in this [Package], reflecting the internal order of
    /// entries within the package..
    pub fn inner_paths(&self) -> Vec<String> {
        self.iter()
            .map(|entry| entry.inner_path().to_owned())
            .collect::<Vec<String>>()
    }

    /// Returns the number of [Entries](Entry) in this [Package].
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns the number of [Entries](Entry) in this [Package].
    pub fn entry_count(&self) -> usize {
        self.len()
    }

    pub fn extract<P: AsRef<Path>>(&self, destination_path: P) -> Result<(), std::io::Error> {
        for entry in self.iter() {
            let entry_dest_path = destination_path.as_ref().join(entry.inner_path());

            std::fs::create_dir_all(&entry_dest_path.parent().unwrap())?;
            let mut file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&entry_dest_path)
                .expect(&format!("Failed to open the file for writing: {}", &entry_dest_path.to_str().unwrap()));

            let result = file.write(entry.content_bytes());

            if let Err(error) = result {
                return Err(error);
            }
        }

        Ok(())
    }
    //endregion
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entry [entries: '{}']", self.len())
    }
}
