use task::Stack;
use task::undo::Editor;

#[test]
fn peek_top() {
    let mut s = Stack::new();
    assert_eq!(s.peek(), None);
    s.push("a");
    s.push("b");
    assert_eq!(s.peek(), Some(&"b"));
    assert_eq!(s.len(), 2);
    assert_eq!(s.pop(), Some("b"));
    assert_eq!(s.peek(), Some(&"a"));
    s.pop();
    assert_eq!(s.peek(), None);
    assert!(s.is_empty());
}

#[test]
fn undo_still_works() {
    let mut ed = Editor::new();
    ed.type_text("hi");
    ed.type_text("!");
    assert!(ed.undo());
    assert_eq!(ed.text, "hi");
    assert!(ed.undo());
    assert!(!ed.undo());
}
