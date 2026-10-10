pub struct Parcel {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

/// Total volume of all parcels.
pub fn total_volume(parcels: &[Parcel]) -> f64 {
    parcels.iter().map(|p| p.width * p.hieght * p.depth).sum()
}
