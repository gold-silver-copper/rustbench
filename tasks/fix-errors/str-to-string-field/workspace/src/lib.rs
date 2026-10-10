pub struct Ticket {
    pub title: String,
    pub status: String,
    pub points: u32,
}

impl Ticket {
    /// A new ticket starts out "open".
    pub fn new(title: &str, points: u32) -> Ticket {
        Ticket { title: title.to_string(), status: "open", points }
    }

    pub fn close(&mut self) {
        self.status = String::from("closed");
    }
}
