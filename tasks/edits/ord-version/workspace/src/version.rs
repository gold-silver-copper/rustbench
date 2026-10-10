use std::fmt;

/// A release version like `1.4.2-beta`.
#[derive(Debug, Clone)]
pub struct Version {
    /// Free-form display name, e.g. "beta" or "lts". Empty for plain releases.
    pub label: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Version {
            label: String::new(),
            major,
            minor,
            patch,
        }
    }

    pub fn labeled(major: u32, minor: u32, patch: u32, label: &str) -> Self {
        Version {
            label: label.to_string(),
            ..Version::new(major, minor, patch)
        }
    }

    /// Parse "1.2.3" or "1.2.3-label".
    pub fn parse(s: &str) -> Option<Self> {
        let (nums, label) = match s.split_once('-') {
            Some((n, l)) => (n, l),
            None => (s, ""),
        };
        let mut parts = nums.split('.').map(|p| p.parse::<u32>().ok());
        let major = parts.next()??;
        let minor = parts.next()??;
        let patch = parts.next()??;
        if parts.next().is_some() {
            return None;
        }
        Some(Version::labeled(major, minor, patch, label))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.label.is_empty() {
            write!(f, "-{}", self.label)?;
        }
        Ok(())
    }
}
