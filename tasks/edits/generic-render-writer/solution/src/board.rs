use crate::Row;
use crate::table::render_table;

/// A scoreboard, kept sorted from best to worst.
#[derive(Debug, Default)]
pub struct Board {
    rows: Vec<Row>,
}

impl Board {
    pub fn record(&mut self, name: &str, score: u32) {
        self.rows.push(Row { name: name.to_string(), score });
        self.rows.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
    }

    /// The top `n` rows as a table.
    pub fn top(&self, n: usize) -> String {
        let mut s = String::new();
        let shown = &self.rows[..n.min(self.rows.len())];
        let _ = render_table(&mut s, shown);
        s
    }
}
