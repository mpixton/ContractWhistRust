//! A playing card composed of a rank and suit for playing card games.
//!
//! The [Rank] attribute holds the value that determines where the Card sorts
//! among other Cards in its [Suit]. The [Suit] holds the value that
//! determines which grouping of Cards the Card belongs to.

use std::fmt;

use crate::rank::Rank;
use crate::suit::Suit;

/// A Card, representing a traditional Card from a French deck of playing cards.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Creates a new Card with the given [Rank] and [Suit].
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    /// Returns a tuple containing the ([Rank], [Suit]) of the Card.
    /// # Examples
    ///
    /// ```
    /// # use mormon_bridge::rank::Rank;
    /// # use mormon_bridge::suit::Suit;
    /// # use mormon_bridge::card::Card;
    ///
    /// let card = Card::new(Rank::Ace, Suit::Hearts);
    /// assert_eq!(card.get_value(), (&Rank::Ace, &Suit::Hearts));
    /// ```
    pub fn get_value(&self) -> (&Rank, &Suit) {
        (&self.rank, &self.suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
