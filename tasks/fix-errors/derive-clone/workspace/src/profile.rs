use crate::settings::Settings;

/// A copy of the settings with sound muted; the original is left alone.
pub fn muted(s: &Settings) -> Settings {
    let mut copy = s.clone();
    copy.muted = true;
    copy
}
