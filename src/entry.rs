use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, Write};

use bytebuffer::{ByteBuffer, Endian};
use byteorder::{LittleEndian, ReadBytesExt};

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
    pub fn from_string(inner_path: &str, content: &str) -> Entry {
        Entry {
            inner_path: inner_path.to_string(),
            content: Vec::from(content),
        }
    }

    pub fn from_bytes(inner_path: &str, content: &[u8]) -> Entry {
        Entry {
            inner_path: inner_path.to_string(),
            content: Vec::from(content),
        }
    }

    pub fn from_reader(input: &mut (impl Read + Seek)) -> Result<Entry, Error> {
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
            Err(source) => Err(Error::read_failed(start_pos, input.stream_position()?))
        }
    }

    pub(crate) fn bytes(&self) -> Result<ByteBuffer, Error> {
        let mut buffer = ByteBuffer::new();
        buffer.set_endian(Endian::LittleEndian);

        // Data size
        buffer.write_u32(self.content.len() as u32);
        // String length (inner_path)
        buffer.write_u32(self.inner_path.len() as u32);
        // Actual string (inner_path)
        buffer.write_all(self.inner_path.as_bytes())?;
        // Data
        buffer.write_all(&self.content)?;

        Ok(buffer)
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

impl PartialEq<str> for Entry {
    fn eq(&self, other: &str) -> bool {
        self.inner_path.eq(other)
    }
}
