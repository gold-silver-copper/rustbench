pub struct Contact {
    pub name: String,
    pub email: Option<String>,
}

impl Contact {
    /// The email if there is one, otherwise "no email on file for NAME".
    pub fn address(&self) -> String {
        match self.email {
            Some(ref e) => e.clone(),
            None => format!("no email on file for {}", self.name),
        }
    }
}
