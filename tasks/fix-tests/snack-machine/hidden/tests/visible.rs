use task::{Event, State, run};

#[test]
fn one_coin_one_snack() {
    assert_eq!(
        run(&[Event::Coin, Event::Select, Event::Done]),
        (State::Idle, 1)
    );
}
