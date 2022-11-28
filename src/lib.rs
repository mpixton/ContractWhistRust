#![allow(clippy::borrowed_box)]

use std::collections::HashMap;

use crate::card::Card;
use crate::player::Player;

pub use crate::game::ContractWhistGame;

pub(crate) mod card;
pub(crate) mod deck;
pub(crate) mod game;
pub(crate) mod hand;
pub(crate) mod player;
pub(crate) mod rank;
pub(crate) mod suit;
pub(crate) mod trick;

/// Type alias for Player Hand.
type PlayerHands<'a> = HashMap<&'a Box<dyn Player>, Vec<Card>>;

pub const MAX_DISPLAY_WIDTH: usize = 35;
