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
    todo!()
}
