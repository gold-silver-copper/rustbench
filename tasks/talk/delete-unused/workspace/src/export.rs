use crate::Contact;

pub fn export_csv(contacts: &[Contact]) -> String {
    let mut out = String::from("name,email\n");
    for c in contacts {
        out += &format!("{},{}\n", c.name, c.email);
    }
    out
}

pub fn export_json(contacts: &[Contact]) -> String {
    let items: Vec<String> = contacts
        .iter()
        .map(|c| format!("{{\"name\":{:?},\"email\":{:?}}}", c.name, c.email))
        .collect();
    format!("[{}]", items.join(","))
}
