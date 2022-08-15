use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use mlua::prelude::LuaUserData;

use crate::error::*;

//region <FtlDat>
pub struct FtlDat {
    path: String,
    file: File,
    package: Package,
}

impl FtlDat {
    pub fn create_new(path: &str) -> Result<FtlDat> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open or create the file '{}'", path));

        let package = Package::new();

        Ok(FtlDat {
            path: path.to_string(),
            file,
            package,
        })
    }

    pub fn open_existing(path: &str) -> Result<FtlDat> {
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open the file '{}'", path));

        let package = Package::from_reader(BufReader::new(&file))?;

        Ok(FtlDat {
            path: path.to_string(),
            file,
            package,
        })
    }

    pub fn write(&self) -> Result<()> {
        self.package.to_file(BufWriter::new(&self.file))
            .unwrap_or_else(|_| panic!("Failed to write FtlDat '{}'", self.path));

        Ok(())
    }

    pub fn write_to(&self, path: &str) -> Result<()> {
        let file = File::options()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open file '{}'", path));

        self.package.to_file(BufWriter::new(file))
            .unwrap_or_else(|_| panic!("Failed to write FtlDat '{}'", self.path));

        Ok(())
    }

    pub fn entry_count(&self) -> usize {
        self.package.entries.len()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn read_data_by_path(&mut self, path: &str) -> Result<Vec<u8>> {
        let mut input = BufReader::new(&self.file);
        self.package.entry_by_path(path).read_data(&mut input)
    }
}

impl Display for FtlDat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FtlDat [ path = '{}', entries: {} ]", self.path, self.entry_count())
    }
}

impl LuaUserData for FtlDat {}
//endregion

//region <Package>
/// The original format FTL used to store resources.
///
/// Structure:
///   Index = A count, followed by a series of offsets to entries.
///   Entries = A series of { data_size, inner_path, data } hunks.
struct Package {
    path_to_index: HashMap<String, usize>,
    entries: Vec<Entry>,
}

impl Package {
    fn new() -> Package {
        Package {
            entries: vec![],
            path_to_index: HashMap::new(),
        }
    }

    fn from_reader(mut input: (impl Read + Seek)) -> Result<Package> {
        input.seek(SeekFrom::Start(0))
            .expect("Failed to navigate to start of file");

        let index_size = input.read_u32::<LittleEndian>()
            .expect("Failed to read index_size") as usize;

        let mut entries = Vec::with_capacity(index_size);
        // Store partial information about entries, since we need to complete traversing
        // the header first before we can read the entries' contents.
        for index in 0..index_size {
            let entry_offset = input.read_u32::<LittleEndian>()
                .expect("Failed to read entry_offset");

            if entry_offset != 0 {
                entries.push(Entry::new(entry_offset));
            } else {
                panic!(
                    "Corrupted FtlDat: entry #{} claims to have offset of 0",
                    index
                )
            }
        };

        let mut path_to_index = HashMap::new();
        for (index, entry) in entries.iter_mut().enumerate() {
            entry.init(&mut input)?;

            if path_to_index.contains_key(&entry.inner_path) {
                return Err(Error::InnerPathAlreadyExists { path: entry.inner_path.clone() });
            }

            path_to_index.insert(entry.inner_path.clone(), index);
        }

        Ok(Package {
            entries,
            path_to_index,
        })
    }

    fn to_file(&self, mut output: (impl Write + Seek)) -> Result<()> {
        // Write the header
        // Index size
        let index_size = self.entries.len() as u32;
        output.seek(SeekFrom::Start(0))
            .expect("Failed to navigate to start of file");
        output.write_u32::<LittleEndian>(index_size)?;

        for index in 0..index_size {
            let entry = self.entries.get(index as usize)
                .unwrap_or_else(|| panic!("Failed to retrieve entry at index {}", index));
            output.write_u32::<LittleEndian>(entry.entry_offset)?;
        }

        for index in 0..index_size {
            let entry = self.entries.get(index as usize)
                .unwrap_or_else(|| panic!("Failed to retrieve entry at index {}", index));

            entry.write(&mut output)?;
        }

        Ok(())
    }

    fn entry_by_path(&self, path: &str) -> &Entry {
        let index = self.path_to_index.get(path)
            .unwrap_or_else(|| panic!("Path not known: '{}'", path));
        self.entries.get(*index)
            .unwrap_or_else(|| panic!("Failed to retrieve entry at index {}", index))
    }

    fn add_entry(&mut self, entry: Entry) -> Result<()> {
        if self.path_to_index.contains_key(&entry.inner_path) {
            return Err(Error::InnerPathAlreadyExists { path: entry.inner_path.clone() });
        }

        let index = self.entries.len();
        self.path_to_index.insert(entry.inner_path.clone(), index);
        self.entries.push(entry);

        Ok(())
    }
}
//endregion

//region <Entry>
/// Information about an innerFile within a .dat.
///
/// entry_offset = Offset (written in header) to the data_size + inner_path + data
/// inner_path   = A virtual location ("dir/dir/filename")
/// data_size    = Size of the innerFile.
///
/// data_offset  = Offset to the innerFile.
struct Entry {
    entry_offset: u32,
    data_size: u32,
    inner_path: String,
    data: Vec<u8>,

    data_offset: u32,
}

impl Default for Entry {
    fn default() -> Self {
        Entry::new(0)
    }
}

impl Entry {
    fn new(entry_offset: u32) -> Entry {
        Entry {
            entry_offset,
            data_size: 0,
            inner_path: String::default(),
            data_offset: 0,
            data: Vec::default(),
        }
    }

    fn init(&mut self, input: &mut (impl Read + Seek)) -> Result<()> {
        input.seek(SeekFrom::Start(self.entry_offset as u64))
            .expect("Failed to navigate to entry");

        self.data_size = input.read_u32::<LittleEndian>()
            .expect("Failed to read data size");
        self.inner_path = read_string(input)
            .expect("Failed to read string");
        self.data_offset = input.stream_position().unwrap() as u32;
        self.data = self.read_data(input)?;

        Ok(())
    }

    fn read_data(&self, input: &mut (impl Read + Seek)) -> Result<Vec<u8>> {
        input.seek(SeekFrom::Start(self.data_offset as u64))
            .expect("Failed to navigate eto entry");

        let mut buffer = vec![0_u8; self.data_size as usize];
        input.read_exact(&mut buffer)
            .expect("Failed to read entry data");

        Ok(buffer)
    }

    fn write(&self, mut output: impl Write) -> Result<()> {
        output.write_u32::<LittleEndian>(self.data_size)?;
        write_string(&mut output, &self.inner_path)?;
        output.write_all(&self.data)?;

        Ok(())
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "DatEntry ['{}', offset: {}, data_offset: {}, size: {}]",
            self.inner_path, self.entry_offset, self.data_offset, self.data_size
        )
    }
}
//endregion

/// FtlDat uses length-prefixed strings
fn read_string(input: &mut impl Read) -> Result<String> {
    let str_len = input.read_u32::<LittleEndian>()
        .expect("Failed to read string size") as usize;
    let mut buffer = vec![0_u8; str_len];
    input.read_exact(&mut buffer)
        .expect("Failed to read string");
    let result = String::from_utf8(buffer)
        .expect("Failed to parse string");

    Ok(result)
}

/// FtlDat uses length-prefixed strings
fn write_string(output: &mut impl Write, str: &str) -> Result<()> {
    output.write_u32::<LittleEndian>(str.len() as u32)
        .expect("Failed to write string size");

    output.write_all(str.as_bytes())
        .expect("Failed to write string");

    Ok(())
}