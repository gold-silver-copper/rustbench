use crate::geom::Rect;

/// Split a rectangle into `n` equal columns (the last one takes the remainder).
pub fn columns(r: Rect, n: u32) -> Vec<Rect> {
    if n == 0 {
        return Vec::new();
    }
    let w = r.width / n;
    (0..n)
        .map(|i| {
            let extra = if i == n - 1 { r.width - w * n } else { 0 };
            Rect::new(r.origin.x + (w * i) as i32, r.origin.y, w + extra, r.height)
        })
        .collect()
}
