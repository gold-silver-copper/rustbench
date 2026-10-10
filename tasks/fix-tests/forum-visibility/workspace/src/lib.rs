//! Which posts a visitor can see on a small forum.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Audience {
    Public,
    MembersOnly,
}

pub struct Post {
    pub id: u32,
    pub audience: Audience,
}

/// Public posts are visible to everyone; members-only posts need a logged-in viewer.
pub fn can_see(post: &Post, viewer: Option<&str>) -> bool {
    match post.audience {
        Audience::Public => true,
        Audience::MembersOnly => viewer.is_none(),
    }
}

/// Ids of the posts `viewer` can see, in order.
pub fn feed(posts: &[Post], viewer: Option<&str>) -> Vec<u32> {
    posts
        .iter()
        .filter(|p| can_see(p, viewer))
        .map(|p| p.id)
        .collect()
}
