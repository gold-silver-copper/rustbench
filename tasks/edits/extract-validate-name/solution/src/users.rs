use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u32,
    pub name: String,
}

/// Usernames are 3 to 16 ASCII letters, digits or '_', not starting with a digit.
pub fn validate_username(name: &str) -> Result<(), String> {
    let len = name.chars().count();
    if !(3..=16).contains(&len) {
        return Err(format!("username must be 3 to 16 characters, got {len}"));
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("username may only contain letters, digits and '_'".to_string());
    }
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        return Err("username must not start with a digit".to_string());
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct Registry {
    by_name: BTreeMap<String, User>,
    next_id: u32,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a user; returns the new id.
    pub fn register(&mut self, name: &str) -> Result<u32, String> {
        validate_username(name)?;
        if self.by_name.contains_key(name) {
            return Err(format!("username {name} is taken"));
        }
        self.next_id += 1;
        let user = User {
            id: self.next_id,
            name: name.to_string(),
        };
        self.by_name.insert(name.to_string(), user);
        Ok(self.next_id)
    }

    /// Change a user's name.
    pub fn rename(&mut self, old: &str, new: &str) -> Result<(), String> {
        validate_username(new)?;
        if self.by_name.contains_key(new) {
            return Err(format!("username {new} is taken"));
        }
        let mut user = self
            .by_name
            .remove(old)
            .ok_or_else(|| format!("no user named {old}"))?;
        user.name = new.to_string();
        self.by_name.insert(new.to_string(), user);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&User> {
        self.by_name.get(name)
    }

    pub fn len(&self) -> usize {
        self.by_name.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}
