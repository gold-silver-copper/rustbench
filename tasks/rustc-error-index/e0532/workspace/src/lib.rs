enum State {
    Succeeded,
    Failed(String),
}

fn print_on_failure(state: &State) {
    match *state {
        State::Failed => println!("Failed"),
        _ => ()
    }
}
