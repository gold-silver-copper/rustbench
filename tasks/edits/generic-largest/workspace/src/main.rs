use task::stats::{self, Steps};
use task::util::largest;

fn main() {
    let people = vec![
        Steps { name: "ana".into(), days: vec![8_100, 12_400, 9_050] },
        Steps { name: "ben".into(), days: vec![3_200, 15_020] },
    ];
    for p in &people {
        println!("{}: best {:?}, spread {}", p.name, p.best_day(), stats::spread(&p.days));
    }
    println!("record: {:?}", stats::record(&people));
    println!("largest of nothing: {:?}", largest(&Vec::<i32>::new()));
}
