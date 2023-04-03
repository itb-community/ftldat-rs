use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Write};
use std::path::{Path};
use std::slice::Iter;

use crate::error::InnerPathAlreadyExistsError;
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
    /// Creates a new empty [Package].
    pub fn new() -> Package {
        Package {
            entries: Vec::new(),
            inner_path_to_entry_index: BTreeMap::new(),
        }
    }

    /// Creates a new empty [Package] with at least the specified capacity.
    pub fn with_capacity(capacity: usize) -> Package {
        Package {
            entries: Vec::with_capacity(capacity),
            inner_path_to_entry_index: BTreeMap::new(),
        }
    }

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

    pub fn extract<P: AsRef<Path>>(&self, destination_path: P) -> Result<(), std::io::Error> {
        let destination_path = destination_path.as_ref();

        for entry in self.iter() {
            let entry_dest_path = destination_path.join(entry.inner_path());

            std::fs::create_dir_all(&entry_dest_path.parent().unwrap())?;
            let mut file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&entry_dest_path)
                .expect(&format!("Failed to open the file for writing: {}", &entry_dest_path.to_str().unwrap()));

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
