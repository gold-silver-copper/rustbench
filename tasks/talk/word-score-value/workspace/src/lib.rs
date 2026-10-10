fn letter_value(c: char) -> u32 {
    match c.to_ascii_lowercase() {
        'q' | 'z' => 10,
        'j' | 'x' => 8,
        'k' => 5,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'b' | 'c' | 'm' | 'p' => 3,
        'd' | 'g' => 2,
        _ => 1,
    }
}

/// Proper nouns score double; long words get a flat bonus on top.
pub fn word_score(word: &str) -> u32 {
    let base: u32 = word
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(letter_value)
        .sum();
    let multiplier = if word.starts_with(|c: char| c.is_uppercase()) {
        2
    } else {
        1
    };
    let bonus = if word.len() >= 7 { 50 } else { 0 };
    base * multiplier + bonus
}
