use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(deck_type: DeckType, pre_shuffled: bool) -> Deck {
        let total_cards = match deck_type {
            DeckType::Full => 52,
        };

        let mut cards: Vec<Card> = Vec::with_capacity(total_cards);

        for suit in Suit::VALUES.iter() {
            for rank in Rank::VALUES.iter() {
                cards.push(Card::new(*rank, *suit))
            }
        }

        if pre_shuffled {
            cards.shuffle(&mut thread_rng());
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self, shuffles: Option<i8>) -> &mut Deck {
        let mut shuffling = || self.cards.shuffle(&mut thread_rng());

        match shuffles {
            Some(iters) if iters > 1 && iters < 10 => {
                for i in 1..=iters {
                    println!("Shuffling... {}", { i });
                    shuffling();
                }
            }
            _ => shuffling(),
        }
        self
    }

    pub fn debug_deck(&self) {
        for card in self.cards.iter() {
            println!("{}", card)
        }
    }

    pub fn total_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

pub enum DeckType {
    Full,
}
