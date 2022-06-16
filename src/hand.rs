//! A play of a Hand of Mormon Bridge, consisting of dealing players in, asking for bids, playing the tricks, and storing the resulting scores.

// Possible states for the hand:
// Start: the hand is dealing all players in and setting trump
// Gathering Bids: asking each player for their bid for the hand
// Playing: playing the hand by playing a series of tricks
// Scoring: players are being scored on the hand
// Finished: the hand is over and players scored

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::card::Card;
use crate::deck::{Deck, DeckType};
use crate::player::Player;
use crate::trick::Trick;
use crate::trick::{self, TrickState};
use crate::PlayerHands;

// https://cliffle.com/blog/rust-typestate

pub struct NewHand<'a, T: HandState> {
    players: &'a Vec<Box<dyn Player>>,
    extra: T,
}

pub struct Start {
    deck: Deck,
    num_tricks: usize,
}
pub struct Bidding<'a> {
    trump: Card,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
    num_tricks: usize,
}
pub struct Playing<'a> {
    bids: HashMap<&'a Box<dyn Player>, usize>,
    trump: Card,
    num_tricks: usize,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
}
pub struct Scoring<'a> {
    bids: HashMap<&'a Box<dyn Player>, usize>,
    tricks_won: HashMap<&'a Box<dyn Player>, usize>,
}
pub struct Finished<'a> {
    points: HashMap<&'a Box<dyn Player>, usize>,
}

pub trait HandState {}
impl HandState for Start {}
impl<'a> HandState for Bidding<'a> {}
impl<'a> HandState for Playing<'a> {}
impl<'a> HandState for Scoring<'a> {}
impl<'a> HandState for Finished<'a> {}

impl<'a, T: HandState> NewHand<'a, T> {
    pub fn new(players: &'a Vec<Box<dyn Player>>, num_tricks: usize) -> NewHand<'a, Start> {
        let deck = Deck::new().deck_type(DeckType::Full).shuffle(Some(7));
        NewHand {
            players,
            extra: Start { deck, num_tricks },
        }
    }
}

impl<'a> NewHand<'a, Start> {
    pub fn deal_players_in(mut self) -> NewHand<'a, Bidding<'a>> {
        let players = self.players;
        let num_tricks = self.extra.num_tricks;
        let mut index: usize = 0;

        let trump = self.extra.deck.debug_trump();
        let mut player_hands: PlayerHands = HashMap::with_capacity(players.len());

        while index < self.extra.num_tricks {
            for (p_index, player) in players.iter().enumerate() {
                match player_hands.entry(player) {
                    Entry::Vacant(e) => {
                        let mut cards = Vec::with_capacity(2);
                        cards.push(self.extra.deck.debug_deal(index + p_index));
                        e.insert(cards);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut()
                            .push(self.extra.deck.debug_deal(index + p_index));
                    }
                }
            }
            index += 1;
        }

        NewHand {
            players,
            extra: Bidding {
                player_hands,
                trump,
                num_tricks,
            },
        }
    }
}

impl<'a> NewHand<'a, Bidding<'a>> {
    pub fn get_player_bids(self) -> NewHand<'a, Playing<'a>> {
        // bind nested fields to variable names for ease of use
        let player_hands: PlayerHands = self.extra.player_hands;
        let trump = self.extra.trump;
        let players = self.players;
        let num_tricks = self.extra.num_tricks;

        let mut bids: HashMap<&'a Box<dyn Player>, usize> = HashMap::with_capacity(players.len());

        for player in players.iter() {
            let cards = player_hands.get(player).unwrap();
            let bid = player.get_player_bid(&trump, &num_tricks, cards);
            bids.insert(player, bid);
        }

        for (player, bid) in bids.iter() {
            println!("{} bid {}", player, bid);
        }

        NewHand {
            players,
            extra: Playing {
                bids,
                trump,
                player_hands,
                num_tricks,
            },
        }
    }
}

impl<'a> NewHand<'a, Playing<'a>> {
    pub fn play_tricks(self) -> NewHand<'a, Scoring<'a>> {
        let mut player_hands = self.extra.player_hands;
        let trump = self.extra.trump;
        let players = self.players;
        let bids = self.extra.bids;
        let num_tricks = self.extra.num_tricks;

        let mut tricks_won: HashMap<&Box<dyn Player>, usize> =
            HashMap::with_capacity(players.len());

        let mut index = 0;

        while index < num_tricks {
            println!("Playing trick: {}", index);
            let player_hands = &mut player_hands;
            let winner = Trick::<trick::Start>::new(&trump, players, player_hands)
                .play_trick()
                .determine_winner()
                .get_winner();
            match tricks_won.entry(winner) {
                Entry::Vacant(_) => 1,
                Entry::Occupied(mut won) => {
                    let new_won = won.get_mut().to_owned() + 1;
                    new_won
                }
            };
            index += 1;
        }

        NewHand {
            players,
            extra: Scoring { bids, tricks_won },
        }
    }
}
impl<'a> NewHand<'a, Finished<'a>> {}

// pub struct Hand {}

// pub struct HandStart<'a> {
//     pub dealer: &'a Box<dyn Player>,
//     pub deck: Deck,
// }

// pub struct HandPlaying {
//     pub trump_card: Card,
// }

// pub struct HandEnd {
//     // pub tricks: Vec<Trick>,
// }

// impl Hand {
//     pub fn new(dealer: &Box<dyn Player>, trump_card: Card, total_players: usize) -> HandStart {
//         let deck = Deck::new().deck_type(DeckType::Full).shuffle(Some(7));
//         HandStart { dealer, deck }
//     }

//     // pub fn deal_players_in(
//     //     &mut self,
//     //     mut deck: Deck,
//     //     players: &'a Vec<Box<dyn Player>>,
//     //     num_tricks: &usize,
//     // ) {
//     //     let mut index: usize = 0;
//     //     while index < *num_tricks {
//     //         for player in players {
//     //             match self.player_cards.entry(player) {
//     //                 Entry::Vacant(e) => {
//     //                     let mut cards = Vec::with_capacity(2);
//     //                     cards.push(deck.debug_deal(index));
//     //                     e.insert(cards);
//     //                 }
//     //                 Entry::Occupied(mut e) => {
//     //                     e.get_mut().push(deck.debug_deal(index));
//     //                 }
//     //             };
//     //         }
//     //         index += 1;
//     //     }
//     // }

//     // pub fn play_hand(&mut self) {
//     //     // Move the logic from main into here
//     //     todo!()
//     // }
// }

// // impl fmt::Display for Hand {
// //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// //         write!(f, "{}", self)
// //     }
// // }
