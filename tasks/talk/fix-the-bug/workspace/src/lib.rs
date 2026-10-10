/// Celsius to Fahrenheit.
pub fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Fahrenheit to Celsius.
pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Celsius to Kelvin; `None` below absolute zero.
pub fn c_to_k(c: f64) -> Option<f64> {
    let k = c + 273.15;
    (k >= 0.0).then_some(k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boiling_point() {
        assert!((c_to_f(100.0) - 212.0).abs() < 1e-9);
        assert!((f_to_c(212.0) - 100.0).abs() < 1e-9);
    }

    #[test]
    fn round_trip() {
        for c in [-40.0, 0.0, 37.0] {
            assert!((f_to_c(c_to_f(c)) - c).abs() < 1e-9);
        }
    }

    #[test]
    fn below_absolute_zero() {
        assert_eq!(c_to_k(-300.0), None);
    }
}
