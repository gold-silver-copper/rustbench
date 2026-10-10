pub mod limits;

use limits::{MAX_FILES_PER_REQUEST, MAX_UPLOAD_MB, REQUESTS_PER_MINUTE};

#[derive(Debug, PartialEq)]
pub enum Rejection {
    TooLarge,
    TooManyFiles,
    RateLimited,
}

pub fn check_upload(file_sizes_mb: &[u32], requests_this_minute: u32) -> Result<(), Rejection> {
    if requests_this_minute >= REQUESTS_PER_MINUTE {
        return Err(Rejection::RateLimited);
    }
    if file_sizes_mb.len() > MAX_FILES_PER_REQUEST {
        return Err(Rejection::TooManyFiles);
    }
    if file_sizes_mb.iter().any(|&s| s > MAX_UPLOAD_MB) {
        return Err(Rejection::TooLarge);
    }
    Ok(())
}
