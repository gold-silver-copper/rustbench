use task::tick_tock;

#[test]
fn first_twenty() {
    assert_eq!(
        tick_tock(20),
        "1,2,Tick,4,Tock,Tick,7!,8,Tick,Tock,11,Tick,13,14,TickTock,16,17!,Tick,19,Tock"
    );
}

#[test]
fn sevens_with_words() {
    let all = tick_tock(75);
    let items: Vec<&str> = all.split(',').collect();
    assert_eq!(items.len(), 75);
    assert_eq!(items[26], "Tick!"); // 27
    assert_eq!(items[56], "Tick!"); // 57
    assert_eq!(items[69], "Tock!"); // 70
    assert_eq!(items[70], "71!");
    assert_eq!(items[74], "TickTock!"); // 75
}

#[test]
fn zero_and_one() {
    assert_eq!(tick_tock(0), "");
    assert_eq!(tick_tock(1), "1");
}
