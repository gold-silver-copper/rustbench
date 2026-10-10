/// A length of time, in whole seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Span {
    pub secs: u64,
}

impl Span {
    pub fn from_secs(secs: u64) -> Self {
        Span { secs }
    }

    pub fn from_minutes(minutes: u64) -> Self {
        Span { secs: minutes * 60 }
    }

    pub fn add(self, other: Span) -> Span {
        Span {
            secs: self.secs + other.secs,
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.secs / 3600;
        let m = self.secs % 3600 / 60;
        let s = self.secs % 60;
        write!(f, "{h}h {m:02}m {s:02}s")
    }
}
