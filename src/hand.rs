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
//! - [x] Implement scoring logic
//! - [x] Change the order of the players passed to the trick based on the previous trick's winner
//! - [ ] Update layout so that Trick::new returns a Builder and Trick<Finished> is just a Trick struct

//! # States
//! [Dealing]: the hand is dealing all players in and setting trump <br>
//! [Bidding]: asking each player for their bid for the hand <br>
//! [Playing]: playing the hand by playing a series of tricks <br>
//! [Scoring]: players are being scored on the hand <br>

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
#[derive(Debug)]
pub struct Hand<'a> {
    players: &'a Vec<Box<dyn Player>>,
    points: HashMap<&'a Box<dyn Player>, isize>,
}

pub struct InProgressHand<'a, T: HandState> {
    players: &'a Vec<Box<dyn Player>>,
    extra: T,
}

/// State of the [Hand] while dealing players in.
///
/// Provides a hand of cards for each player, sets the trump, and sets the hand to the
/// [Bidding] state.
#[derive(Debug)]
pub struct Dealing<'a> {
    deck: Deck,
    num_tricks: usize,
    dealer: &'a Box<dyn Player>,
}

/// State of the [Hand] while gathering bids.
///
/// Asks [Player]s for their bids this [Hand] and stores them.
#[derive(Debug)]
pub struct Bidding<'a> {
    trump: Card,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
    num_tricks: usize,
    dealer: &'a Box<dyn Player>,
    bid_order: Vec<&'a Box<dyn Player>>,
}

/// State of the [Hand] while playing a series of [Trick]s.
///
/// Plays a number of [Trick]s equal to the `num-trick` parameter passed when creating
/// the [Hand].
#[derive(Debug)]
pub struct Playing<'a> {
    bids: HashMap<&'a Box<dyn Player>, isize>,
    trump: Card,
    num_tricks: usize,
    player_hands: HashMap<&'a Box<dyn Player>, Vec<Card>>,
    initial_player_order: Vec<&'a Box<dyn Player>>,
}

/// State of the [Hand] while scoring the players.
///
/// Checks each player's bid vs actual tricks taken and determines points for the [Hand].
#[derive(Debug)]
pub struct Scoring<'a> {
    bids: HashMap<&'a Box<dyn Player>, isize>,
    tricks_won: HashMap<&'a Box<dyn Player>, isize>,
}

// /// Final state of the [Hand] containing total points scored by player.
// #[derive(Debug)]
// pub struct Finished<'a> {
//     points: HashMap<&'a Box<dyn Player>, isize>,
// }

/// Used to constraint the structs that may be used with [Hand].
pub trait HandState {}
impl<'a> HandState for Dealing<'a> {}
impl<'a> HandState for Bidding<'a> {}
impl<'a> HandState for Playing<'a> {}
impl<'a> HandState for Scoring<'a> {}

impl<'a> Hand<'a> {
    /// Creates the new [Hand] and returns the [Dealing] state.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        players: &'a Vec<Box<dyn Player>>,
        num_tricks: usize,
        dealer: &'a Box<dyn Player>,
    ) -> InProgressHand<'a, Dealing<'a>> {
        let deck = Deck::new().deck_type(DeckType::Full).shuffle(7).end();

        InProgressHand {
            players,
            extra: Dealing {
                deck,
                num_tricks,
                dealer,
            },
        }
    }

    /// Get the player scores for the Hand.
    pub fn get_scores(&self) -> &HashMap<&'a Box<dyn Player>, isize> {
        &self.points
    }

    /// Display the final points for the Hand.
    pub fn display_points(&self) {
        let points = &self.points;
        println!();
        println!("     Player         Score");
        println!("{}", "-".repeat(26));
        for player in self.players.iter() {
            let points = points.get(player).unwrap();
            println!("{:<20} {:^5}", format!("{}", player), points);
        }
    }
}

