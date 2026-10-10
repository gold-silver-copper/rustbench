use task::{Playlist, Song};

fn main() {
    let mut p = Playlist::new("road trip");
    p.add(Song::new("Long Road", "The Wheels", 247));
    p.add(Song::new("Exit 12", "Mile Markers", 183));
    for s in &p.songs {
        println!("{} - {} ({})", s.artist, s.title, s.length());
    }
    println!("total: {}s", p.total_seconds());
}
