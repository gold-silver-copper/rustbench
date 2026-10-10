/// The scores at or above the pass mark, in their original order.
pub fn passing(scores: &[u32], pass_mark: u32) -> Vec<u32> {
    let kept: Vec<u32> = scores.iter().copied().filter(|&s| s >= pass_mark).collect();
    kept
}
