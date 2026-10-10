//! Login sessions for a small web app.

pub mod auth;
pub mod session;

/// How long a session stays valid after the last request, in seconds.
pub const SESSION_TIMEOUT_SECS: u64 = 1800;
