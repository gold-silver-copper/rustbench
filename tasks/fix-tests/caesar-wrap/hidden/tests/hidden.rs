use task::{decrypt, encrypt};

#[test]
fn rot13() {
    assert_eq!(encrypt("Hello, World!", 13), "Uryyb, Jbeyq!");
}

#[test]
fn zero_shift_and_round_trip() {
    assert_eq!(encrypt("Zebra", 0), "Zebra");
    assert_eq!(encrypt("Zebra", 26), "Zebra");
    for k in 0..30 {
        assert_eq!(
            decrypt(&encrypt("The quick brown fox, 42!", k), k),
            "The quick brown fox, 42!"
        );
    }
}
