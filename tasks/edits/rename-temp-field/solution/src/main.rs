use task::Reading;

fn main() {
    let data = "6,-1.5,90\n12,18.0,55\n15,36.2,20";
    let readings: Vec<Reading> = data.lines().filter_map(Reading::parse).collect();
    for r in &readings {
        println!("{:02}:00 {:.1} C / {:.1} F", r.hour, r.celsius, r.fahrenheit());
    }
    if let Some(m) = task::stats::mean(&readings) {
        println!("mean {m:.2}");
    }
    let night = Reading { hour: 23, celsius: 4.0, humidity: 80.0 };
    println!("night {:.1}", night.celsius);
    for a in task::alerts::alerts(&readings) {
        println!("{a:?}");
    }
}
