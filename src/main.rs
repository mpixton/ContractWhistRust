#![allow(dead_code)]
#![allow(unused_variables, unused_imports)]
#![allow(clippy::borrowed_box)]

mod card;
mod deck;
mod game;
mod hand;
mod player;
mod rank;
mod suit;
mod trick;

use core::num;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::card::Card;
use crate::deck::{Deck, DeckType};
use crate::game::MormonBridgeGame;
use crate::hand::Hand;
use crate::player::{HumanPlayer, Player};
use crate::trick::{Playing, Trick};

/// Type alias for Player Hand.
type PlayerHands<'a> = HashMap<&'a Box<dyn Player>, Vec<Card>>;

fn main() {
    let game = MormonBridgeGame::play(true);
}

pub const MAX_DISPLAY_WIDTH: usize = 35;

// // let mut cards_played: Vec<(Card, &String)> = Vec::with_capacity(4);

// // for player in players.iter() {
// //     cards_played.push((deck.deal(), player));
// // }

// // let mut trick = Trick::new(cards_played, deck.deal());

// // trick.show_results();

// let mut deck = Deck::new().deck_type(DeckType::Full).default_shuffle();

// let game = MormonBridgeGame::new();
// game.display_players();

// let mut hand = Hand::new(&game.players[0], deck.debug_trump(), game.players.len());

// println!("{}", hand);

// hand.deal_players_in(deck, &game.players, &MormonBridgeGame::TRICKS_PER_HAND[1]);

// let trick: Trick<Playing> = Trick::new(&deck.debug_trump(), &game.players);

// // let mut player_cards: HashMap<&Box<dyn Player>, Vec<Card>> = HashMap::with_capacity(7);

// // for player in game.players.iter() {
// //     player_cards.insert(player, Vec::with_capacity(2));
// // }

// // let double_players = game.players.iter().chain(game.players.iter());

// // for (index, player) in double_players.enumerate() {
// //     match player_cards.entry(player) {
// //         Entry::Vacant(e) => {
// //             let mut cards = Vec::with_capacity(2);
// //             cards.push(deck.debug_deal(index));
// //             e.insert(cards);
// //         }
// //         Entry::Occupied(mut e) => {
// //             e.get_mut().push(deck.debug_deal(index));
// //         }
// //     };
// // }

// // println!("{:#?}", player_cards);

// // let double_players = game.players.iter().chain(game.players.iter());

// // for player in double_players {
// //     println!("{}", player_cards.get_mut(player).unwrap().pop().unwrap());
// // }

// // let mut num_deal = 0;
// // loop {
// //     if num_deal == 14 {
// //         break;
// //     }
// //     game.players[num_deal % 7].add_card_to_hand(deck.debug_deal(num_deal));
// //     num_deal += 1;
// // }

// // for player in game.players.iter() {
// //     println!("{}", player);
// //     player.display_hand();
// // }

// // // vec to store all cards played
// // // In practice, after each trick, the cards played will be used to create a trick obj
// // let mut cards_played: Vec<(Card, &Box<dyn Player>)> = Vec::with_capacity(14);

// // // Simulate playing cards in a trick
// // for (index, player) in game.players.iter_mut().enumerate() {
// //     // If the first card of the trick, no led card
// //     if index % 7 == 0 {
// //         cards_played.push((player.play_card(&hand.trump_card, None), player))
// //     } else {
// //         let led_card = cards_played.get(index);
// //         cards_played.push((player.play_card(&hand.trump_card, led_card), player))
// //     }
// // }

// // for (index, player) in game.players.iter().enumerate() {
// //     println!("{}", player);
// //     player.display_hand();
// //     // if index == 0 || index == 7 {
// //     //     player.play_card(&hand.trump_card, None).unwrap();
// //     // } else {
// //     //     if (index < 6)
// //     //     player.play_card(&hand.trump_card, player.).unwrap();
// //     // }
// // }
