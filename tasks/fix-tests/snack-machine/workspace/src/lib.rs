//! A snack vending machine that takes one coin per snack.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    HasCoin,
    Dispensing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Coin,
    Select,
    Refund,
    Done,
}

/// The state after `event`. Events that make no sense in a state leave it unchanged.
pub fn step(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::Coin) => State::HasCoin,
        (State::HasCoin, Event::Select) => State::Dispensing,
        (State::HasCoin, Event::Refund) => State::Idle,
        (State::Dispensing, Event::Done) => State::HasCoin,
        (s, _) => s,
    }
}

/// Runs events from `Idle`; returns the final state and how many snacks came out.
pub fn run(events: &[Event]) -> (State, u32) {
    let mut state = State::Idle;
    let mut snacks = 0;
    for &e in events {
        let next = step(state, e);
        if state == State::Dispensing && next != State::Dispensing {
            snacks += 1;
        }
        state = next;
    }
    (state, snacks)
}
