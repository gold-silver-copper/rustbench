use std::collections::{BTreeMap, BTreeSet};

/// Orders tasks so that every task comes after all the tasks it depends on.
///
/// The tasks are every name in `tasks` plus every name in `deps`, each
/// counted once. A pair `(task, dependency)` in `deps` means `task` must
/// come after `dependency`; pairs may repeat, and `(a, a)` makes `a` depend
/// on itself.
///
/// The order is built one task at a time: the next task is always the
/// alphabetically smallest (by `str` ordering) of the tasks not yet placed
/// whose dependencies have all been placed. If tasks remain but none of
/// them can be placed, returns `Err` with all remaining tasks in
/// alphabetical order. No tasks gives `Ok(vec![])`.
///
/// ```
/// let order = task::build_order(&["lint"], &[("test", "build"), ("deploy", "test"), ("build", "fetch")]);
/// assert_eq!(order, Ok(vec!["fetch".into(), "build".into(), "lint".into(), "test".into(), "deploy".into()]));
/// ```
pub fn build_order(tasks: &[&str], deps: &[(&str, &str)]) -> Result<Vec<String>, Vec<String>> {
    let mut waiting: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for &t in tasks {
        waiting.entry(t).or_default();
    }
    for &(t, d) in deps {
        waiting.entry(t).or_default().insert(d);
        waiting.entry(d).or_default();
    }
    let mut order = Vec::new();
    while let Some(t) = waiting.iter().find(|(_, ds)| ds.is_empty()).map(|(t, _)| *t) {
        waiting.remove(t);
        for ds in waiting.values_mut() {
            ds.remove(t);
        }
        order.push(t.to_string());
    }
    if waiting.is_empty() {
        Ok(order)
    } else {
        Err(waiting.keys().map(|t| t.to_string()).collect())
    }
}
