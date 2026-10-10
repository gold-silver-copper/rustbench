use task::{Form, problems};

fn form(username: &str, email: &str, age: u32) -> Form {
    Form {
        username: username.into(),
        email: email.into(),
        age,
    }
}

#[test]
fn combinations() {
    assert_eq!(problems(&form("alice", "a@b.c", 30)), Vec::<&str>::new());
    assert_eq!(
        problems(&form("alice", "a@b.c", 10)),
        vec!["must be 13 or older"]
    );
    assert_eq!(
        problems(&form("x", "nope", 5)),
        vec![
            "username too short",
            "email needs an @",
            "must be 13 or older"
        ]
    );
    assert_eq!(problems(&form("bob", "bob", 13)), vec!["email needs an @"]);
}
