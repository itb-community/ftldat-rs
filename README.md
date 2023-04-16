# FTLDat-rs

Rust implementation of FTLDat - a simple library for unpacking and repacking of .dat files, which are used
by the games [Into the Breach](https://subsetgames.com/itb.html) and [Faster than Light](https://subsetgames.com/ftl.html) (until version 1.6.1).

This library also supports the PKG format used by FTL after version 1.6.1.

# Usage

Opening a package is fairly straightforward:

```rs
use ftldat::Package;

let package = Package::from_path_dat("path/to/file.dat");

# Can now query the package's contents, list or iterate...
println!("Number of entries: {}", &package.entry_count());
println!("Does the package contain a file at path 'test.txt'? {}", &package.entry_exists("test.txt"));

# List paths of all files within the package
let inner_paths = package.inner_paths();

for entry in package.iter() {
    # Do something with each entry
}
```

The underlying file is memory-mapped, and only read when initially creating the `package` instance, or when fetching
an entry's content.

Packages can be modified to add, replace, or remove entries:
```rs
use ftldat::{Package, PackageEntry};

let mut package = Package::from_path_dat("path/to/file.dat");

# `add_entry` will only add the entry if the package does NOT already contain an entry at the specified path (test.txt).
# Otherwise, an error is returned.
package.add_entry(PackageEntry::from_string("test.txt", "My text file's content."));

# `put_entry` will overwrite the entry at the specified path (test2.txt) with the provided entry.
package.put_entry(PackageEntry::from_string("test2.txt", "Lorem ipsum dolor sit amet"));

# Remove individual entry
package.remove_entry("test.txt");

# Remove all entries
package.clear();
```

Entries can be created in a few ways:
```rs
# Directly from a string, mostly useful for testing (functionally the same as in-memory byte array)
let entry = PackageEntry::from_string("file.txt", "Lorem ipsum dolor sit amet");

# From a file on disk
let entry = PackageEntry::from_file("file.png", "path/to/file.png");

# From an in-memory byte array
let content = [0, 1, 2, 3];
PackageEntry::from_byte_array("file.bin", content.into());

# From a memory-mapped file
let mmap = ...     # Reference to the memory map
let mmap_rc = Rc::new(mmap);
let offset = ...   # Offset to the beginning of the entry's content within the memory-mapped file
let length = ...   # Number of bytes that make up the entry's content
let entry = PackageEntry::from_memory_mapped_file(
    "file.wav",
    mmap_rc.clone(),
    offset,
    length
);
```

Packages can be written back to a file:
```rs
use ftldat::Package;

let package = Package::from_path_dat("path/to/file.dat");

# `write_to_path_*` does not consume the `package`, allowing for multiple writes, but only allows writing to
# files other than the file from which the package was originally loaded.
package.to_path_dat("path/to/other/file.dat");

# `write_into_path_*` consumes the `package`, but releases file system resources and allows overwriting the
# file from which the package was originally loaded.
package.write_into_path_dat(package, "path/to/file.dat");
```

Contents of the package can also be extracted:
```rs
use ftldat::Package;

let package = Package::from_path_dat("path/to/file.dat");
package.extract("destination/directory/");
```

# Areas to Improve

Considering that this project served me as a way to familiarize myself with Rust, there's bound to be a lot of room for
improvement. In no particular order:
- Error handling. Tried to use `thiserror`, a popular crate for error-handling, but I don't feel particularly confident about it.
- Ownership of strings, I just used heap-allocated Strings and copied them left and right
- Naming of functions, following proper Rust conventions (`from`, `into`, etc.)
- Serialization of structs to bytes can probably be handled better (though I like keeping in-memory and on-disk representations separate)
