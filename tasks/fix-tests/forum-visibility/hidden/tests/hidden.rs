use task::{Audience, Post, can_see, feed};

#[test]
fn guests_see_public_only() {
    let posts = [
        Post {
            id: 7,
            audience: Audience::MembersOnly,
        },
        Post {
            id: 8,
            audience: Audience::Public,
        },
        Post {
            id: 9,
            audience: Audience::MembersOnly,
        },
    ];
    assert_eq!(feed(&posts, None), vec![8]);
    assert_eq!(feed(&posts, Some("joe")), vec![7, 8, 9]);
}

#[test]
fn single_posts() {
    let members = Post {
        id: 1,
        audience: Audience::MembersOnly,
    };
    assert!(can_see(&members, Some("x")));
    assert!(!can_see(&members, None));
}
