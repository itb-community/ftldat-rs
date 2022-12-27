use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{Entry, Package};
use crate::error::{EntryReadError, EntryReadErrorImpl, PackageCorruptErrorImpl, PackageReadError};
use crate::pkg_package::constants::{ENTRY_SIZE, INDEX_SIZE, PKG_DEFLATED, PKG_SIGNATURE};
use crate::pkg_package::shared::calculate_path_hash;

// PKG packages have the following structure:
// - `PKG\n` signature
// - `index_size` := number of bytes in the header, including signature (?) (1x u16)
// - `entry_size` := number of bytes in each entry (1x u16)
// - `entry_count` := number of entries (1x u32)
// - `path_region_size` := total number of bytes used to store all inner paths (1x u32)
// - Entry headers (`entry_size` x `entry_count`)
//   - `inner_path_hash` := calculated hash of the entry's inner path, for error checking (1x u32)
//   - `entry_options` := bit flags used to process the entry, eg. if it is deflated (1x u8)
//   - `inner_path_offset` := offset relative to the start of inner path region (1x u24)
//   - `data_offset` := offset relative to start of file (1x u32)
//   - `data_size` := size of data to read (1x u32)
//   - `unpacked_data_size` := size of data after deflating, for error checking (1x u32)
// - path region (`path_region_size`)
//   - null-terminated ASCII strings (x `entry_count`)
// - padding for 4-byte alignment (u8/u16/u24, depending on length of path region)
// - Entries / data region (`Entry.data_size` x `entry_count`, until EOF)

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

    for expected_signature_byte in PKG_SIGNATURE {
        let signature_byte = input.read_u8()?;
        if signature_byte != expected_signature_byte {
            return PackageCorruptErrorImpl::SignatureMismatchError {
                expected: expected_signature_byte,
                actual: signature_byte
            }.into();
        }
    }

    let index_size = input.read_u16::<BigEndian>()?;
    if index_size != INDEX_SIZE {
        return PackageCorruptErrorImpl::HeaderSizeMismatchError {
            expected: INDEX_SIZE,
            actual: index_size
        }.into();
    }

    let entry_size = input.read_u16::<BigEndian>()?;
    if entry_size != ENTRY_SIZE {
        return PackageCorruptErrorImpl::EntriesHeaderSizeMismatchError {
            expected: ENTRY_SIZE,
            actual: entry_size
        }.into();
    }

    let entry_count = input.read_u32::<BigEndian>()? as usize;
    let path_region_size = input.read_u32::<BigEndian>()? as usize;

    let mut entry_builders: Vec<EntryBuilder> = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        let entry_builder = EntryBuilder::read_entry_header(&mut input)?;
        entry_builders.push(entry_builder);
    }

    let mut path_region_buffer = vec![0_u8; path_region_size as usize];
    input.read_exact(&mut path_region_buffer)?;
    let mut path_region_cursor = Cursor::new(path_region_buffer);

    for mut entry_builder in entry_builders {
        entry_builder.read_inner_path(&mut path_region_cursor)?;
        entry_builder.read_data(&mut input)?;
        let entry = entry_builder.into();
        result.add_entry_internal(entry)?;
    }

    drop(path_region_cursor);

    Ok(result)
}

struct EntryBuilder {
    inner_path_hash: u32,
    inner_path_offset: u32,
    data_offset: u32,
    data_size: u32,
    inner_path: Option<String>,
    data: Option<Vec<u8>>,
}

impl EntryBuilder {
    fn read_entry_header(input: &mut (impl Read + Seek)) -> Result<EntryBuilder, EntryReadError> {
        let inner_path_hash = input.read_u32::<BigEndian>()?;
        let entry_options = input.read_u8()?;
        let is_data_deflated = (entry_options & PKG_DEFLATED) != 0;
        let inner_path_offset = input.read_u24::<BigEndian>()?;

        let data_offset = input.read_u32::<BigEndian>()?;
        let data_size = input.read_u32::<BigEndian>()?;
        let _unpacked_size = input.read_u32::<BigEndian>()?;

        if is_data_deflated {
            return EntryReadErrorImpl::UnsupportedDeflatedEntryError().into();
        }

        Ok(EntryBuilder {
            inner_path_hash,
            inner_path_offset,
            data_offset,
            data_size,
            inner_path: Option::None,
            data: Option::None,
        })
    }

    fn read_inner_path(&mut self, path_region_input: &mut (impl Read + Seek)) -> Result<(), EntryReadError> {
        path_region_input.seek(SeekFrom::Start(self.inner_path_offset as u64))?;
        self.inner_path = Some(read_null_terminated_string(path_region_input)?);

        let inner_path = self.inner_path.as_ref().unwrap();
        let calculated_hash = calculate_path_hash(inner_path);
        if calculated_hash != self.inner_path_hash {
            return EntryReadErrorImpl::PathHashMismatchError {
                inner_path: inner_path.to_string(),
                expected: self.inner_path_hash,
                actual: calculated_hash
            }.into();
        }

        Ok(())
    }

    fn read_data(&mut self, data_input: &mut (impl Read + Seek)) -> Result<(), EntryReadError> {
        data_input.seek(SeekFrom::Start(self.data_offset as u64))?;
        let mut buffer = vec![0_u8; self.data_size as usize];
        data_input.read_exact(&mut buffer)?;
        self.data = Some(buffer);

        Ok(())
    }
}

impl Into<Entry> for EntryBuilder {
    fn into(self) -> Entry {
        Entry {
            inner_path: self.inner_path.expect("Missing inner_path!"),
            content: self.data.expect("Missing data!")
        }
    }
}

fn read_null_terminated_string(input: &mut (impl Read + Seek)) -> Result<String, EntryReadError> {
    let mut result: String = String::new();
    loop {
        let read_byte = input.read_u8()?;
        if read_byte == 0 {
            break;
        }

        result.push(read_byte as char);
    }

    Ok(result)
}
