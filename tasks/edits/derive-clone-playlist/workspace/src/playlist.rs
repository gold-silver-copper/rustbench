use crate::song::Song;

#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<Song>,
}

impl Playlist {
    pub fn new(name: &str) -> Self {
        Playlist {
            name: name.to_string(),
            songs: Vec::new(),
        }
    }

    pub fn add(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub fn total_seconds(&self) -> u32 {
        self.songs.iter().map(|s| s.seconds).sum()
    }

    /// Remove every song by `artist`; returns how many were removed.
    pub fn remove_artist(&mut self, artist: &str) -> usize {
        let before = self.songs.len();
        self.songs.retain(|s| s.artist != artist);
        before - self.songs.len()
    }
}
