#[cfg(test)]
mod test_dat_reader {
    use std::path::Path;

    use ftldat::prelude::{DatPackageReader, DatPackageWriter, Package};
    use ftldat_rs::prelude::{DatPackageReader, DatPackageWriter, Package};

    const TEST_DAT_PATH: &str = "./tests-resources/test.dat";

    #[test]
    fn reader_should_correctly_read_package() {
        // Execute
        let result = DatPackageReader::read_package_from_path(TEST_DAT_PATH);

        // Check
        assert!(result.is_ok());
        let package = result.unwrap();
        assert_eq!(3, package.len());

        let paths = package.inner_paths();
        assert_eq!("test1.txt", paths[0]);
        assert_eq!("test2.txt", paths[1]);
        assert_eq!("test3.txt", paths[2]);

        let contents = paths.iter()
            .map(|path| package.string_content_by_path(path).unwrap())
            .collect::<Vec<String>>();
        assert_eq!("test001", contents[0]);
        assert_eq!("test002", contents[1]);
        assert_eq!("test003", contents[2]);
    }

    #[test]
    fn writer_should_create_file_on_disk_if_missing() {
        // Prepare
        let mut package = Package::new();
        package.put_entry_from_string("test", "test123");

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        // Execute
        let result = DatPackageWriter::write_package_to_path(package, tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(tmp_file.path().exists());
        assert_eq!(27, tmp_file.as_file().metadata().unwrap().len());
    }

    #[test]
    fn writer_should_update_file_on_disk_if_exists() {
        // Prepare
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        std::fs::copy(Path::new(TEST_DAT_PATH), tmp_file.path())
            .expect("failed to copy test.dat for testing");

        let mut package = Package::new();
        package.put_entry_from_string("test", "test123");

        // Execute
        let result = DatPackageWriter::write_package_to_path(package, tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(tmp_file.path().exists());
        assert_eq!(27, tmp_file.as_file().metadata().unwrap().len());
    }

    #[test]
    fn entry_order_should_be_retained_between_writes() {
        // Prepare
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        let package = DatPackageReader::read_package_from_path(TEST_DAT_PATH).unwrap();
        let order_before_write = package.inner_paths();

        // Execute
        let result = DatPackageWriter::write_package_to_path(package, tmp_path);
        assert!(result.is_ok());
        let package = DatPackageReader::read_package_from_path(tmp_path).unwrap();
        let order_after_write = package.inner_paths();

        // Check
        assert_eq!(order_before_write.len(), order_after_write.len());
        assert_eq!(order_before_write[0], order_after_write[0]);
        assert_eq!(order_before_write[1], order_after_write[1]);
        assert_eq!(order_before_write[2], order_after_write[2]);
    }
}
