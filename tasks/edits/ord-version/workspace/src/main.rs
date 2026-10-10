use task::releases;

fn main() {
    let list = releases::parse_list("1.2.0\n1.10.1-beta\n1.9.9\n0.9.0-lts\n");
    match releases::latest(&list) {
        Some(v) => println!("latest: {v}"),
        None => println!("no releases"),
    }
}
