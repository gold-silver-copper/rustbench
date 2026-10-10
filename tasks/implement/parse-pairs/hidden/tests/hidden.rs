use std::collections::BTreeMap;
use task::{parse_pairs, PairError};

fn map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn parses() {
    assert_eq!(parse_pairs("host=localhost;port=8080"), Ok(map(&[("host", "localhost"), ("port", "8080")])));
    assert_eq!(parse_pairs("  k  =  two words  ; e= ;"), Ok(map(&[("k", "two words"), ("e", "")])));
    assert_eq!(parse_pairs("eq===;"), Ok(map(&[("eq", "==")])));
    assert_eq!(parse_pairs(";; \t;a=1;;"), Ok(map(&[("a", "1")])));
}

#[test]
fn empty() {
    assert_eq!(parse_pairs(""), Ok(BTreeMap::new()));
    assert_eq!(parse_pairs(" ; ;"), Ok(BTreeMap::new()));
}

#[test]
fn errors() {
    assert_eq!(parse_pairs(";;x"), Err(PairError::MissingEquals(2)));
    assert_eq!(parse_pairs("a=1; =2"), Err(PairError::EmptyKey(1)));
    assert_eq!(parse_pairs("=;b"), Err(PairError::EmptyKey(0)));
    assert_eq!(parse_pairs("a=1;b=2; a =3"), Err(PairError::DuplicateKey("a".to_string())));
    assert_eq!(parse_pairs("a=1;a=1;zz"), Err(PairError::DuplicateKey("a".to_string())));
}
