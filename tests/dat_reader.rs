#[cfg(test)]
mod test_dat_reader {
    use ftldat::prelude::dat_package;

    const TEST_DAT_PATH: &str = "./tests-resources/test.dat";

    #[test]
    fn reader_should_correctly_read_package() {
        // Execute
        let result = dat_package::read_from_path(TEST_DAT_PATH);

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
}
