use task::{Audience, Post, feed};

#[test]
fn member_sees_everything() {
    let posts = [
        Post {
            id: 1,
            audience: Audience::Public,
        },
        Post {
            id: 2,
            audience: Audience::MembersOnly,
        },
    ];
    assert_eq!(feed(&posts, Some("mia")), vec![1, 2]);
}
