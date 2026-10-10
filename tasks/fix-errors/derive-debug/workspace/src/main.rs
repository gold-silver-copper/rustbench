use task::Reading;

fn main() {
    let readings = vec![
        Reading { sensor: "attic".into(), celsius: 18.5 },
        Reading { sensor: "cellar".into(), celsius: 12.0 },
    ];
    println!("{:?}", readings);
}
