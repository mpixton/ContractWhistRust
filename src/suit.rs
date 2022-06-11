use std::fmt;

/// Enum of all Suits in a French deck of cards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
    /// All Suits foriterating over while creating a deck.
    pub const VALUES: [Suit; 4] = [Self::Hearts, Self::Clubs, Self::Diamonds, Self::Spades];
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}
