use crate::util::largest;

/// Daily step counts for one person.
pub struct Steps {
    pub name: String,
    pub days: Vec<i32>,
}

impl Steps {
    pub fn best_day(&self) -> Option<i32> {
        largest(&self.days)
    }
}

/// The highest single-day count across everyone.
pub fn record(people: &[Steps]) -> Option<i32> {
    let bests: Vec<i32> = people.iter().filter_map(Steps::best_day).collect();
    largest(&bests)
}

/// max - min across the days, or 0 with no data.
pub fn spread(days: &[i32]) -> i32 {
    let negated: Vec<i32> = days.iter().map(|d| -d).collect();
    match (largest(days), largest(&negated)) {
        (Some(hi), Some(neg_lo)) => hi + neg_lo,
        _ => 0,
    }
}
