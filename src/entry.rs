use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::*;

/// Represents a file entry in FtlDat.
///
/// These entries consist basically only of the file's path within the package (here called
/// `inner_path`), and the file's binary content.
///
/// Within the FtlDat file on disk, these entries are laid out as follows:
/// - `data_size` := file content length (1x u32)
/// - `str_len` := file name length (1x u32)
/// - file name (`str_len` x u8)
/// - file content (`data_size` x u8)
#[derive(Debug)]
pub struct Entry {
    pub(crate) inner_path: String,
    content: Vec<u8>,
}

impl Entry {
    //region <Constructors>
    /// Constructs an [Entry] from the given `inner_path` and text `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - textual content of the file.
    pub fn from_string(inner_path: &str, content: &str) -> Entry {
        Entry {
            inner_path: inner_path.to_string(),
            content: Vec::from(content),
        }
    }

    /// Constructs an [Entry] from the given `inner_path` and binary `content`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `content` - binary content of the file.
    pub fn from_bytes(inner_path: &str, content: &[u8]) -> Entry {
        Entry {
            inner_path: inner_path.to_string(),
            content: Vec::from(content),
        }
    }

    /// Constructs an [Entry] from the given `inner_path` and [File] at the specified `source_path`.
    ///
    /// * `inner_path` - path under which the file will be stored within the [Package].
    /// * `source_path` - path to a file whose content will be stored by the created [Entry].
    pub fn from_file(inner_path: &str, source_path: &str) -> Result<Entry, Error> {
        let bytes = std::fs::read(source_path)?;
        Ok(Entry::from_bytes(inner_path, &bytes))
    }
    //endregion

    pub(crate) fn from_reader(input: &mut (impl Read + Seek)) -> Result<Entry, Error> {
        let start_pos = input.stream_position()?;

        let mut do_read = || -> Result<Entry, Error> {
            let content_length = input.read_u32::<LittleEndian>()
                .expect("Failed to read content length");
            let inner_path_length = input.read_u32::<LittleEndian>()
                .expect("Failed to read inner_path length");

            let mut buffer = vec![0_u8; inner_path_length as usize];
            input.read_exact(&mut buffer)
                .expect("Failed to read inner_path");
            let inner_path = String::from_utf8(buffer)
                .expect("Failed to interpret inner_path bytes as utf8");

            let mut buffer = vec![0_u8; content_length as usize];
            input.read_exact(&mut buffer)
                .expect("Failed to read content");

            Ok(Entry {
                inner_path,
                content: buffer,
            })
        };

        match do_read() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::read_failed(start_pos, input.stream_position()?))
        }
    }

    pub(crate) fn write(&self, output: &mut impl Write) -> Result<(), Error> {
        // Data size
        output.write_u32::<LittleEndian>(self.content.len() as u32)?;
        // String length (inner_path)
        output.write_u32::<LittleEndian>(self.inner_path.len() as u32)?;
        // Actual string (inner_path)
        output.write_all(self.inner_path.as_bytes())?;
        // Data
        output.write_all(&self.content)?;

        Ok(())
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
