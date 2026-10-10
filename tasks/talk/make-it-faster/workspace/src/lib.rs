use std::collections::{BTreeSet, HashMap};

/// A tiny inverted index from lowercase words to document ids.
#[derive(Debug, Default)]
pub struct Index {
    words: HashMap<String, BTreeSet<usize>>,
    docs: Vec<String>,
}

impl Index {
    pub fn add(&mut self, text: &str) -> usize {
        let id = self.docs.len();
        for w in text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()) {
            self.words.entry(w.to_lowercase()).or_default().insert(id);
        }
        self.docs.push(text.to_string());
        id
    }

    /// Ids of documents containing every word of `query`.
    pub fn search(&self, query: &str) -> Vec<usize> {
        let mut sets = query
            .split_whitespace()
            .map(|w| self.words.get(&w.to_lowercase()));
        let Some(Some(first)) = sets.next() else {
            return Vec::new();
        };
        let mut hits = first.clone();
        for s in sets {
            match s {
                Some(s) => hits.retain(|id| s.contains(id)),
                None => return Vec::new(),
            }
        }
        hits.into_iter().collect()
    }

    pub fn doc(&self, id: usize) -> Option<&str> {
        self.docs.get(id).map(String::as_str)
    }
}
