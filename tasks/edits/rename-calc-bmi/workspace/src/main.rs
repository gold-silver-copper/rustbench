use task::calc_bmi;
use task::report::{Person, average_bmi, line};

fn main() {
    let people = vec![
        Person { name: "Iris".into(), weight_kg: 58.0, height_cm: 165.0 },
        Person { name: "Tom".into(), weight_kg: 92.0, height_cm: 180.0 },
    ];
    for p in &people {
        println!("{}", line(p));
    }
    if let Some(avg) = average_bmi(&people) {
        println!("average {avg:.1}");
    }
    println!("reference: {:.1}", calc_bmi(70.0, 175.0));
}
