
pub(super) fn calculate_path_hash<S: AsRef<str>>(inner_path: S) -> u32 {
    let mut hash: u32 = 0;

    let inner_path = inner_path.as_ref();
    for lowercase_char in inner_path.to_lowercase().chars() {
        let byte = lowercase_char as u32;
        hash = hash.rotate_right(5);
        hash = hash ^ byte;
    }

    hash
}

#[cfg(test)]
mod tests {
    use crate::pkg::shared::calculate_path_hash;

    #[test]
    fn calculated_path_hash_is_correct_for_sample_1() {
        let result = calculate_path_hash("test1.txt");
        assert_eq!(157232055, result);
    }

    #[test]
    fn calculated_path_hash_is_correct_for_sample_2() {
        let result = calculate_path_hash("audio/music/bp_MUS_CivilBATTLE.ogg");
        assert_eq!(1735861093, result);
    }
}
