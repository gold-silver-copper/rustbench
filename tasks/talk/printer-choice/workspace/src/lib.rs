pub trait Printer {
    fn print(&self, lines: &[(&str, u32)]) -> String;
}

pub struct PlainPrinter;

pub struct FancyPrinter {
    pub width: usize,
}

impl Printer for PlainPrinter {
    fn print(&self, lines: &[(&str, u32)]) -> String {
        lines
            .iter()
            .map(|(name, cents)| format!("{name} {cents}\n"))
            .collect()
    }
}

impl Printer for FancyPrinter {
    fn print(&self, lines: &[(&str, u32)]) -> String {
        let rule = "=".repeat(self.width);
        let mut out = format!("{rule}\n");
        for (name, cents) in lines {
            let price = format!("{}.{:02}", cents / 100, cents % 100);
            let pad = self.width.saturating_sub(name.len() + price.len());
            out += &format!("{name}{}{price}\n", " ".repeat(pad));
        }
        out + &rule + "\n"
    }
}

pub struct Settings {
    pub plain: bool,
    pub width: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            plain: false,
            width: 32,
        }
    }
}

impl Settings {
    /// Fancy receipts need room for the price column.
    pub fn narrow_terminal_fallback(mut self) -> Self {
        if self.width < 48 {
            self.plain = true;
        }
        self
    }
}

pub fn printer_for(s: &Settings) -> Box<dyn Printer> {
    if s.plain {
        Box::new(PlainPrinter)
    } else {
        Box::new(FancyPrinter { width: s.width })
    }
}
