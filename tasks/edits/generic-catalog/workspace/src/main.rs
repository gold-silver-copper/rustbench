use task::{Book, catalog};

fn main() {
    let shelf = vec![
        Book {
            title: "Dune".into(),
            author: "Frank Herbert".into(),
            pages: 412,
        },
        Book {
            title: "Emma".into(),
            author: "Jane Austen".into(),
            pages: 474,
        },
    ];
    print!("{}", catalog(&shelf));
}
