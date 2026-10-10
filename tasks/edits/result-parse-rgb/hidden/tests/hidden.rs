use task::Rgb;
use task::color::parse_hex;
use task::palette;

#[test]
fn valid_colors_parse() {
    assert_eq!(parse_hex("#ffcc00"), Ok(Rgb { r: 255, g: 204, b: 0 }));
    assert_eq!(parse_hex("#0A0b0C"), Ok(Rgb { r: 10, g: 11, b: 12 }));
}

#[test]
fn bad_colors_are_errors_not_panics() {
    for bad in ["ffcc00", "#ffcc0", "#ffcc000", "#12zz00", "#", "", "#ééé"] {
        let r = std::panic::catch_unwind(|| parse_hex(bad));
        match r {
            Ok(Err(msg)) => assert!(!msg.is_empty(), "empty error for {bad:?}"),
            Ok(Ok(c)) => panic!("{bad:?} parsed as {c:?}"),
            Err(_) => panic!("parse_hex panicked on {bad:?}"),
        }
    }
}

#[test]
fn palette_propagates_errors() {
    let ok = palette::parse_palette("#ffffff, #000000,").map(palette::by_brightness);
    assert_eq!(ok, Ok(vec![Rgb { r: 0, g: 0, b: 0 }, Rgb { r: 255, g: 255, b: 255 }]));
    assert!(palette::parse_palette("#ffffff, nope").is_err());
}
