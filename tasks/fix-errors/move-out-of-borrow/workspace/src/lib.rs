pub struct Contact {
    pub name: String,
    pub email: Option<String>,
}

impl Contact {
    /// The email if there is one, otherwise "no email on file for NAME".
    pub fn address(&self) -> String {
        match self.email {
            Some(e) => e,
            None => format!("no email on file for {}", self.name),
        }
    }
}
