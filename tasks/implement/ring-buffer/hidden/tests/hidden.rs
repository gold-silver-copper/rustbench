use task::RingBuffer;

#[test]
fn fifo_with_overwrite() {
    let mut r = RingBuffer::new(3);
    assert!(r.is_empty());
    assert!(!r.is_full());
    for v in 1..=3 {
        assert_eq!(r.push(v), None);
    }
    assert!(r.is_full());
    assert_eq!(r.push(4), Some(1));
    assert_eq!(r.push(5), Some(2));
    assert_eq!(r.to_vec(), vec![3, 4, 5]);
    assert_eq!(r.pop(), Some(3));
    assert_eq!(r.len(), 2);
    assert!(!r.is_full());
    assert_eq!(r.push(6), None);
    assert_eq!(r.to_vec(), vec![4, 5, 6]);
}

#[test]
fn drains() {
    let mut r = RingBuffer::new(2);
    r.push(10);
    r.push(20);
    r.push(30);
    assert_eq!(r.pop(), Some(20));
    assert_eq!(r.pop(), Some(30));
    assert_eq!(r.pop(), None);
    assert!(r.is_empty());
    assert_eq!(r.to_vec(), Vec::<i32>::new());
}

#[test]
fn zero_capacity() {
    let mut r = RingBuffer::new(0);
    assert!(r.is_full());
    assert!(r.is_empty());
    assert_eq!(r.push(7), Some(7));
    assert_eq!(r.len(), 0);
    assert_eq!(r.pop(), None);
}
