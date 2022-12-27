use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

use byteorder::{BigEndian, WriteBytesExt};

use crate::pkg_package::constants::{ENTRY_SIZE, INDEX_SIZE, PKG_SIGNATURE};
use crate::pkg_package::error::PkgWriteError;
use crate::pkg_package::shared::calculate_path_hash;
use crate::shared::entry::Entry;
use crate::shared::error::PackageWriteError;
use crate::shared::package::Package;

/// Writes out the specified [Package] in binary PKG format to a file at the specified `target_path`.
pub fn write_package_to_path<P: AsRef<Path>>(package: Package, target_path: P) -> Result<(), PackageWriteError> {
    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(target_path)
        .expect("Failed to open the file for writing");

    write_package_to_output(package, BufWriter::new(file))
}

/// Writes out the specified [Package] in binary PKG format to the given `output`,
/// consuming it in the process.
pub fn write_package_to_output(package: Package, mut output: (impl Write + Seek)) -> Result<(), PackageWriteError> {
    output.write_all(&PKG_SIGNATURE)?;
    output.write_u16::<BigEndian>(INDEX_SIZE)?;
    output.write_u16::<BigEndian>(ENTRY_SIZE)?;

    if package.entry_count() > u32::MAX as usize {
        return Err(PkgWriteError::EntryCountExceededError().into());
    }

    output.write_u32::<BigEndian>(package.entry_count() as u32)?;

    let mut data_offset: u32 = 0;
    let mut entry_headers: Vec<EntryHeader> = Vec::with_capacity(package.entry_count());
    let mut path_region_buffer: Vec<u8> = Vec::new();
    for entry in package.entries() {
        let mut entry_header = EntryHeader::from(entry);
        entry_header.inner_path_offset = path_region_buffer.len() as u32;
        entry_header.data_offset = data_offset;
        data_offset += entry.content.len() as u32;

        path_region_buffer.extend_from_slice(entry.inner_path.as_bytes());
        // Append null terminator
        path_region_buffer.write_u8(0_u8)?;

        entry_headers.push(entry_header);
    }

    if path_region_buffer.len() > u32::MAX as usize {
        return Err(PkgWriteError::PathAreaSizeExceededError(path_region_buffer.len()).into());
    }

    output.write_u32::<BigEndian>(path_region_buffer.len() as u32)?;

    let data_region_offset = INDEX_SIZE as u64
        + (ENTRY_SIZE as u64 * package.entry_count() as u64)
        + path_region_buffer.len() as u64
        + (4 - (path_region_buffer.len() as u64 % 4));

    for mut entry_header in entry_headers {
        entry_header.data_offset += data_region_offset as u32;
        entry_header.write_entry_header(&mut output)?;
    }

    output.write_all(&path_region_buffer)?;
    drop(path_region_buffer);

    output.seek(SeekFrom::Start(data_region_offset))?;
    for entry in package.entries() {
        output.write_all(&entry.content)?;
    }

    Ok(())
}

struct EntryHeader {
    inner_path_hash: u32,
    entry_options: u8,
    inner_path_offset: u32,
    data_offset: u32,
    data_size: u32,
    unpacked_data_size: u32,
}

impl EntryHeader {
    fn write_entry_header(self, output: &mut impl Write) -> Result<(), PackageWriteError> {
        output.write_u32::<BigEndian>(self.inner_path_hash)?;
        output.write_u8(self.entry_options)?;
        output.write_u24::<BigEndian>(self.inner_path_offset)?;
        output.write_u32::<BigEndian>(self.data_offset)?;
        output.write_u32::<BigEndian>(self.data_size)?;
        output.write_u32::<BigEndian>(self.unpacked_data_size)?;

        Ok(())
    }
}

impl From<&Entry> for EntryHeader {
    fn from(entry: &Entry) -> Self {
        EntryHeader {
            inner_path_hash: calculate_path_hash(&entry.inner_path),
            // We do not support deflated entries, so always write out 0 for entry options.
            entry_options: 0,
            inner_path_offset: 0,
            data_offset: 0,
            data_size: entry.content.len() as u32,
            unpacked_data_size: entry.content.len() as u32,
        }
    }
}