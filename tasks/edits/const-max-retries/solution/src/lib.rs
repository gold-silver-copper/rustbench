pub mod fetch;
pub mod link;

pub use fetch::{FetchError, fetch_with_retry};
pub use link::FlakyLink;

/// How many times a request is tried before giving up.
pub const MAX_RETRIES: u32 = 3;
