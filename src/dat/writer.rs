use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

use byteorder::{LittleEndian, WriteBytesExt};

use crate::shared::entry::PackageEntry;
use crate::shared::error::PackageWriteError;
use crate::shared::package::Package;

/// Writes out the specified [Package] in binary FtlDat format to a file at the specified `target_path`.
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

/// Writes out the specified [Package] in binary FtlDat format to a file at the specified `target_path`.
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

/// Writes out the specified [Package] in binary FtlDat format to the given `output`,
/// consuming it in the process.
pub fn write_package_to_output(package: &Package, mut output: (impl Write + Seek)) -> Result<(), PackageWriteError> {
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
