use crate::shapes::{Circle, Square};

/// One line per shape, squares first: "square: area 4.00, perimeter 8.00".
pub fn report(squares: &[Square], circles: &[Circle]) -> Vec<String> {
    let mut lines = Vec::new();
    for s in squares {
        lines.push(format!("square: area {:.2}, perimeter {:.2}", s.area(), s.perimeter()));
    }
    for c in circles {
        lines.push(format!("circle: area {:.2}, perimeter {:.2}", c.area(), c.perimeter()));
    }
    lines
}
