use task::Reading;
use task::alerts::{Alert, alerts};

#[test]
fn celsius_field() {
    let r = Reading { hour: 7, celsius: 100.0, humidity: 10.0 };
    assert_eq!(r.fahrenheit(), 212.0);
    let p = Reading::parse("9, -4.5, 60").unwrap();
    assert_eq!(p.celsius, -4.5);
    assert_eq!(p.hour, 9);
}

#[test]
fn stats_and_alerts_unchanged() {
    let rs = [
        Reading { hour: 0, celsius: -2.0, humidity: 70.0 },
        Reading { hour: 13, celsius: 40.0, humidity: 15.0 },
        Reading { hour: 18, celsius: 22.0, humidity: 40.0 },
    ];
    assert_eq!(task::stats::mean(&rs), Some(20.0));
    assert_eq!(task::stats::warmest(&rs).map(|r| r.hour), Some(13));
    assert_eq!(alerts(&rs), vec![Alert::Frost { hour: 0 }, Alert::Heat { hour: 13 }]);
}

