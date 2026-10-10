use task::auth::{authorize, cookie};
use task::session::Session;

fn main() {
    let mut s = Session::start("mira", 1_000);
    println!("{}", cookie(&s));
    println!("{:?}", authorize(&mut s, 2_000));
    println!("{:?}", authorize(&mut s, 5_000));
}
