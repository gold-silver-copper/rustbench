use task::advice::{advice, good_for_cycling};
use task::icon::icon;
use task::{Weather, forecast};

#[test]
fn fog_is_handled() {
    assert_eq!(Weather::from_code("FG"), Some(Weather::Fog));
    assert_eq!(advice(Weather::Fog), "drive carefully");
    assert_eq!(icon(Weather::Fog), "~");
    assert_eq!(forecast("FG"), "~ drive carefully");
    let _ = good_for_cycling(Weather::Fog);
}

#[test]
fn others_unchanged() {
    assert_eq!(forecast("SU"), "* sunglasses");
    assert_eq!(forecast("RA"), "/ umbrella");
    assert_eq!(forecast("SN"), "# warm boots");
    assert_eq!(forecast("XX"), "? unknown code XX");
    assert!(good_for_cycling(Weather::Sunny));
    assert!(!good_for_cycling(Weather::Snowy));
}
