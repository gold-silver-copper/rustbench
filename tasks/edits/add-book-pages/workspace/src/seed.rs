use crate::book::Book;

/// The books every new catalog starts with.
pub fn starter_books() -> Vec<Book> {
    vec![
        Book {
            title: "The River Road".to_string(),
            author: "M. Okafor".to_string(),
            year: 1938,
        },
        Book {
            title: "Glass Orchards".to_string(),
            author: "L. Brandt".to_string(),
            year: 1997,
        },
        Book {
            title: "Notes on Salt".to_string(),
            author: "A. Ivers".to_string(),
            year: 2011,
        },
    ]
}
