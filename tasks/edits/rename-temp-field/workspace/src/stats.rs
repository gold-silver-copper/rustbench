use crate::reading::Reading;

pub fn mean(readings: &[Reading]) -> Option<f64> {
    if readings.is_empty() {
        return None;
    }
    Some(readings.iter().map(|r| r.temp).sum::<f64>() / readings.len() as f64)
}

pub fn warmest(readings: &[Reading]) -> Option<Reading> {
    readings.iter().copied().reduce(|a, b| if b.temp > a.temp { b } else { a })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_two() {
        let rs = [
            Reading { hour: 1, temp: 10.0, humidity: 50.0 },
            Reading { hour: 2, temp: 20.0, humidity: 50.0 },
        ];
        assert_eq!(mean(&rs), Some(15.0));
        assert_eq!(warmest(&rs).map(|r| r.hour), Some(2));
    }
}
