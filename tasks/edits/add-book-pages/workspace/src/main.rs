use task::{Book, Catalog};

fn main() {
    let mut c = Catalog::with_starter_books();
    c.add(Book {
        title: "Lanterns".to_string(),
        author: "M. Okafor".to_string(),
        year: 1945,
    });
    println!("{} books", c.len());
    for b in c.classics() {
        println!("classic: {}", b.citation());
    }
}
