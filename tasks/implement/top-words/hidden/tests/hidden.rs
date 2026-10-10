use task::top_words;

fn owned(v: &[(&str, usize)]) -> Vec<(String, usize)> {
    v.iter().map(|(w, n)| (w.to_string(), *n)).collect()
}

#[test]
fn ranks_and_ties() {
    let text = "b a c b a d, B! c-a";
    assert_eq!(top_words(text, 3), owned(&[("a", 3), ("b", 3), ("c", 2)]));
    assert_eq!(top_words(text, 10), owned(&[("a", 3), ("b", 3), ("c", 2), ("d", 1)]));
}

#[test]
fn word_boundaries() {
    assert_eq!(top_words("don't DON'T x2x naïve", 10), owned(&[("don", 2), ("t", 2), ("x", 2), ("na", 1), ("ve", 1)]));
}

#[test]
fn empty_cases() {
    assert_eq!(top_words("anything at all", 0), vec![]);
    assert_eq!(top_words("", 3), vec![]);
    assert_eq!(top_words("123 ... !!", 3), vec![]);
}
