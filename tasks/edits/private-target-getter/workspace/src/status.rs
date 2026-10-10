use crate::Thermostat;

/// One status line, e.g. `18.2°C -> 21.5°C (heating)`.
pub fn line(t: &Thermostat) -> String {
    let state = if t.heating() { "heating" } else { "idle" };
    format!("{:.1}°C -> {:.1}°C ({state})", t.current(), t.target)
}