impl<'a> InProgressHand<'a, Dealing<'a>> {
    /// Generates a hand of cards for each player, set the trump, and returns the [Bidding] state.
    pub fn deal_players_in(self) -> InProgressHand<'a, Bidding<'a>> {
        let players = self.players;
        let num_tricks = self.extra.num_tricks;
        let mut deck = self.extra.deck;
        let dealer = self.extra.dealer;

        let trump = deck.deal();
        let mut player_hands: PlayerHands = HashMap::with_capacity(players.len());

        println!();
        println!("{} is dealing...", &dealer);

        for _ in 0..num_tricks {
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
        }

        let total_players = players.len();
        let dealer_position = players.iter().position(|e| e == dealer).unwrap();
        let mut bid_order: Vec<&Box<dyn Player>> = Vec::with_capacity(total_players);

        for index in dealer_position..total_players + dealer_position {
            bid_order.push(&players[index % total_players]);
        }

        InProgressHand {
            players,
            extra: Bidding {
                player_hands,
                trump,
                num_tricks,
                bid_order,
                dealer,
            },
        }
    }
}

impl<'a> InProgressHand<'a, Bidding<'a>> {
    // Ask each player for their bid this Hand and return the Playing state.
    pub fn get_player_bids(self) -> InProgressHand<'a, Playing<'a>> {
        let player_hands: PlayerHands = self.extra.player_hands;
        let trump = self.extra.trump;
        let players = self.players;
        let num_tricks = self.extra.num_tricks;
        let bid_order = self.extra.bid_order;
        let dealer = self.extra.dealer;

        let mut bids: HashMap<&'a Box<dyn Player>, isize> = HashMap::with_capacity(players.len());

        for player in bid_order.iter() {
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

        let total_players = players.len();
        let dealer_position = players.iter().position(|e| e == dealer).unwrap();
        let mut initial_player_order: Vec<&Box<dyn Player>> = Vec::with_capacity(total_players);

        for index in dealer_position..total_players + dealer_position {
            initial_player_order.push(&players[index % total_players]);
        }

        InProgressHand {
            players,
            extra: Playing {
                bids,
                trump,
                player_hands,
                num_tricks,
                initial_player_order,
            },
        }
    }
}

impl<'a> InProgressHand<'a, Playing<'a>> {
    // Plays through the number of tricks in this Hand and returns the Scoring state.
    pub fn play_tricks(self) -> InProgressHand<'a, Scoring<'a>> {
        let mut player_hands = self.extra.player_hands;
        let trump = self.extra.trump;
        let players = self.players;
        let bids = self.extra.bids;
        let num_tricks = self.extra.num_tricks;
        let mut player_order: Vec<&Box<dyn Player>> = self.extra.initial_player_order;

        let set_new_player_order = |winner| {
            let total_players = players.len();
            let winner_position = players.iter().position(|e| e == winner).unwrap();
            let mut new_player_order: Vec<&Box<dyn Player>> = Vec::with_capacity(total_players);

            for index in winner_position..total_players + winner_position {
                new_player_order.push(&players[index % total_players]);
            }

            new_player_order
        };

        let mut tricks_won: HashMap<&Box<dyn Player>, isize> =
            HashMap::with_capacity(players.len());

        for index in 0..num_tricks {
            println!();
            println!("Playing trick: {}", index + 1);
            let player_hands = &mut player_hands;
            let trick = Trick::new(&trump, player_order, player_hands)
                .play_trick()
                .determine_winner();
            let winner = trick.get_winner();
            trick.display_trick();

            tricks_won
                .entry(winner)
                .and_modify(|e| *e += 1)
                .or_insert(1);

            player_order = set_new_player_order(winner);
        }

        InProgressHand {
            players,
            extra: Scoring { bids, tricks_won },
        }
    }
}

impl<'a> InProgressHand<'a, Scoring<'a>> {
    /// Score the Hand and return a Finished Hand.
    pub fn score_hand(self) -> Hand<'a> {
        let players = self.players;
        let tricks_won = self.extra.tricks_won;
        let bids = self.extra.bids;

        let mut points: HashMap<&Box<dyn Player>, isize> = HashMap::with_capacity(players.len());

        for player in players.iter() {
            let player_bid: isize = *bids.get(player).unwrap();
            let player_tricks_won: isize = *tricks_won.get(player).unwrap_or(&0);

            let sandbag: isize = player_bid - player_tricks_won;

            match sandbag {
                0 => {
                    points.insert(player, 10 + player_bid);
                }
                num => {
                    points.insert(player, -(10 + num.abs()));
                }
            };
        }

        Hand { players, points }
    }
}
