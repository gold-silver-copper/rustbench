use task::{count_level, first_error};

#[test]
fn skips_comments() {
    let log = "# rotated at midnight\nINFO up\n# note\nERROR out of memory\nERROR again";
    assert_eq!(first_error(log), Some((4, "out of memory")));
}

#[test]
fn no_error() {
    assert_eq!(first_error("INFO a\n\nWARN b\n"), None);
    assert_eq!(first_error(""), None);
}

#[test]
fn first_line() {
    assert_eq!(first_error("ERROR boom\n"), Some((1, "boom")));
    assert_eq!(count_level("ERROR a\nINFO b\nERROR c", "ERROR"), 2);
}
