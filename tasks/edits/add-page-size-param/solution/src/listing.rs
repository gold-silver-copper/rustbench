use crate::paginate;

pub struct Contact {
    pub name: String,
    pub phone: String,
}

/// The lines shown for one page of contacts, sorted by name.
pub fn page_lines(contacts: &[Contact], page: usize) -> Vec<String> {
    let mut names: Vec<&Contact> = contacts.iter().collect();
    names.sort_by(|a, b| a.name.cmp(&b.name));
    paginate(&names, page, 10)
        .into_iter()
        .map(|c| format!("{:<12}{}", c.name, c.phone))
        .collect()
}

/// The first page of names that start with `prefix`.
pub fn search(contacts: &[Contact], prefix: &str) -> Vec<String> {
    let hits: Vec<String> = contacts
        .iter()
        .filter(|c| c.name.starts_with(prefix))
        .map(|c| c.name.clone())
        .collect();
    paginate(&hits, 0, 10)
}
