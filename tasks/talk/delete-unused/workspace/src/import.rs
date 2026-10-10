use crate::Contact;

pub fn import_csv(text: &str) -> Vec<Contact> {
    text.lines()
        .skip(1)
        .filter_map(|l| l.split_once(','))
        .map(|(n, e)| Contact {
            name: n.to_string(),
            email: e.to_string(),
        })
        .collect()
}

/// Old address-book format: `Name <email>` per line.
pub fn import_legacy(text: &str) -> Vec<Contact> {
    text.lines()
        .filter_map(|l| {
            let (name, rest) = l.split_once('<')?;
            Some(Contact {
                name: name.trim().to_string(),
                email: rest.trim_end_matches('>').to_string(),
            })
        })
        .collect()
}
