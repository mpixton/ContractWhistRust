#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
#![allow(clippy::borrowed_box)]

use std::collections::HashMap;

use crate::card::Card;
use crate::player::Player;

pub use crate::game::MormonBridgeGame;

mod card;
mod deck;
mod game;
mod hand;
mod player;
mod rank;
mod suit;
mod trick;

/// Type alias for Player Hand.
type PlayerHands<'a> = HashMap<&'a Box<dyn Player>, Vec<Card>>;

pub const MAX_DISPLAY_WIDTH: usize = 35;
