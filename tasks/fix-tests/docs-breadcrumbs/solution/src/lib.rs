//! Breadcrumb trails for a documentation site: "Home > Guides > Install".

use std::collections::HashMap;

/// `parents` maps each page to its parent; the root page has no entry.
/// Returns the trail from the root down to `page`, joined with " > ".
pub fn trail(parents: &HashMap<&str, &str>, page: &str) -> String {
    let mut chain = vec![page];
    let mut current = page;
    while let Some(&parent) = parents.get(current) {
        if chain.len() > parents.len() {
            break; // a cycle in the site map; stop rather than loop forever
        }
        chain.push(parent);
        current = parent;
    }
    chain.reverse();
    chain.join(" > ")
}
