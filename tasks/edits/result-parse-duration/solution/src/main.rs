use task::Config;

fn main() {
    let text = std::env::args().nth(1).unwrap_or_else(|| "interval = 15m\ntimeout = 45s".to_string());
    match Config::parse(&text) {
        Ok(config) => println!("{config:?}"),
        Err(e) => eprintln!("bad config: {e}"),
    }
}
