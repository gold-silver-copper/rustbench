//! Validating a sign-up form. Every problem is reported at once.

pub struct Form {
    pub username: String,
    pub email: String,
    pub age: u32,
}

fn long_enough(f: &Form) -> bool {
    f.username.chars().count() >= 3
}

fn has_at(f: &Form) -> bool {
    f.email.contains('@')
}

fn old_enough(f: &Form) -> bool {
    f.age >= 13
}

const RULES: [(fn(&Form) -> bool, &str); 3] = [
    (long_enough, "username too short"),
    (has_at, "email needs an @"),
    (old_enough, "must be 13 or older"),
];

/// Every problem with the form, in the order the fields appear.
pub fn problems(form: &Form) -> Vec<&'static str> {
    let mut out = Vec::new();
    for (ok, message) in RULES {
        if !ok(form) {
            out.push(message);
            return out;
        }
    }
    out
}
