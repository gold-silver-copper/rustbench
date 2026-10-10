use task::first_error;

#[test]
fn finds_error_after_blank_line() {
    let log = "INFO starting\n\nWARN slow disk\nERROR disk full\n";
    assert_eq!(first_error(log), Some((4, "disk full")));
}
