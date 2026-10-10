use task::{Event, State, run, step};

#[test]
fn no_free_snacks() {
    use Event::*;
    assert_eq!(run(&[Coin, Select, Done, Select, Done]), (State::Idle, 1));
    assert_eq!(
        run(&[Coin, Select, Done, Coin, Select, Done]),
        (State::Idle, 2)
    );
}

#[test]
fn other_transitions() {
    use Event::*;
    assert_eq!(run(&[Coin, Refund]), (State::Idle, 0));
    assert_eq!(run(&[Coin, Coin, Select]), (State::Dispensing, 0));
    assert_eq!(step(State::Idle, Select), State::Idle);
    assert_eq!(step(State::Dispensing, Done), State::Idle);
}
