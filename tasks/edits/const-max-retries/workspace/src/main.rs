use task::{FlakyLink, fetch_with_retry};

fn main() {
    let mut link = FlakyLink::new(2);
    match fetch_with_retry(&mut link, "/status") {
        Ok(body) => println!("{body}"),
        Err(e) => println!("{}", e.message()),
    }
}
