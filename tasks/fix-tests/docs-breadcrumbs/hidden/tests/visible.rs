use std::collections::HashMap;
use task::trail;

#[test]
fn trail_from_home_down() {
    let parents = HashMap::from([("Install", "Guides"), ("Guides", "Home")]);
    assert_eq!(trail(&parents, "Install"), "Home > Guides > Install");
}
