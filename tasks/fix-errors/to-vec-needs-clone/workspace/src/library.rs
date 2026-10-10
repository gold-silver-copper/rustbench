use crate::book::Book;

/// Books published before `year`, oldest first. The shelf is left untouched.
pub fn older_than(shelf: &[Book], year: u16) -> Vec<Book> {
    let mut found = shelf.to_vec();
    found.retain(|b| b.year < year);
    found.sort_by_key(|b| b.year);
    found
}
