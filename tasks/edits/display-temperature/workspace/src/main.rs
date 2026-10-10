use task::Temperature;

fn main() {
    let hourly = [
        Temperature::from_celsius(4.2),
        Temperature::from_fahrenheit(77.0),
        Temperature::from_celsius(31.0),
    ];
    if let Some((lo, hi)) = task::forecast::range(&hourly) {
        println!("low {:?}, high {:?}", lo, hi);
    }
    println!("vent: {}", task::forecast::needs_venting(&hourly, Temperature::from_celsius(30.0)));
}
