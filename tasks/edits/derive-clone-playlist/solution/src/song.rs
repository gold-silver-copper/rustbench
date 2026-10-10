#[derive(Debug, Clone)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub seconds: u32,
}

impl Song {
    pub fn new(title: &str, artist: &str, seconds: u32) -> Self {
        Song {
            title: title.to_string(),
            artist: artist.to_string(),
            seconds,
        }
    }

    /// "3:07" style length.
    pub fn length(&self) -> String {
        format!("{}:{:02}", self.seconds / 60, self.seconds % 60)
    }
}
