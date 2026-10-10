use crate::SESSION_TIMEOUT_SECS;
use crate::session::Session;

/// The `Set-Cookie` header value for a new session.
pub fn cookie(session: &Session) -> String {
    format!("sid={}; Max-Age={}; HttpOnly", session.user, SESSION_TIMEOUT_SECS)
}

/// Check a request against a session, refreshing it if still valid.
pub fn authorize(session: &mut Session, now: u64) -> Result<(), String> {
    if session.is_expired(now) {
        return Err(format!("session for {} expired (limit {}s)", session.user, SESSION_TIMEOUT_SECS));
    }
    session.touch(now);
    Ok(())
}
