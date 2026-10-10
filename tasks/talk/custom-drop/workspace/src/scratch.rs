use std::path::PathBuf;

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new(name: &str) -> Self {
        TempFile {
            path: std::env::temp_dir().join(name),
        }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
