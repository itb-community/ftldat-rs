use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::rc::Rc;

use byteorder::{LittleEndian, ReadBytesExt};
use memmap2::Mmap;
use crate::dat::constants::INDEX_SIZE;
use crate::shared::entry::PackageEntry;

use crate::shared::error::PackageReadError;
use crate::shared::package::{Package};

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
    read_from_input(file)
}

/// Constructs a [Package] instance from data in the given `input',
/// consuming it in the process.
pub fn read_from_input(file: File) -> Result<Package, PackageReadError> {
    let mut result = Package::new();

    let mmap = unsafe {
        Mmap::map(&file)
    }?;

    let mut cursor = Cursor::new(&mmap[..INDEX_SIZE]);
    let entry_count = cursor.read_u32::<LittleEndian>()? as usize;

    // TODO: Skip offsets and simply read entries until EOF?
    let entry_area_offset = INDEX_SIZE + entry_count * 4;
    let mut cursor = Cursor::new(&mmap[INDEX_SIZE..entry_area_offset]);
    let mut entry_offsets = Vec::with_capacity(entry_count);
    for _ in 0..entry_count {
        let entry_offset = cursor.read_u32::<LittleEndian>()?;
        entry_offsets.push(entry_offset);
    }

    let entry_builders: Vec<EntryBuilder> = entry_offsets.iter()
        .map(|entry_offset| {
            EntryBuilder::read_entry(&mmap, *entry_offset as usize)
                .expect("Failed to read entry")
        })
        .collect();

    let mmap_rc = Rc::new(mmap);
    for entry_builder in entry_builders {
        let entry = entry_builder.build(mmap_rc.clone());
        result.add_entry(entry)?;
    }

    Ok(result)
}

struct EntryBuilder {
    inner_path: String,
    data_offset: usize,
    data_size: usize
}

impl EntryBuilder {
    fn read_entry(mmap: &Mmap, entry_offset: usize) -> Result<EntryBuilder, PackageReadError> {
        let entry_variable_area_offset = entry_offset + 8;
        let mut cursor = Cursor::new(&mmap[entry_offset..entry_variable_area_offset]);

        let entry_content_length = cursor.read_u32::<LittleEndian>()? as usize;
        let inner_path_length = cursor.read_u32::<LittleEndian>()? as usize;

        let entry_end = entry_variable_area_offset + inner_path_length + entry_content_length;

        let mut cursor = Cursor::new(&mmap[entry_variable_area_offset..entry_end]);
        let inner_path = {
            let mut buffer = vec![0u8; inner_path_length];
            cursor.read_exact(&mut buffer)?;
            String::from_utf8(buffer)?
        };

        let entry_content_offset = entry_variable_area_offset + inner_path_length;

        Ok(EntryBuilder {
            inner_path,
            data_offset: entry_content_offset,
            data_size: entry_content_length
        })
    }

    fn build(self, input: Rc<Mmap>) -> PackageEntry {
        PackageEntry::from_memory_mapped_file(
            self.inner_path,
            input,
            self.data_offset as u64,
            self.data_size as u64
        )
    }
}
