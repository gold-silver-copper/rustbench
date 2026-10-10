use crate::book::Book;

#[derive(Debug, Default)]
pub struct Catalog {
    books: Vec<Book>,
}

impl Catalog {
    pub fn with_starter_books() -> Catalog {
        Catalog { books: crate::seed::starter_books() }
    }

    pub fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    pub fn classics(&self) -> Vec<&Book> {
        self.books.iter().filter(|b| b.is_classic()).collect()
    }

    pub fn len(&self) -> usize {
        self.books.len()
    }

    pub fn is_empty(&self) -> bool {
        self.books.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_by_author() {
        let mut c = Catalog::default();
        c.add(Book {
            title: "Two Winters".to_string(),
            author: "R. Holm".to_string(),
            year: 1921,
            pages: 0,
        });
        assert_eq!(c.by_author("R. Holm").len(), 1);
        assert_eq!(c.classics().len(), 1);
    }
}
