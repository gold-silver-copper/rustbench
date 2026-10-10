#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Heading {
    North,
    East,
    South,
    West,
}

pub fn turn_right(h: Heading) -> Heading {
    match h {
        Heading::North => Heading::East,
        Heading::East => Heading::South,
        Heading::South => Heading::West,
        Heading::West => Heading::North,
    }
}

/// One step: North is y+1, East is x+1, South is y-1, West is x-1.
pub fn step(pos: (i32, i32), h: Heading) -> (i32, i32) {
    let (x, y) = pos;
    match h {
        Heading::North => (x, y + 1),
        Heading::East => (x + 1, y),
        Heading::South => (x, y - 1),
        Heading::West => (x - 1, y),
    }
}
