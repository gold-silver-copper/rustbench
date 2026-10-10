use std::collections::VecDeque;

/// A first-in-first-out queue of `i32` with a fixed capacity. When it is
/// full, pushing a new value drops the oldest one.
///
/// ```
/// let mut r = task::RingBuffer::new(2);
/// assert_eq!(r.push(1), None);
/// assert_eq!(r.push(2), None);
/// assert_eq!(r.push(3), Some(1));
/// assert_eq!(r.to_vec(), vec![2, 3]);
/// ```
pub struct RingBuffer {
    capacity: usize,
    items: VecDeque<i32>,
}

impl RingBuffer {
    /// An empty buffer that holds at most `capacity` values.
    pub fn new(capacity: usize) -> Self {
        Self { capacity, items: VecDeque::with_capacity(capacity) }
    }

    /// Appends `value` at the back. If the buffer was already full, the
    /// oldest value is removed and returned; otherwise returns `None`. With
    /// capacity 0 nothing is stored and `value` itself is returned.
    pub fn push(&mut self, value: i32) -> Option<i32> {
        if self.capacity == 0 {
            return Some(value);
        }
        let dropped = if self.items.len() >= self.capacity { self.items.pop_front() } else { None };
        self.items.push_back(value);
        dropped
    }

    /// Removes and returns the oldest value, or `None` if the buffer is empty.
    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    /// The stored values, oldest first.
    pub fn to_vec(&self) -> Vec<i32> {
        self.items.iter().copied().collect()
    }

    /// The number of stored values.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether no values are stored.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Whether the buffer holds `capacity` values. A capacity-0 buffer is
    /// always full.
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }
}
