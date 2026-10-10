#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Perceived brightness, 0..=255.
    pub fn luma(self) -> u8 {
        let l = 299 * u32::from(self.r) + 587 * u32::from(self.g) + 114 * u32::from(self.b);
        (l / 1000) as u8
    }

    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

/// Parse "#rrggbb" (the '#' is required, digits are case-insensitive).
pub fn parse_hex(s: &str) -> Result<Rgb, String> {
    let digits = s
        .strip_prefix('#')
        .ok_or_else(|| format!("color must start with '#': {s}"))?;
    if digits.len() != 6 || !digits.is_ascii() {
        return Err(format!("color must have six hex digits: {s}"));
    }
    let channel = |i: usize| {
        digits
            .get(i..i + 2)
            .and_then(|d| u8::from_str_radix(d, 16).ok())
            .ok_or_else(|| format!("invalid hex digit in {s}"))
    };
    Ok(Rgb {
        r: channel(0)?,
        g: channel(2)?,
        b: channel(4)?,
    })
}
