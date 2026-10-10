use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

/// Number of distinct tiles on a walk.
pub fn distinct_tiles(path: &[Tile]) -> usize {
    path.iter().copied().collect::<HashSet<Tile>>().len()
}
