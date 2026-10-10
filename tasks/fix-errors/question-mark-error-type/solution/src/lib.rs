/// Parses "WIDTHxHEIGHT", e.g. "640x480".
pub fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let (w, h) = s.split_once('x').ok_or(format!("missing 'x' in {s:?}"))?;
    let w: u32 = w.trim().parse().map_err(|e| format!("bad width: {e}"))?;
    let h: u32 = h.trim().parse().map_err(|e| format!("bad height: {e}"))?;
    Ok((w, h))
}
