#[derive(Debug, Clone, PartialEq)]
pub struct Lock {
    pub owner: String,
    pub held: bool,
}

impl Lock {
    pub fn release(&mut self) {
        self.held = false;
    }
}
