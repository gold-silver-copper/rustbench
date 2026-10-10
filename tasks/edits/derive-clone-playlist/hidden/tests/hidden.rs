use task::{Playlist, Song};

fn sample() -> Playlist {
    let mut p = Playlist::new("mix");
    p.add(Song::new("A", "Ann", 100));
    p.add(Song::new("B", "Bob", 200));
    p.add(Song::new("C", "Ann", 61));
    p
}

#[test]
fn clone_is_independent() {
    let original = sample();
    let mut copy = original.clone();
    copy.name.push_str(" (edit)");
    assert_eq!(copy.remove_artist("Ann"), 2);
    copy.songs[0].title = "B2".to_string();
    assert_eq!(original.name, "mix");
    assert_eq!(original.songs.len(), 3);
    assert_eq!(original.songs[1].title, "B");
    assert_eq!(original.total_seconds(), 361);
    assert_eq!(copy.total_seconds(), 200);
}

#[test]
fn song_clones() {
    let s = Song::new("Exit 12", "Mile Markers", 183);
    let t = s.clone();
    assert_eq!(t.title, s.title);
    assert_eq!(t.length(), "3:03");
}
