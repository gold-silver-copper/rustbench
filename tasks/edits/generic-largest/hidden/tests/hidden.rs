use task::stats::{self, Steps};
use task::util::largest;

#[test]
fn works_for_floats_and_chars() {
    assert_eq!(largest(&[1.5, -2.0, 9.25, 3.0]), Some(9.25));
    assert_eq!(largest(&['q', 'z', 'a']), Some('z'));
    assert_eq!(largest::<f64>(&[]), None);
    assert_eq!(largest(&[7u8]), Some(7));
}

#[test]
fn ints_unchanged() {
    assert_eq!(largest(&[3, -1, 12, 12, 4]), Some(12));
    assert_eq!(largest::<i32>(&[]), None);
    let people = vec![
        Steps { name: "a".into(), days: vec![8_100, 12_400, 9_050] },
        Steps { name: "b".into(), days: vec![3_200, 15_020] },
        Steps { name: "c".into(), days: vec![] },
    ];
    assert_eq!(people[0].best_day(), Some(12_400));
    assert_eq!(stats::record(&people), Some(15_020));
    assert_eq!(stats::spread(&people[1].days), 11_820);
    assert_eq!(stats::spread(&[]), 0);
}
