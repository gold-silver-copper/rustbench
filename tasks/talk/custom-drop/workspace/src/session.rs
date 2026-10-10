use crate::lock::Lock;

pub struct Session {
    pub user: String,
    pub lock: Lock,
}

impl Session {
    /// Ends the session, releasing its lock.
    pub fn close(mut self) -> String {
        self.lock.release();
        let user = self.user.clone();
        drop(self);
        user
    }
}
