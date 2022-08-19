use std::fmt::{Display, Formatter};

/// Represents a file entry in FtlDat.
///
/// These entries consist basically only of the file's path within the package (here called
/// `inner_path`), and the file's binary content.
#[derive(Debug)]
pub(crate) struct Entry {
    pub(crate) inner_path: String,
    pub(crate) content: Vec<u8>,
}

impl Entry {
    /// Constructs an [Entry] from the given `inner_path` and text `content`. Copies the strings
    /// to have the Entry gain ownership over them. This method is primarily for testing convenience.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    pub fn _from(inner_path: &str, content: &str) -> Entry {
        Entry {
            inner_path: inner_path.to_owned(),
            content: Vec::from(content.to_owned()),
        }
    }

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

pub(crate) trait EntryFrom<T> {
    fn entry_from(inner_path: String, content: T) -> Entry;
}

impl EntryFrom<String> for Entry {
    /// Constructs an [Entry] from the given `inner_path` and text `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    fn entry_from(inner_path: String, content: String) -> Entry {
        Entry {
            inner_path,
            content: Vec::from(content),
        }
    }
}

impl EntryFrom<Vec<u8>> for Entry {
    /// Constructs an [Entry] from the given `inner_path` and binary `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - binary content of the file.
    fn entry_from(inner_path: String, content: Vec<u8>) -> Entry {
        Entry {
            inner_path,
            content,
        }
    }
}
