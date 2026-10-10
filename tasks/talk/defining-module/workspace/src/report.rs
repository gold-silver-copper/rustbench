use crate::invoice::{Line, round_total};
use crate::money::to_display;

pub fn summary(lines: &[Line]) -> String {
    format!("{} lines, total {}", lines.len(), to_display(round_total(lines)))
}
