use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{Entry, Package};
use crate::shared::error::PackageReadError;

// Dat packages have the following structure:
// - `entry_count` := number of entries (1x u32)
// - offsets to Entries (`entry_count` x u32)
// - Entries (`entry_count` x Entry)
//
// Entries have the following structure:
// - `data_size` := file content length (1x u32)
// - `str_len` := file name length (1x u32)
// - file name (`str_len` x u8)
// - file content (`data_size` x u8)

/// Reads and creates a [Package] instance out of the specified [Path], using .dat format.
pub fn read_from_path<P: AsRef<Path>>(source_path: P) -> Result<Package, PackageReadError> {
    let file = File::options()
        .read(true)
        .open(source_path)
        .expect("Failed to open the file for reading");
    read_from_input(BufReader::new(file))
}

/// Constructs a [Package] instance from data in the given `input',
/// consuming it in the process.
pub fn read_from_input(mut input: (impl Read + Seek)) -> Result<Package, PackageReadError> {
    let mut result = Package::new();
    input.seek(SeekFrom::Start(0))?;

    let entry_count = input.read_u32::<LittleEndian>()? as usize;

    // TODO: Skip offsets and simply read entries until EOF?
    let mut entry_offsets = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        let entry_offset = input.read_u32::<LittleEndian>()?;
        entry_offsets.push(entry_offset);
    }

    for entry_offset in entry_offsets {
        input.seek(SeekFrom::Start(entry_offset as u64))?;

        let entry = read_entry(&mut input)?;

        result.add_entry_internal(entry)?;
    }

    Ok(result)
}

fn read_entry(input: &mut (impl Read + Seek)) -> Result<Entry, PackageReadError> {
    let content_length = input.read_u32::<LittleEndian>()?;
    let inner_path_length = input.read_u32::<LittleEndian>()?;

    let mut buffer = vec![0_u8; inner_path_length as usize];
    input.read_exact(&mut buffer)?;
    let inner_path = String::from_utf8(buffer)?;

    let mut buffer = vec![0_u8; content_length as usize];
    input.read_exact(&mut buffer)?;

    Ok(Entry::from_byte_array(inner_path, buffer))
}