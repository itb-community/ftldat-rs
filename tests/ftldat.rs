#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    use ftldat::prelude::*;

    const TEST_DAT_PATH: &str = "./tests-resources/test.dat";

    //region <Modification API>
    #[test]
    fn new_package_should_be_empty() {
        let package = FtlDatPackage::new();

        assert_eq!(0, package.len());
    }

    #[test]
    fn add_entry_should_succeed_when_innerpath_is_free() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content = "test";

        // Execute
        let result = package.add_entry_from_string(
            inner_path,
            content,
        );

        // Check
        assert!(result.is_ok());
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content, package.string_content_by_path(inner_path)
            .unwrap_or("".to_string()));
    }

    #[test]
    fn add_entry_should_fail_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content1 = "test";

        let result = package.add_entry_from_string(
            inner_path,
            content1,
        );
        assert!(result.is_ok());

        // Execute
        let content2 = "test123";
        let result = package.add_entry_from_string(
            inner_path,
            content2,
        );

        // Check
        assert!(result.is_err());
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content1, package.string_content_by_path(inner_path)
            .unwrap_or("".to_string()));
    }

    #[test]
    fn put_entry_should_succeed_when_innerpath_is_free() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content = "test";

        // Execute
        package.put_entry_from_string(inner_path, content);

        // Check
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content, package.string_content_by_path(inner_path)
            .unwrap_or("".to_string()));
    }

    #[test]
    fn put_entry_should_succeed_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content1 = "test";

        package.put_entry_from_string(inner_path, content1);

        // Execute
        let content2 = "test123";
        package.put_entry_from_string(inner_path, content2);

        // Check
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content2, package.string_content_by_path(inner_path)
            .unwrap_or("".to_string()));
    }

    #[test]
    fn remove_entry_should_return_false_when_innerpath_is_free() {
        let mut package = FtlDatPackage::new();

        let result = package.remove_entry("test");

        assert_eq!(false, result);
    }

    #[test]
    fn remove_entry_should_return_true_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        package.put_entry_from_string(inner_path, "test");

        // Execute
        let result = package.remove_entry(inner_path);

        // Check
        assert_eq!(true, result);
        assert_eq!(0, package.len());
    }

    #[test]
    fn entry_exists_should_return_false_when_innerpath_is_free() {
        // Prepare
        let package = FtlDatPackage::new();

        let result = package.entry_exists("test");

        assert_eq!(false, result);
    }

    #[test]
    fn entry_exists_should_return_true_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        package.put_entry_from_string(inner_path, "test");

        // Execute
        let result = package.entry_exists(inner_path);

        // Check
        assert_eq!(true, result);
        assert_eq!(1, package.len());
    }

    #[test]
    fn content_by_path_should_return_content_when_exists() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        package.put_entry_from_string(inner_path, "test");

        // Execute
        let result: Option<String> = package.string_content_by_path(inner_path);

        // Check
        assert!(result.is_some());
        let content = result.unwrap();
        assert_eq!("test", content);
    }

    #[test]
    fn content_by_path_should_return_none_when_doesnt_exist() {
        // Prepare
        let package = FtlDatPackage::new();

        let result: Option<String> = package.string_content_by_path("test");

        assert!(result.is_none());
    }

    #[test]
    fn clear_should_remove_all_entries_from_package() {
        // Prepare
        let mut package = FtlDatPackage::new();
        package.put_entry_from_string("test1", "test");
        package.put_entry_from_string("test2", "test");
        package.put_entry_from_string("test3", "test");
        assert_eq!(3, package.len());

        // Execute
        package.clear();

        // Check
        assert_eq!(0, package.len());
    }
    //endregion

    //region <I/O API>
    #[test]
    fn from_reader_should_correctly_read_ftldat() {
        // Execute
        let result = FtlDatPackage::from_file(TEST_DAT_PATH);

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
    fn write_should_create_file_on_disk_if_missing() {
        // Prepare
        let mut package = FtlDatPackage::new();
        package.put_entry_from_string("test", "test123");

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        // Execute
        let result = package.to_file(tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(tmp_file.path().exists());
        assert_eq!(27, tmp_file.as_file().metadata().unwrap().len());
    }

    #[test]
    fn write_should_update_file_on_disk_if_exists() {
        // Prepare
        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        std::fs::copy(Path::new(TEST_DAT_PATH), tmp_file.path())
            .expect("failed to copy test.dat for testing");

        let mut package = FtlDatPackage::new();
        package.put_entry_from_string("test", "test123");

        // Execute
        let result = package.to_file(tmp_path);

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

        let package = FtlDatPackage::from_file(TEST_DAT_PATH).unwrap();
        let order_before_write = package.inner_paths();

        // Execute
        let result = package.to_file(tmp_path);
        assert!(result.is_ok());
        let package = FtlDatPackage::from_file(tmp_path).unwrap();
        let order_after_write = package.inner_paths();

        // Check
        assert_eq!(order_before_write.len(), order_after_write.len());
        assert_eq!(order_before_write[0], order_after_write[0]);
        assert_eq!(order_before_write[1], order_after_write[1]);
        assert_eq!(order_before_write[2], order_after_write[2]);
    }
    //endregion

    #[test]
    fn should_extract_all_contents() {
        // Prepare
        let tmp_file = tempdir().unwrap();
        let tmp_path = tmp_file.path().to_str().unwrap();

        let package = FtlDatPackage::from_file(TEST_DAT_PATH).unwrap();

        // Execute
        let result = package.extract(tmp_path);

        // Check
        assert!(result.is_ok());
        assert!(PathBuf::from(tmp_path).join("test1.txt").exists());
        assert!(PathBuf::from(tmp_path).join("test2.txt").exists());
        assert!(PathBuf::from(tmp_path).join("test3.txt").exists());
    }
}
