#[derive(Debug, Clone, PartialEq)]
pub enum SendError {
    NotTried,
    Timeout,
    Rejected(String),
}

impl SendError {
    fn is_retryable(&self) -> bool {
        matches!(self, SendError::Timeout)
    }
}

pub trait Transport {
    fn send(&mut self, msg: &str) -> Result<(), SendError>;
}

pub struct Client<T> {
    transport: T,
    max_retries: u32,
}

impl<T: Transport> Client<T> {
    pub fn new(transport: T) -> Self {
        Client {
            transport,
            max_retries: 3,
        }
    }

    /// Sends `msg`, retrying on timeouts.
    pub fn deliver(&mut self, msg: &str) -> Result<(), SendError> {
        let mut last = SendError::NotTried;
        for _attempt in 0..=self.max_retries {
            match self.transport.send(msg) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    let retry = e.is_retryable();
                    last = e;
                    if !retry {
                        break;
                    }
                }
            }
        }
        Err(last)
    }
}
