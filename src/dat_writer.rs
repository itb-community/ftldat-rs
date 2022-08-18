use std::io::{Seek, SeekFrom, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::entry::Entry;
use crate::error::Error;
use crate::package::Package;

pub(crate) struct DatWriter {}

impl DatWriter {
    pub(crate) fn write_package(package: &Package, mut output: (impl Write + Seek)) -> Result<(), Error> {
        let index_size = package.len();
        // Index size
        output.write_u32::<LittleEndian>(index_size as u32)?;

        // Reserve space for entry offsets
        output.seek(SeekFrom::Start((4 + 4 * index_size) as u64))?;

        // Write Entries and store the offsets they were written at
        let mut entry_offsets = Vec::with_capacity(index_size);
        for entry in package.entries() {
            entry_offsets.push(output.stream_position()? as u32);
            DatWriter::write_entry(entry, &mut output)?;
        }

        // Go back to write offsets to Entries in the index
        output.seek(SeekFrom::Start(4))?;
        for offset in entry_offsets {
            output.write_u32::<LittleEndian>(offset)?;
        }

        Ok(())
    }

    pub(crate) fn write_entry(entry: &Entry, output: &mut impl Write) -> Result<(), Error> {
        // Data size
        output.write_u32::<LittleEndian>(entry.content.len() as u32)?;
        // String length (inner_path)
        output.write_u32::<LittleEndian>(entry.inner_path.len() as u32)?;
        // Actual string (inner_path)
        output.write_all(entry.inner_path.as_bytes())?;
        // Data
        output.write_all(&entry.content)?;

        Ok(())
    }
}