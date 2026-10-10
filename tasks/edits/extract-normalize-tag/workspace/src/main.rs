use task::Note;
use task::search::tagged;
use task::tags::add_tag;

fn main() {
    let mut a = Note { title: "Groceries".into(), ..Note::default() };
    add_tag(&mut a, "  To Do! ");
    let mut b = Note { title: "Taxes".into(), ..Note::default() };
    add_tag(&mut b, "to do");
    add_tag(&mut b, "Money");
    let notes = [a, b];
    println!("{:?}", tagged(&notes, "TO DO"));
}
