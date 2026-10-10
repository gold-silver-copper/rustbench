use task::Runner;

#[test]
fn ranks() {
    let mut r = vec![
        Runner { name: "a".into(), seconds: 12.5 },
        Runner { name: "b".into(), seconds: 11.0 },
        Runner { name: "c".into(), seconds: 12.5 },
    ];
    task::rank(&mut r);
    let names: Vec<&str> = r.iter().map(|x| x.name.as_str()).collect();
    assert_eq!(names, ["b", "a", "c"]);
}
