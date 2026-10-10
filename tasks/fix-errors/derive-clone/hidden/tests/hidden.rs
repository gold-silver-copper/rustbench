use task::settings::Settings;

#[test]
fn mutes_a_copy() {
    let s = Settings::standard();
    let m = task::profile::muted(&s);
    assert!(m.muted);
    assert!(!s.muted);
    assert_eq!(m.volume, 70);
    assert_eq!(m.theme, "light");
}
