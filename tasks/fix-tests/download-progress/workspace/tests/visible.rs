use task::{Download, status};

#[test]
fn status_shows_percent_done() {
    let d = Download {
        name: "photo.jpg".into(),
        received: 400,
        size: 1000,
    };
    assert_eq!(status(&d), "photo.jpg 40%");
}
