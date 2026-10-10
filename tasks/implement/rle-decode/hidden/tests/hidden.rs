use task::rle_decode;

#[test]
fn decodes() {
    assert_eq!(rle_decode(""), Ok(String::new()));
    assert_eq!(rle_decode("abc"), Ok("abc".to_string()));
    assert_eq!(rle_decode("x1y12"), Ok(format!("x{}", "y".repeat(12))));
    assert_eq!(rle_decode("é2 3!"), Ok("éé   !".to_string()));
    assert_eq!(rle_decode("z65535").map(|s| s.len()), Ok(65535));
}

#[test]
fn errors() {
    assert_eq!(rle_decode("3a"), Err(0));
    assert_eq!(rle_decode("ab0"), Err(2));
    assert_eq!(rle_decode("a07"), Err(1));
    assert_eq!(rle_decode("é0"), Err(2));
    assert_eq!(rle_decode("a2b65536"), Err(3));
    assert_eq!(rle_decode("a99999999999999999999999"), Err(1));
}
