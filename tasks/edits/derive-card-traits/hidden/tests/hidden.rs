use task::deck::{full_deck, hand_points};
use task::{Card, Rank, Suit};

fn copy_of<T: Copy>(x: &T) -> T {
    *x
}

#[test]
fn debug_format() {
    let c = Card { rank: Rank::Number(7), suit: Suit::Hearts };
    assert_eq!(format!("{c:?}"), "Card { rank: Number(7), suit: Hearts }");
    assert_eq!(format!("{:?}", Rank::Queen), "Queen");
    assert_eq!(format!("{:?}", Suit::Spades), "Spades");
}

#[test]
fn equality_and_copy() {
    let deck = full_deck();
    let first = deck[0];
    let again = copy_of(&deck[0]);
    assert_eq!(first, again);
    assert_eq!(first, Card { rank: Rank::Number(2), suit: Suit::Clubs });
    assert_ne!(deck[0], deck[1]);
    assert_eq!(deck[51].clone(), Card { rank: Rank::Ace, suit: Suit::Spades });
    assert_eq!(copy_of(&Suit::Hearts), Suit::Hearts);
    assert_eq!(copy_of(&Rank::King), Rank::King);
}

fn assert_eq_trait<T: Eq>() {}

#[test]
fn eq_and_behaviour() {
    assert_eq_trait::<Card>();
    assert_eq_trait::<Rank>();
    assert_eq_trait::<Suit>();
    let deck = full_deck();
    assert_eq!(deck.len(), 52);
    assert_eq!(hand_points(&deck[8..10]), 20);
    assert_eq!(deck[12].short(), "AC");
}
