use task::Reading;

#[test]
fn warmest() {
    let r = vec![
        Reading { sensor: "a".into(), celsius: 1.0 },
        Reading { sensor: "b".into(), celsius: 3.0 },
    ];
    assert_eq!(task::warmest(&r), Some("b"));
    assert_eq!(format!("{:?}", r[0]), "Reading { sensor: \"a\", celsius: 1.0 }");
}
