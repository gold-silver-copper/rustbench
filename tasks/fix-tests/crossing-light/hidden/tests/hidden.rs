use task::{Light, light_at};

#[test]
fn full_cycle() {
    assert_eq!(Light::Red.next(), Light::Green);
    assert_eq!(Light::Yellow.next(), Light::Red);
    assert_eq!(Light::Green.next().next().next(), Light::Green);
}

#[test]
fn timeline() {
    assert_eq!(light_at(0), Light::Green);
    assert_eq!(light_at(29), Light::Green);
    assert_eq!(light_at(30), Light::Yellow);
    assert_eq!(light_at(34), Light::Red);
    assert_eq!(light_at(53), Light::Red);
    assert_eq!(light_at(54), Light::Green);
    assert_eq!(light_at(54 * 3 + 31), Light::Yellow);
}
