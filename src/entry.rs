use std::fmt::{Display, Formatter};

use crate::error::*;

/// Represents a file entry in FtlDat.
///
/// These entries consist basically only of the file's path within the package (here called
/// `inner_path`), and the file's binary content.
#[derive(Debug)]
pub struct Entry {
    pub(crate) inner_path: String,
    pub(crate) content: Vec<u8>,
}

impl Entry {
    //region <Constructors>
    /// Constructs an [Entry] from the given `inner_path` and text `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    pub fn from_string(inner_path: String, content: String) -> Entry {
        Entry {
            inner_path,
            content: Vec::from(content),
        }
    }

    /// Constructs an [Entry] from the given `inner_path` and binary `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - binary content of the file.
    pub fn from_bytes(inner_path: String, content: Vec<u8>) -> Entry {
        Entry {
            inner_path,
            content,
        }
    }

    /// Constructs an [Entry] from the given `inner_path` and text `content`. Copies the strings
    /// to have the Entry gain ownership over them. This method is primarily for testing convenience.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    pub fn from(inner_path: &str, content: &str) -> Entry {
        Entry {
            inner_path: inner_path.to_owned(),
            content: Vec::from(content.to_owned()),
        }
    }

    /// Constructs an [Entry] from the given `inner_path` and [File] at the specified `source_path`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `source_path` - path to a file whose content will be stored by the created [Entry].
    pub fn from_file(inner_path: String, source_path: String) -> Result<Entry, Error> {
        let bytes = std::fs::read(source_path)?;
        Ok(Entry::from_bytes(inner_path, bytes))
    }
    //endregion

    /// Returns the `inner_path` of this entry.
    pub fn inner_path(&self) -> &str {
        &self.inner_path
    }

    /// Returns a view of this entry's content as bytes.
    pub fn content_bytes(&self) -> &[u8] {
        &self.content
    }

    /// Returns a string representation of this entry's content.
    pub fn content_string(&self) -> String {
        String::from_utf8(self.content.to_owned()).expect("Invalid UTF-8 sequence")
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Entry [inner_path: '{}', content_length: {}]",
            self.inner_path, self.content.len()
        )
    }
}
