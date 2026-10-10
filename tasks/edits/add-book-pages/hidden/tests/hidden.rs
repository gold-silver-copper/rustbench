use task::{Book, Catalog};

#[test]
fn book_has_pages() {
    let b = Book {
        title: "Small Hours".to_string(),
        author: "P. Lind".to_string(),
        year: 1932,
        pages: 214,
    };
    assert_eq!(b.pages, 214);
    assert_eq!(b.citation(), "Small Hours (P. Lind, 1932)");
    assert!(b.is_classic());
}

#[test]
fn starter_books_have_zero_pages() {
    let books = task::seed::starter_books();
    assert_eq!(books.len(), 3);
    assert!(books.iter().all(|b| b.pages == 0));
    let c = Catalog::with_starter_books();
    assert_eq!(c.classics().len(), 1);
    assert_eq!(c.by_author("L. Brandt")[0].year, 1997);
}
