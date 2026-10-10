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
///
/// Panics if the input is not a valid color.
pub fn parse_hex(s: &str) -> Rgb {
    let digits = s.strip_prefix('#').expect("color must start with '#'");
    if digits.len() != 6 || !digits.is_ascii() {
        panic!("color must have six hex digits: {s}");
    }
    let channel = |i: usize| {
        u8::from_str_radix(&digits[i..i + 2], 16).expect("invalid hex digit")
    };
    Rgb {
        r: channel(0),
        g: channel(2),
        b: channel(4),
    }
}
