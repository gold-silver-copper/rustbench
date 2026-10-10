#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub pages: u32,
}

impl Book {
    /// "Title (Author, Year)"
    pub fn citation(&self) -> String {
        format!("{} ({}, {})", self.title, self.author, self.year)
    }

    pub fn is_classic(&self) -> bool {
        self.year < 1950
    }
}
