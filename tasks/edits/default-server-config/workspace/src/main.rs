fn main() {
    let text = "# local dev\nport = 3000\nverbose = true\n";
    match task::parse::parse(text) {
        Ok(c) => println!("listening on {} (max {} connections)", c.address(), c.max_connections),
        Err(e) => eprintln!("config error: {e}"),
    }
}
