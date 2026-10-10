//! Standings table for a chess club.

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub points: u32,
    pub wins: u32,
}

/// Names in standings order: most points first; on equal points, more wins
/// first; then by name, A to Z.
pub fn standings(players: &[Player]) -> Vec<String> {
    let mut sorted = players.to_vec();
    sorted.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then(b.wins.cmp(&a.wins))
            .then(a.name.cmp(&b.name))
    });
    sorted.into_iter().map(|p| p.name).collect()
}
