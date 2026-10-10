use task::{Settings, printer_for};

fn main() {
    let settings = Settings {
        width: 40,
        ..Settings::default()
    }
    .narrow_terminal_fallback();
    let printer = printer_for(&settings);
    print!("{}", printer.print(&[("coffee", 350), ("bagel", 275)]));
}
