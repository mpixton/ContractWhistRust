use crate::card::Card;
use crate::rank::Rank;
use crate::suit::Suit;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    #[warn(clippy::new_ret_no_self)]
    pub fn new() -> DeckBuilder {
        DeckBuilder { cards: Vec::new() }
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

pub enum DeckType {
    Full,
}

pub struct DeckBuilder {
    cards: Vec<Card>,
}

impl DeckBuilder {
    pub fn deck_type(&mut self, deck_type: DeckType) -> DeckBuilder {
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

    pub fn default_shuffle(mut self) -> Deck {
        let mut shuffling = || self.cards.shuffle(&mut thread_rng());
        {
            for i in 0..7 {
                println!("Shuffling... {}", { i });
                shuffling();
            }
        }

        Deck { cards: self.cards }
    }

    pub fn shuffle(mut self, shuffles: Option<i8>) -> Deck {
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

        Deck { cards: self.cards }
    }
}
