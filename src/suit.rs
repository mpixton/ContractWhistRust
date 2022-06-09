use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
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