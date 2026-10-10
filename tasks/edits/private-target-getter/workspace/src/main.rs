use task::Thermostat;
use task::{schedule, status};

fn main() {
    let mut t = Thermostat::new(18.2);
    for hour in [3, 7, 13] {
        schedule::apply(&mut t, hour);
        println!("{hour:02}:00 {}", status::line(&t));
    }
    schedule::bump(&mut t, 50.0);
    println!("max: {}", t.target);
}
