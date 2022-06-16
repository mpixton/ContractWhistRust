//! A play of a [Hand] (or round) of Mormon Bridge, consisting of dealing players in,
//! asking for bids, playing the tricks, and storing the resulting scores.
//!
//! The Hand progresses through 6 states. The first, Start, is only to create the Hand.
//! The players and number of tricks in the hand are saved and the deck is created. In
//! the Dealing state, cards are dealt to each player. The hand is then transitioned to
//! the Bidding state. In this state, each player is asked for their bid for the hand.
//! The hand then enters the Playing state. While Playing, the hand plays through a
//! number of tricks passed to the hand during instantiation. When finished playing all
//! tricks, the hand begins Scoring. Scoring is used to compare the actual number of
//! tricks won to the player's bid of tricks won and points tallied.
//!
//! # Todo
//! [] Implement scoring logic
//! [] Change the order of the players passed to the trick based on the previous trick's winner

//! Possible states for the hand:
//! [Start]: Used to create a new Hand
//! [Dealing]: the hand is dealing all players in and setting trump
//! [Bidding]: asking each player for their bid for the hand
//! [Playing]: playing the hand by playing a series of tricks
//! [Scoring]: players are being scored on the hand
//! [Finished]: the hand is over and players scored

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::card::Card;
use crate::deck::{Deck, DeckType};
use crate::player::Player;
use crate::trick::{self, Trick};
use crate::{PlayerHands, MAX_DISPLAY_WIDTH};

/// Base struct of the Hand, used with the HandState trait structs.
///
/// The Hand progresses through 5 states which determine the data stored in the hand.
pub struct Hand<'a, T: HandState> {
    players: &'a Vec<Box<dyn Player>>,
    extra: T,
}

/// Used to only to create a new [Hand].
///
/// Creates a new [Hand] and returns the [Dealing] state.
pub struct Start {
    deck: Deck,
    num_tricks: usize,
}

/// State of the [Hand] while dealing players in.
///
/// Provides a hand of cards for each player, sets the trump, and sets the hand to the
/// [Bidding] state.
pub struct Dealing {
    deck: Deck,
    num_tricks: usize,
}

/// State of the [Hand] while gathering bids.
///
/// Asks [Player]s for their bids this [Hand] and stores them.
pub struct Bidding<'a> {
    trump: Card,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
    num_tricks: usize,
}

/// State of the [Hand] while playing a series of [Trick]s.
///
/// Plays a number of [Trick]s equal to the `num-trick` parameter passed when creating
/// the [Hand].
pub struct Playing<'a> {
    bids: HashMap<&'a Box<dyn Player>, usize>,
    trump: Card,
    num_tricks: usize,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
}

/// State of the [Hand] while scoring the players.
///
/// Checks each player's bid vs actual tricks taken and determines points for the [Hand].
pub struct Scoring<'a> {
    bids: HashMap<&'a Box<dyn Player>, usize>,
    tricks_won: HashMap<&'a Box<dyn Player>, usize>,
}

/// Final state of the [Hand] containing total points scored by player.
pub struct Finished<'a> {
    points: HashMap<&'a Box<dyn Player>, usize>,
}

/// Used to constraint the structs that may be used with [Hand].
pub trait HandState {}
impl HandState for Start {}
impl HandState for Dealing {}
impl<'a> HandState for Bidding<'a> {}
impl<'a> HandState for Playing<'a> {}
impl<'a> HandState for Scoring<'a> {}
impl<'a> HandState for Finished<'a> {}

impl<'a> Hand<'a, Start> {
    /// Creates the new [Hand] and returns the [Bidding] state.
    pub fn new(players: &'a Vec<Box<dyn Player>>, num_tricks: usize) -> Hand<'a, Start> {
        let deck = Deck::new().deck_type(DeckType::Full).shuffle(Some(7));

        Hand {
            players,
            extra: Start { deck, num_tricks },
        }
    }
}

impl<'a> Hand<'a, Dealing> {
    /// Generates a hand of cards for each player, set the trump, and returns the Bidding state.
    pub fn deal_players_in(self) -> Hand<'a, Bidding<'a>> {
        let players = self.players;
        let num_tricks = self.extra.num_tricks;
        let mut deck = self.extra.deck;

        let trump = deck.deal();
        let mut player_hands: PlayerHands = HashMap::with_capacity(players.len());
        let mut index: usize = 0;

        while index < num_tricks {
            for (p_index, player) in players.iter().enumerate() {
                match player_hands.entry(player) {
                    Entry::Vacant(e) => {
                        let mut cards = Vec::with_capacity(num_tricks);
                        cards.push(deck.deal());
                        e.insert(cards);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(deck.deal());
                    }
                }
            }
            index += 1;
        }

        Hand {
            players,
            extra: Bidding {
                player_hands,
                trump,
                num_tricks,
            },
        }
    }
}

impl<'a> Hand<'a, Bidding<'a>> {
    // Ask each player for their bid this Hand and return the Playing state.
    pub fn get_player_bids(self) -> Hand<'a, Playing<'a>> {
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

        println!();
        println!("{:-^1$}", "Player Bids", MAX_DISPLAY_WIDTH);

        for (player, bid) in bids.iter() {
            println!("- {: <2$} {}", player, bid, 20);
        }

        println!();

        Hand {
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

impl<'a> Hand<'a, Playing<'a>> {
    // Plays through the number of tricks in this Hand and returns the Scoring state.
    pub fn play_tricks(self) -> Hand<'a, Scoring<'a>> {
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
            let trick = Trick::<trick::Start>::new(&trump, players, player_hands)
                .play_trick()
                .determine_winner();
            let winner = trick.get_winner();
            trick.display_trick();
            match tricks_won.entry(winner) {
                Entry::Vacant(_) => 1,
                Entry::Occupied(mut won) => {
                    let new_won = won.get_mut().to_owned() + 1;
                    new_won
                }
            };
            index += 1;
        }

        Hand {
            players,
            extra: Scoring { bids, tricks_won },
        }
    }
}

impl<'a> Hand<'a, Scoring<'a>> {}
impl<'a> Hand<'a, Finished<'a>> {}
