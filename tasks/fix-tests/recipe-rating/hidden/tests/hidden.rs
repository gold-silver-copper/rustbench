use task::{average, label};

#[test]
fn averages() {
    assert_eq!(average(&[4, 5]), Some(4.5));
    assert_eq!(average(&[1, 2, 2]), Some(1.7));
    assert_eq!(average(&[3, 3, 4]), Some(3.3));
    assert_eq!(average(&[4, 4, 5, 4]), Some(4.3));
    assert_eq!(average(&[]), None);
}

#[test]
fn labels() {
    assert_eq!(label(&[2, 3, 3]), "2.7 stars");
    assert_eq!(label(&[]), "no ratings yet");
}
