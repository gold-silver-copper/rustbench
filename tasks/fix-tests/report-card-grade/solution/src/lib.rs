//! Report cards: turning a percentage into a letter grade.

/// A for 90 and up, B for 80 and up, C for 70 and up, D for 60 and up, F below.
pub fn grade(percent: u32) -> char {
    if percent >= 90 {
        'A'
    } else if percent >= 80 {
        'B'
    } else if percent >= 70 {
        'C'
    } else if percent >= 60 {
        'D'
    } else {
        'F'
    }
}

/// Each student's name with their grade.
pub fn report(scores: &[(&str, u32)]) -> Vec<(String, char)> {
    scores
        .iter()
        .map(|&(name, p)| (name.to_string(), grade(p)))
        .collect()
}
