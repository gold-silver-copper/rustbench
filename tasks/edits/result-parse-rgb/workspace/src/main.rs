use task::palette;

fn main() {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "#ffcc00, #102030, #7f7f7f".to_string());
    let colors = palette::parse_palette(&input);
    for c in palette::by_brightness(colors) {
        println!("{} luma {}", c.to_hex(), c.luma());
    }
}
