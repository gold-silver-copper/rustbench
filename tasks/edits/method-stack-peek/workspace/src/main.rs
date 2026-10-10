use task::undo::Editor;

fn main() {
    let mut ed = Editor::new();
    ed.type_text("hello");
    ed.type_text(" world");
    ed.undo();
    println!("{}", ed.text);
}
