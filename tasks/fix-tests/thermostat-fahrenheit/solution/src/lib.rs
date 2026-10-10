//! Unit conversions for a thermostat display (whole degrees).

/// Celsius to Fahrenheit, rounded toward zero.
pub fn c_to_f(c: i32) -> i32 {
    c * 9 / 5 + 32
}

/// Fahrenheit to Celsius, rounded toward zero.
pub fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}
