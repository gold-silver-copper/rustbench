use crate::SESSION_TIMEOUT_SECS;

/// A logged-in user's session. Times are seconds since the epoch.
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub user: String,
    pub last_seen: u64,
}

impl Session {
    pub fn start(user: &str, now: u64) -> Session {
        Session { user: user.to_string(), last_seen: now }
    }

    /// Sessions expire `SESSION_TIMEOUT_SECS` seconds after the last request.
    pub fn is_expired(&self, now: u64) -> bool {
        now.saturating_sub(self.last_seen) >= SESSION_TIMEOUT_SECS
    }

    /// Seconds left before the session expires.
    pub fn remaining(&self, now: u64) -> u64 {
        (self.last_seen + SESSION_TIMEOUT_SECS).saturating_sub(now)
    }

    pub fn touch(&mut self, now: u64) {
        self.last_seen = now;
    }
}
