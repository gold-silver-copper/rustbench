use crate::MAX_RETRIES;
use crate::link::FlakyLink;

#[derive(Debug, PartialEq)]
pub struct FetchError {
    pub attempts: u32,
    pub last: String,
}

impl FetchError {
    pub fn message(&self) -> String {
        format!(
            "gave up after {}/{MAX_RETRIES} attempts: {}",
            self.attempts, self.last
        )
    }
}

/// Try the request up to `MAX_RETRIES` times.
pub fn fetch_with_retry(link: &mut FlakyLink, path: &str) -> Result<String, FetchError> {
    let mut last = String::new();
    for attempt in 1..=MAX_RETRIES {
        match link.request(path) {
            Ok(body) => return Ok(body),
            Err(e) => {
                last = e;
                if attempt == MAX_RETRIES {
                    break;
                }
            }
        }
    }
    Err(FetchError {
        attempts: MAX_RETRIES,
        last,
    })
}
