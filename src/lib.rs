/// It takes a string, a pattern, and a mutable vector of bytes, and it appends the first line of the
/// string that contains the pattern to the vector
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

/// It takes a string slice, a string slice, and a mutable reference to a type that implements the
/// `std::io::Write` trait, and it writes all lines from the first string slice that contain the second
/// string slice to the third argument
///
/// Arguments:
///
/// * `content`: &str - The content to search through.
/// * `pattern`: &str
/// * `writer`: impl std::io::Write
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
