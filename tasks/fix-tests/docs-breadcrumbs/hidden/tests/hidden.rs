use std::collections::HashMap;
use task::trail;

#[test]
fn deeper_and_shallow() {
    let parents = HashMap::from([
        ("Linux", "Install"),
        ("Install", "Guides"),
        ("Guides", "Home"),
        ("API", "Home"),
    ]);
    assert_eq!(trail(&parents, "Linux"), "Home > Guides > Install > Linux");
    assert_eq!(trail(&parents, "API"), "Home > API");
    assert_eq!(trail(&parents, "Home"), "Home");
    assert_eq!(trail(&parents, "Orphan"), "Orphan");
}
