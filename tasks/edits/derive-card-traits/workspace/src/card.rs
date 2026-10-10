pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    pub fn symbol(&self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }
}

impl Rank {
    /// Blackjack-style points (ace counts 11).
    pub fn points(&self) -> u8 {
        match self {
            Rank::Number(n) => *n,
            Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Rank::Number(n) => n.to_string(),
            Rank::Jack => "J".into(),
            Rank::Queen => "Q".into(),
            Rank::King => "K".into(),
            Rank::Ace => "A".into(),
        }
    }
}

impl Card {
    pub fn short(&self) -> String {
        format!("{}{}", self.rank.label(), self.suit.symbol())
    }
}
