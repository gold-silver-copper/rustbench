/// Something that can describe itself in one line.
pub trait Describe {
    fn describe(&self) -> String;
}

pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

pub struct Movie {
    pub title: String,
    pub year: u16,
    pub minutes: u32,
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}

impl Describe for Movie {
    fn describe(&self) -> String {
        format!("{} ({}, {} min)", self.title, self.year, self.minutes)
    }
}
