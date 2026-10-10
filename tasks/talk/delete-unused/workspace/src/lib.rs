pub mod export;
pub mod import;

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub name: String,
    pub email: String,
}
