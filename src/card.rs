//! Todo
//! [] Update documentation

use std::fmt;

use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn get_value(&self) -> (&Rank, &Suit) {
        (&self.rank, &self.suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
