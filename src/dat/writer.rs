use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{Entry, Package};
use crate::shared::error::PackageWriteError;

/// Writes out the specified [Package] in binary FtlDat format to a file at the specified `target_path`.
pub fn write_package_to_path<P: AsRef<Path>>(package: Package, target_path: P) -> Result<(), PackageWriteError> {
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(target_path)
        .expect("Failed to open the file for writing");

    write_package_to_output(package, BufWriter::new(file))
}

/// Writes out the specified [Package] in binary FtlDat format to the given `output`,
/// consuming it in the process.
pub fn write_package_to_output(package: Package, mut output: (impl Write + Seek)) -> Result<(), PackageWriteError> {
    let index_size = package.len();
    // Index size
    output.write_u32::<LittleEndian>(index_size as u32)?;

    // Reserve space for entry offsets
    output.seek(SeekFrom::Start((4 + 4 * index_size) as u64))?;

    // Write Entries and store the offsets they were written at
    let mut entry_offsets = Vec::with_capacity(index_size);
    for entry in package.entries() {
        entry_offsets.push(output.stream_position()? as u32);
        write_entry(entry, &mut output)?;
    }

    // Go back to write offsets to Entries in the index
    output.seek(SeekFrom::Start(4))?;
    for offset in entry_offsets {
        output.write_u32::<LittleEndian>(offset)?;
    }

    Ok(())
}

fn write_entry(entry: &Entry, output: &mut impl Write) -> Result<(), PackageWriteError> {
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
