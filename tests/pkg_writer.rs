#[cfg(test)]
mod test_pkg_writer {
    use std::path::Path;

    use ftldat::{Package, PackageEntry};

    const SOURCE_PATH: &str = "./tests-resources/test.pkg";

    #[test]
    fn writer_should_create_file_on_disk_if_missing() {
        // Prepare
        let mut package = Package::new();
        package.put_entry(PackageEntry::from_string("test", "test123"));

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        // Execute
        let result = package.to_path_pkg(tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(tmp_file.path().exists());
        assert_eq!(51, tmp_file.as_file().metadata().unwrap().len());
    }

    #[test]
    fn writer_should_update_file_on_disk_if_exists() {
        // Prepare
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        std::fs::copy(Path::new(SOURCE_PATH), tmp_file.path())
            .expect("failed to copy test.dat for testing");

        let mut package = Package::new();
        package.put_entry(PackageEntry::from_string("test", "test123"));

        // Execute
        let result = package.to_path_pkg(tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(tmp_file.path().exists());
        assert_eq!(51, tmp_file.as_file().metadata().unwrap().len());
    }

    #[test]
    fn entry_order_should_be_retained_between_writes() {
        // Prepare
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        let package = Package::from_path_pkg(SOURCE_PATH).unwrap();
        let order_before_write = package.inner_paths();

        // Execute
        let result = package.into_path_pkg(tmp_path);

        assert!(result.is_ok());
        let package = Package::from_path_pkg(tmp_path).unwrap();
        let order_after_write = package.inner_paths();

        // Check
        assert_eq!(order_before_write.len(), order_after_write.len());
        assert_eq!(order_before_write[0], order_after_write[0]);
        assert_eq!(order_before_write[1], order_after_write[1]);
        assert_eq!(order_before_write[2], order_after_write[2]);
    }
}