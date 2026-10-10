use task::Version;
use task::releases;

#[test]
fn orders_by_numbers_not_label() {
    let mut vs = vec![
        Version::labeled(1, 10, 0, "alpha"),
        Version::labeled(1, 2, 3, "zeta"),
        Version::new(2, 0, 0),
        Version::labeled(1, 9, 9, "beta"),
        Version::new(0, 1, 0),
    ];
    vs.sort();
    let shown: Vec<String> = vs.iter().map(|v| v.to_string()).collect();
    assert_eq!(shown, ["0.1.0", "1.2.3-zeta", "1.9.9-beta", "1.10.0-alpha", "2.0.0"]);
    assert_eq!(vs.iter().max(), Some(&Version::new(2, 0, 0)));
}

#[test]
fn label_is_ignored_by_equality() {
    assert_eq!(Version::labeled(1, 2, 3, "rc"), Version::new(1, 2, 3));
    assert!(Version::new(1, 2, 4) > Version::labeled(1, 2, 3, "zzz"));
    assert!(Version::new(0, 9, 9) < Version::new(1, 0, 0));
}

#[test]
fn latest_still_works() {
    let list = releases::parse_list("1.2.0\n1.10.1-beta\nnope\n1.9.9\n");
    assert_eq!(list.len(), 3);
    assert_eq!(releases::latest(&list).map(|v| v.to_string()).as_deref(), Some("1.10.1-beta"));
    assert!(releases::latest(&[]).is_none());
}
