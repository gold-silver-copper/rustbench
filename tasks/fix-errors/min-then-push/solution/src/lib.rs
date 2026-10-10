/// Records a lap time (seconds) and returns the best (lowest) lap so far, including this one.
pub fn add_lap(laps: &mut Vec<u32>, lap: u32) -> u32 {
    let best = *laps.iter().min().unwrap_or(&lap);
    laps.push(lap);
    best.min(lap)
}
