/// A pretend connection that fails a fixed number of times before answering.
pub struct FlakyLink {
    pub failures_left: u32,
    pub calls: u32,
}

impl FlakyLink {
    pub fn new(failures: u32) -> Self {
        FlakyLink {
            failures_left: failures,
            calls: 0,
        }
    }

    pub fn request(&mut self, path: &str) -> Result<String, String> {
        self.calls += 1;
        if self.failures_left > 0 {
            self.failures_left -= 1;
            Err(format!("timeout on {path}"))
        } else {
            Ok(format!("200 OK {path}"))
        }
    }
}
