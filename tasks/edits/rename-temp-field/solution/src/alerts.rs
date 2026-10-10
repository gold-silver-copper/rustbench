use crate::reading::Reading;

#[derive(Debug, PartialEq, Eq)]
pub enum Alert {
    Frost { hour: u8 },
    Heat { hour: u8 },
}

/// Frost below 0 °C, heat at or above 35 °C.
pub fn alerts(readings: &[Reading]) -> Vec<Alert> {
    readings
        .iter()
        .filter_map(|r| {
            if r.celsius < 0.0 {
                Some(Alert::Frost { hour: r.hour })
            } else if r.celsius >= 35.0 {
                Some(Alert::Heat { hour: r.hour })
            } else {
                None
            }
        })
        .collect()
}
