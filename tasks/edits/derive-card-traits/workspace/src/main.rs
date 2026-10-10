use task::deck::{full_deck, hand_points};

fn main() {
    let deck = full_deck();
    let hand = &deck[..2];
    let names: Vec<String> = hand.iter().map(|c| c.short()).collect();
    println!("{} = {}", names.join(" "), hand_points(hand));
}
