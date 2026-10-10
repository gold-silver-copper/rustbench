use task::listing::{Contact, page_lines};
use task::paginate;

fn main() {
    let contacts: Vec<Contact> = (1..=14)
        .map(|i| Contact { name: format!("person{i:02}"), phone: format!("555-01{i:02}") })
        .collect();
    for line in page_lines(&contacts, 1) {
        println!("{line}");
    }
    let ids: Vec<u32> = (100..130).collect();
    println!("{:?}", paginate(&ids, 2, 10));
}
