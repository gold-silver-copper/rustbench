use std::thread;

/// Sums each chunk of the numbers on its own thread; returns the per-chunk totals.
pub fn chunk_sums(numbers: Vec<u64>, chunk: usize) -> Vec<u64> {
    let mut handles = Vec::new();
    for part in numbers.chunks(chunk.max(1)) {
        let part = part.to_vec();
        handles.push(thread::spawn(|| part.iter().sum::<u64>()));
    }
    handles.into_iter().map(|h| h.join().unwrap_or(0)).collect()
}
