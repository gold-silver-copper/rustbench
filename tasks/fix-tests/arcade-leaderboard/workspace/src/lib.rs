//! A high-score table for an arcade game.

#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub player: String,
    pub points: u32,
}

/// The top `n` scores, highest first. Equal scores are listed by player name, A to Z.
pub fn top(scores: &[Score], n: usize) -> Vec<Score> {
    let mut sorted = scores.to_vec();
    sorted.sort_by(|a, b| {
        a.points
            .cmp(&b.points)
            .then_with(|| a.player.cmp(&b.player))
    });
    sorted.truncate(n);
    sorted
}
