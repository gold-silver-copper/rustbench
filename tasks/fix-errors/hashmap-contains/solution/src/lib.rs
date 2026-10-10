use std::collections::HashMap;

/// Registers a username unless it's taken; returns whether it was added.
pub fn register(users: &mut HashMap<String, u32>, name: &str, id: u32) -> bool {
    if users.contains_key(name) {
        return false;
    }
    users.insert(name.to_string(), id);
    true
}
