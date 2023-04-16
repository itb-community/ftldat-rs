use std::io::{Seek, SeekFrom, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::PackageWriter;
use crate::shared::entry::PackageEntry;
use crate::shared::error::PackageWriteError;
use crate::shared::package::Package;

pub struct DatWriter();

impl PackageWriter for DatWriter {
    fn write_package_to_output<T: Write + Seek>(&self, package: &Package, mut output: T) -> Result<(), PackageWriteError> {
        let index_size = package.entry_count();
        // Index size
        output.write_u32::<LittleEndian>(index_size as u32)?;

        // Reserve space for entry offsets
        output.seek(SeekFrom::Start((4 + 4 * index_size) as u64))?;

        // Write Entries and store the offsets they were written at
        let mut entry_offsets = Vec::with_capacity(index_size);

        for entry in package.iter() {
            entry_offsets.push(output.stream_position()? as u32);
            write_entry(entry, &mut output)
                .expect("Failed to write entry");
        }

        // Go back to write offsets to Entries in the index
        output.seek(SeekFrom::Start(4))?;
        for offset in entry_offsets {
            output.write_u32::<LittleEndian>(offset)?;
        }

        Ok(())
    }
}

fn write_entry(entry: &PackageEntry, output: &mut impl Write) -> Result<(), PackageWriteError> {
    let inner_path = entry.inner_path();
    let content = entry.content()?;
    // Data size
    output.write_u32::<LittleEndian>(content.len() as u32)?;
    // String length (inner_path)
    output.write_u32::<LittleEndian>(inner_path.len() as u32)?;
    // Actual string (inner_path)
    output.write_all(inner_path.as_bytes())?;
    // Data
    output.write_all(content.as_ref())?;

    Ok(())
}
