use task::Span;
use task::report::{Entry, summary};

fn main() {
    let entries = vec![
        Entry { project: "garden".into(), span: Span::from_minutes(50) },
        Entry { project: "taxes".into(), span: Span::from_secs(1810) },
        Entry { project: "garden".into(), span: Span::from_minutes(25) },
    ];
    print!("{}", summary(&entries));
}
