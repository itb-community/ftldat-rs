use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::entry::{Entry};
use crate::error::{ReadEntryError, ReadPackageError};
use crate::package::Package;

pub(crate) struct DatReader {}

impl DatReader {
    /// Within the FtlDat file on disk, this package is laid out as follows:
    /// - `index_size` := size of the index (1x u32)
    /// - offsets to [Entries](Entry) (`index_size` x u32)
    /// - [Entries](Entry) (`index_size` x [Entry])
    pub(crate) fn read_package(mut input: (impl Read + Seek)) -> Result<Package, ReadPackageError> {
        let mut result = Package::new();
        input.seek(SeekFrom::Start(0))?;

        let index_size = input.read_u32::<LittleEndian>()? as usize;

        // TODO: Skip offsets and simply read entries until EOF?
        let mut entry_offsets = Vec::with_capacity(index_size);
        for _ in 0..index_size {
            let entry_offset = input.read_u32::<LittleEndian>()?;
            entry_offsets.push(entry_offset);
        }

        for entry_offset in entry_offsets {
            input.seek(SeekFrom::Start(entry_offset as u64))?;

            let entry = DatReader::read_entry(&mut input)?;

            result.add_entry_internal(entry)
                .map_err(ReadPackageError::ProcessPackageError)?;
        }

        Ok(result)
    }

    /// Within the FtlDat file on disk, these entries are laid out as follows:
    /// - `data_size` := file content length (1x u32)
    /// - `str_len` := file name length (1x u32)
    /// - file name (`str_len` x u8)
    /// - file content (`data_size` x u8)
    pub(crate) fn read_entry(input: &mut (impl Read + Seek)) -> Result<Entry, ReadEntryError> {
        let content_length = input.read_u32::<LittleEndian>()?;
        let inner_path_length = input.read_u32::<LittleEndian>()?;

        let mut buffer = vec![0_u8; inner_path_length as usize];
        input.read_exact(&mut buffer)?;
        let inner_path = String::from_utf8(buffer)?;

        let mut buffer = vec![0_u8; content_length as usize];
        input.read_exact(&mut buffer)?;

        Ok(Entry::from_byte_array(inner_path, buffer))
    }
}