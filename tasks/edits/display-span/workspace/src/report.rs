use crate::Span;

/// One logged block of work.
pub struct Entry {
    pub project: String,
    pub span: Span,
}

/// Total time per project, in the order projects first appear.
pub fn totals(entries: &[Entry]) -> Vec<(String, Span)> {
    let mut out: Vec<(String, Span)> = Vec::new();
    for e in entries {
        match out.iter_mut().find(|(p, _)| *p == e.project) {
            Some((_, total)) => *total = total.add(e.span),
            None => out.push((e.project.clone(), e.span)),
        }
    }
    out
}

/// One line per project: `<project>: <seconds>s`.
pub fn summary(entries: &[Entry]) -> String {
    totals(entries)
        .iter()
        .map(|(p, s)| format!("{p}: {}s\n", s.secs))
        .collect()
}
