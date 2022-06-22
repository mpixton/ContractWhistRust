//! Deck of playing cards with associated funationality.
//!
//! At its core, a wrapper around a Vec of [Card]s. Dealing a [Card] removes one
//! from the top of the Deck and returns it. Shuffling the Deck randomizes the
//! order. Building a [Deck] is done by calling configuration methods after a
//! `Deck::new()` call.
//!
//! # Examples
//! ```
//! // Create a new full 52 card deck and shuffle it 7 times
//! let deck = Deck::new().deck_type(DeckType::Full).shuffle(Some(7));
//! assert_eq!(deck.len(), 52);
//! ```
//!
//! # Todo
//! [] Update documentation
//! [] TypeState DeckBuilder to prevent excessive shuffling

use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;

/// A wrapper around a Vec of [Card]s.
///
/// Provides dealing and shuffling funtionality to randomize Card order and
/// return a [Card].
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Provides a [DeckBuilder] for Deck configuration.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DeckBuilder {
        DeckBuilder { cards: Vec::new() }
    }

    /// Prints all the Cards in the Deck in order for debugging purposes.
    pub fn debug_deck(&self) {
        for card in self.cards.iter() {
            println!("{}", card)
        }
    }

    /// Returns the number of Cards left in the Deck.
    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }

    /// Returns the top Card of the Deck.
    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    /// Returns a static card intended to be used for debugging.
    pub fn debug_trump(&mut self) -> Card {
        *self
            .cards
            .iter()
            .find(|e: &&Card| {
                let (rank, suit) = e.get_value();
                rank == &Rank::Two && suit == &Suit::Hearts
            })
            .unwrap()
    }

    /// Returns a a series of static cards, intended to be used for debugging.
    pub fn debug_deal(&mut self, index: usize) -> Card {
        let debug_suit = &Suit::VALUES[index % 4];
        let debug_rank = &Rank::VALUES[index % 7];

        let find_card = |e: &&Card| {
            let (rank, suit) = e.get_value();
            rank == debug_rank && suit == debug_suit
        };
        *self.cards.iter().find(find_card).unwrap()
    }
}

/// Types of Deck that may be created.
pub enum DeckType {
    Full,
}

/// A throwaway object used to configure a [Deck].
///
/// # Examples
/// ```
/// // create the DeckBuilder
/// let deck_builder = Deck::new();
/// // COnfigure the Deck
/// let deck = deck_builder.deck_type(DeckType::Full).end();
/// assert_eq!(deck.total_cards(), 52);
/// ```
pub struct DeckBuilder {
    cards: Vec<Card>,
}

impl DeckBuilder {
    /// Set the type of Deck, which determines the amount, rank, and suit of cards.
    pub fn deck_type(self, deck_type: DeckType) -> DeckBuilder {
        let total_cards = match deck_type {
            DeckType::Full => 52,
        };

        let mut cards: Vec<Card> = Vec::with_capacity(total_cards);

        for suit in Suit::VALUES.iter() {
            for rank in Rank::VALUES.iter() {
                cards.push(Card::new(*rank, *suit))
            }
        }

        DeckBuilder { cards }
    }

    /// Shuffles the Deck 7 times.
    pub fn default_shuffle(mut self) -> DeckBuilder {
        let mut shuffling = || self.cards.shuffle(&mut thread_rng());
        {
            for i in 0..7 {
                shuffling();
            }
        }

        DeckBuilder { cards: self.cards }
    }

    /// Shuffles the Deck anywhere from 1 to 10 times.
    pub fn shuffle(mut self, shuffles: Option<i8>) -> DeckBuilder {
        let mut shuffling = || self.cards.shuffle(&mut thread_rng());

        match shuffles {
            Some(iters) if iters > 1 && iters < 10 => {
                for i in 1..=iters {
                    shuffling();
                }
            }
            _ => shuffling(),
        }

        DeckBuilder { cards: self.cards }
    }

    /// Finishes configuration of the Deck and returns a new Deck.
    pub fn end(self) -> Deck {
        let cards = self.cards;

        Deck { cards }
    }
}
