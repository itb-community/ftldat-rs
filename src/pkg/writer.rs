use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

use byteorder::{BigEndian, WriteBytesExt};
use crate::{Package, PackageEntry};

use crate::pkg::constants::{ENTRY_SIZE, INDEX_SIZE, PKG_SIGNATURE};
use crate::pkg::error::PkgWriteError;
use crate::pkg::shared::calculate_path_hash;
use crate::shared::error::PackageWriteError;

/// Writes out the specified [Package] in binary PKG format to a file at the specified `target_path`.
///
/// This method does not consume the [Package], so it is possible to write the same instance to many
/// different files.
/// However, the [Package] holds a reference to the file from which it has been created, therefore
/// it is impossible to overwrite the source file with this method. If this is what you want to do,
/// use [write_package_into_path] instead.
pub fn write_package_to_path<P: AsRef<Path>>(package: &Package, target_path: P) -> Result<(), PackageWriteError> {
    let target_path = target_path.as_ref();

    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&target_path)
        .expect(&format!("Failed to open file '{:?}' for writing", &target_path));

    write_package_to_output(package, BufWriter::new(file))
        .expect(&format!(
            "Failed to write package to output file '{:?}'. If you're trying to overwrite \
            the same file from which you've read the package, use `write_package_into_path` instead.",
            &target_path
        ));

    Ok(())
}

/// Writes out the specified [Package] in binary PKG format to a file at the specified `target_path`.
///
/// This method consumes the [Package], so after this method completes, it will not be possible to
/// write the [Package]'s contents to any other files. However, since consuming the [Package] closes
/// the underlying file, this method can overwrite the source file from which the [Package] was
/// originally created.
pub fn write_package_into_path<P: AsRef<Path>>(package: Package, target_path: P) -> Result<(), PackageWriteError> {
    let target_path = target_path.as_ref();
    let target_path_tmp = target_path.with_extension("tmp");

    let file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&target_path_tmp)
        .expect(&format!("Failed to open file '{:?}' for writing", &target_path_tmp));

    write_package_to_output(&package, BufWriter::new(file))
        .expect(&format!("Failed to write package to output file '{:?}'", &target_path_tmp));

    drop(package);

    std::fs::remove_file(&target_path)
        .expect(&format!("Failed to delete file at '{:?}'", &target_path));
    std::fs::rename(&target_path_tmp, &target_path)
        .expect(&format!("Failed to move file from '{:?}' to '{:?}' after writing", &target_path_tmp, target_path));

    Ok(())
}

/// Writes out the specified [Package] in binary PKG format to the given `output`,
/// consuming it in the process.
pub fn write_package_to_output(package: &Package, mut output: (impl Write + Seek)) -> Result<(), PackageWriteError> {
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
    for entry in package.iter() {
        let mut entry_header = EntryHeader::from(entry);
        entry_header.inner_path_offset = path_region_buffer.len() as u32;
        entry_header.data_offset = data_offset;
        data_offset += entry.content()?.len() as u32;

        path_region_buffer.extend_from_slice(entry.inner_path().as_bytes());
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
    for entry in package.iter() {
        output.write_all(&entry.content()?)?;
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

impl From<&PackageEntry> for EntryHeader {
    fn from(entry: &PackageEntry) -> Self {
        let content = entry.content().expect("Failed to read content of entry");
        EntryHeader {
            inner_path_hash: calculate_path_hash(&entry.inner_path()),
            // We do not support deflated entries, so always write out 0 for entry options.
            entry_options: 0,
            inner_path_offset: 0,
            data_offset: 0,
            data_size: content.len() as u32,
            unpacked_data_size: content.len() as u32,
        }
    }
}