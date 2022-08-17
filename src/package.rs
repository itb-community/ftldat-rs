use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::*;

use crate::entry::Entry;

/// Represents the internal structure of an FTLDat package.
///
/// These packages consist of a list of Entries. For ease of use, this struct stores Entries in a
/// map.
///
/// Within the FtlDat file on disk, this package is laid out as follows:
/// - `index_size` := size of the index (1x u32)
/// - offsets to [Entries](Entry) (`index_size` x u32)
/// - [Entries](Entry) (`index_size` x [Entry])
#[derive(Debug)]
pub struct Package {
    inner_path_to_entry: BTreeMap<String, Entry>,
}

impl Package {
    pub fn new() -> Package {
        Package {
            inner_path_to_entry: BTreeMap::new()
        }
    }

    pub fn from_reader(mut input: (impl Read + Seek)) -> Result<Package, Error> {
        let start_pos = input.stream_position()?;

        let mut do_read = || -> Result<Package, Error> {
            let mut result = Package::new();
            input.seek(SeekFrom::Start(0))?;

            let index_size = input.read_u32::<LittleEndian>()
                .expect("Failed to read index length") as usize;

            // TODO: Skip offsets and simply read entries until EOF?
            let mut entry_offsets = Vec::with_capacity(index_size);
            for _ in 0..index_size {
                let entry_offset = input.read_u32::<LittleEndian>()?;
                entry_offsets.push(entry_offset);
            }

            for entry_offset in entry_offsets {
                input.seek(SeekFrom::Start(entry_offset as u64))?;

                let entry = Entry::from_reader(&mut input)
                    .expect("Failed to read Entry");

                result.add_entry(entry)?;
            }

            Ok(result)
        };

        match do_read() {
            Ok(result) => Ok(result),
            Err(source) => Err(Error::read_failed(start_pos, input.stream_position()?))
        }
    }

    pub fn add_entry(&mut self, entry: Entry) -> Result<(), Error> {
        if self.entry_exists_by_path(&entry.inner_path) {
            return Err(Error::inner_path_already_exists(entry.inner_path));
        }

        self.inner_path_to_entry.insert(entry.inner_path.to_string(), entry);
        Ok(())
    }

    pub fn entry_exists_by_path(&mut self, inner_path: &str) -> bool {
        self.inner_path_to_entry.contains_key(inner_path)
    }

    pub fn write(&self, mut output: (impl Write + Seek)) -> Result<(), Error> {
        let index_size = self.inner_path_to_entry.len();
        // Index size
        output.write_u32::<LittleEndian>(index_size as u32)?;

        // Reserve space for entry offsets
        output.seek(SeekFrom::Start((4 + 4 * index_size) as u64))?;

        // Write Entries and store the offsets they were written at
        let mut entry_offsets = Vec::with_capacity(index_size);
        for entry in self.inner_path_to_entry.values() {
            entry_offsets.push(output.stream_position()? as u32);
            output.write_all(&entry.bytes()?.to_bytes())?;
        }

        // Go back to write offsets to Entries in the index
        output.seek(SeekFrom::Start(4))?;
        for offset in entry_offsets {
            output.write_u32::<LittleEndian>(offset)?;
        }

        Ok(())
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Entry [entries: '{}']",
            self.inner_path_to_entry.len()
        )
    }
}
