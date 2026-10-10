use crate::card::{Card, Rank, Suit};

/// A fresh 52-card deck, suit by suit.
pub fn full_deck() -> Vec<Card> {
    let mut deck = Vec::with_capacity(52);
    for suit in Suit::ALL {
        let ranks = (2..=10)
            .map(Rank::Number)
            .chain([Rank::Jack, Rank::Queen, Rank::King, Rank::Ace]);
        for rank in ranks {
            let suit = match suit {
                Suit::Clubs => Suit::Clubs,
                Suit::Diamonds => Suit::Diamonds,
                Suit::Hearts => Suit::Hearts,
                Suit::Spades => Suit::Spades,
            };
            deck.push(Card { rank, suit });
        }
    }
    deck
}

pub fn hand_points(hand: &[Card]) -> u32 {
    hand.iter().map(|c| u32::from(c.rank.points())).sum()
}
