use std::io::BufRead;

use task::{exec, parse};

fn main() {
    let mut counter = 0;
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        match parse::parse(&line) {
            Some(cmd) => println!("{}", exec::apply(&mut counter, cmd)),
            None => println!("?"),
        }
    }
}
