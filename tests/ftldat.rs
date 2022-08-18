#[cfg(test)]
mod tests {
    use ftldat::prelude::*;

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
        let result = package.add_entry(FtlDatEntry::from(inner_path, content));

        // Check
        assert!(result.is_ok());
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content, package.entry_by_path(inner_path)
            .map(|e| e.content_string())
            .unwrap_or("".to_string()));
    }

    #[test]
    fn add_entry_should_fail_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content1 = "test";

        let result = package.add_entry(FtlDatEntry::from(inner_path, content1));
        assert!(result.is_ok());

        // Execute
        let content2 = "test123";
        let result = package.add_entry(FtlDatEntry::from(inner_path, content2));

        // Check
        assert!(result.is_err());
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content1, package.entry_by_path(inner_path)
            .map(|e| e.content_string())
            .unwrap_or("".to_string()));
    }

    #[test]
    fn put_entry_should_succeed_when_innerpath_is_free() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content = "test";

        // Execute
        package.put_entry(FtlDatEntry::from(inner_path, content));

        // Check
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content, package.entry_by_path(inner_path)
            .map(|e| e.content_string())
            .unwrap_or("".to_string()));
    }

    #[test]
    fn put_entry_should_succeed_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        let content1 = "test";

        package.put_entry(FtlDatEntry::from(inner_path, content1));

        // Execute
        let content2 = "test123";
        package.put_entry(FtlDatEntry::from(inner_path, content2));

        // Check
        assert_eq!(1, package.len());
        assert!(package.entry_exists(inner_path));
        assert_eq!(content2, package.entry_by_path(inner_path)
            .map(|e| e.content_string())
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
        package.put_entry(FtlDatEntry::from(inner_path, "test"));

        // Execute
        let result = package.remove_entry(inner_path);

        // Check
        assert_eq!(true, result);
        assert_eq!(0, package.len());
    }

    #[test]
    fn entry_exists_should_return_false_when_innerpath_is_free() {
        // Prepare
        let mut package = FtlDatPackage::new();

        let result = package.entry_exists("test");

        assert_eq!(false, result);
    }

    #[test]
    fn entry_exists_should_return_true_when_innerpath_is_taken() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        package.put_entry(FtlDatEntry::from(inner_path, "test"));

        // Execute
        let result = package.entry_exists(inner_path);

        // Check
        assert_eq!(true, result);
        assert_eq!(1, package.len());
    }

    #[test]
    fn entry_by_path_should_return_reference_to_entry_when_exists() {
        // Prepare
        let mut package = FtlDatPackage::new();
        let inner_path = "test";
        package.put_entry(FtlDatEntry::from(inner_path, "test"));

        // Execute
        let result = package.entry_by_path(inner_path);

        // Check
        assert!(result.is_some());
        let entry = result.unwrap();
        assert_eq!(inner_path, entry.inner_path());
        assert_eq!("test", entry.content_string());
    }

    #[test]
    fn entry_by_path_should_return_none_when_doesnt_exist() {
        // Prepare
        let package = FtlDatPackage::new();

        let result = package.entry_by_path("test");

        assert!(result.is_none());
    }

    #[test]
    fn clear_should_remove_all_entries_from_package() {
        // Prepare
        let mut package = FtlDatPackage::new();
        package.put_entry(FtlDatEntry::from("test1", "test"));
        package.put_entry(FtlDatEntry::from("test2", "test"));
        package.put_entry(FtlDatEntry::from("test3", "test"));
        assert_eq!(3, package.len());

        // Execute
        package.clear();

        // Check
        assert_eq!(0, package.len());
    }
    //endregion
}
