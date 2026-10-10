use crate::link::FlakyLink;

#[derive(Debug, PartialEq)]
pub struct FetchError {
    pub attempts: u32,
    pub last: String,
}

impl FetchError {
    pub fn message(&self) -> String {
        format!("gave up after {}/3 attempts: {}", self.attempts, self.last)
    }
}

/// Try the request up to 3 times.
pub fn fetch_with_retry(link: &mut FlakyLink, path: &str) -> Result<String, FetchError> {
    let mut last = String::new();
    for attempt in 1..=3 {
        match link.request(path) {
            Ok(body) => return Ok(body),
            Err(e) => {
                last = e;
                if attempt == 3 {
                    break;
                }
            }
        }
    }
    Err(FetchError { attempts: 3, last })
}
