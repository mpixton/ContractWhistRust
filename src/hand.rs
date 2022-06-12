use std::fmt;

use crate::card::Card;
use crate::player::Player;

pub struct Hand<'a> {
    pub dealer: &'a Box<dyn Player>,
    pub trump_card: Card,
}

impl<'a> Hand<'a> {
    pub fn new(dealer: &'a Box<dyn Player>, trump_card: Card) -> Hand {
        Hand { dealer, trump_card }
    }

    pub fn play_hand(&mut self) {
        // Move the logic from main into here
        todo!()
    }
}

impl<'a> fmt::Display for Hand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dealer)
    }
}
