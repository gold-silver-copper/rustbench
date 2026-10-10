use std::collections::BTreeMap;

/// Why [`parse_pairs`] rejected its input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairError {
    /// The segment with this index is not blank but contains no `=`.
    MissingEquals(usize),
    /// The segment with this index has an empty key (after trimming).
    EmptyKey(usize),
    /// This key (trimmed) appears in more than one segment.
    DuplicateKey(String),
}

/// Parses `key=value` pairs separated by `;` into a map.
///
/// - The input is split at every `;` into segments, numbered from 0. Blank
///   segments still get a number.
/// - A segment that is empty or only whitespace is skipped, so `"a=1;;b=2;"`
///   is fine.
/// - Any other segment is split at its first `=`: the key is the part before
///   it, the value the part after it, both trimmed of surrounding
///   whitespace. The value may be empty and may contain more `=`.
/// - A segment with no `=` gives `MissingEquals(index)`, an empty key gives
///   `EmptyKey(index)`, and a key already seen in an earlier segment gives
///   `DuplicateKey(key)`. The first error, in segment order, is returned.
///
/// An empty input gives an empty map.
///
/// ```
/// let m = task::parse_pairs(" a = 1 ;b=x=y;").unwrap();
/// assert_eq!(m["a"], "1");
/// assert_eq!(m["b"], "x=y");
/// assert_eq!(task::parse_pairs("a=1;oops"), Err(task::PairError::MissingEquals(1)));
/// ```
pub fn parse_pairs(input: &str) -> Result<BTreeMap<String, String>, PairError> {
    let mut map = BTreeMap::new();
    for (i, seg) in input.split(';').enumerate() {
        if seg.trim().is_empty() {
            continue;
        }
        let (key, value) = seg.split_once('=').ok_or(PairError::MissingEquals(i))?;
        let key = key.trim();
        if key.is_empty() {
            return Err(PairError::EmptyKey(i));
        }
        if map.insert(key.to_string(), value.trim().to_string()).is_some() {
            return Err(PairError::DuplicateKey(key.to_string()));
        }
    }
    Ok(map)
}
