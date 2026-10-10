#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub volume: u8,
    pub muted: bool,
    pub theme: String,
}

impl Settings {
    pub fn standard() -> Settings {
        Settings { volume: 70, muted: false, theme: "light".to_string() }
    }
}
