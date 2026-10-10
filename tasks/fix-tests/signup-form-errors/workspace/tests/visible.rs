use task::{Form, problems};

#[test]
fn reports_every_problem() {
    let form = Form {
        username: "al".into(),
        email: "al.example.com".into(),
        age: 30,
    };
    assert_eq!(
        problems(&form),
        vec!["username too short", "email needs an @"]
    );
}
