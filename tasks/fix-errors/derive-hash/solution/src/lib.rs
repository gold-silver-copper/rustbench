use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub row: u32,
    pub col: u32,
}

/// How many times each cell was hit.
pub fn hit_counts(shots: &[Cell]) -> HashMap<Cell, u32> {
    let mut counts = HashMap::new();
    for s in shots {
        *counts.entry(s.clone()).or_insert(0) += 1;
    }
    counts
}
