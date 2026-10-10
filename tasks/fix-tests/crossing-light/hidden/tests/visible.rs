use task::Light;

#[test]
fn yellow_turns_red() {
    assert_eq!(Light::Green.next().next(), Light::Red);
}
