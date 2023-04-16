#[cfg(test)]
mod test_pkg_reader {
    use ftldat::Package;

    const SOURCE_PATH: &str = "./tests-resources/test.pkg";

    #[test]
    fn reader_correctly_reads_test_package() {
        // Execute
        let result = Package::from_path_pkg(SOURCE_PATH);
        if result.is_err() {
            panic!("{:?}", result.unwrap_err());
        }

        // Check
        let package = result.unwrap();
        assert_eq!(3, package.entry_count());

        let paths = package.inner_paths();
        assert_eq!("test1.txt", paths[0]);
        assert_eq!("test2.txt", paths[1]);
        assert_eq!("test3.txt", paths[2]);

        let contents = paths.iter()
            .map(|path| {
                let vec = package.content_by_path(path).unwrap();
                String::from_utf8(vec).expect("Invalid UTF-8 sequence")
            })
            .collect::<Vec<String>>();
        assert_eq!("test001", contents[0]);
        assert_eq!("test002", contents[1]);
        assert_eq!("test003", contents[2]);
    }
}
