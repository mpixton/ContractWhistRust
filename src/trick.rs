//! A play of a Trick of Mormon Bridge, consisting of asking players for their plays.

// Possible states:
// Playing: ask each player for their Card
// Finished: winner is determined based on played Cards

use std::collections::HashMap;
use std::fmt;

use crate::card::Card;
use crate::player::Player;
use crate::suit::Suit;
use crate::PlayerHands;

/// New Trick struct for using Generic Type Parameters.
///
/// Creating a new Trick will return a [Playing] state, which allows for asking
/// players for a card and determining the winner.
pub struct Trick<T: TrickState> {
    extra: T,
}

/// State of the [Trick] while being played.
///
/// Asks [Player]s for their [Card], stores it, and determines the winner of the trick.
pub struct Playing<'a> {
    cards_played: HashMap<&'a Box<dyn Player>, Card>,
    players: &'a Vec<Box<dyn Player>>,
    trump_card: &'a Card,
    player_hands: &'a mut PlayerHands<'a>,
}

pub struct Scoring<'a> {
    cards_played: HashMap<&'a Box<dyn Player>, Card>,
    trump_card: &'a Card,
    players: &'a Vec<Box<dyn Player>>,
}

/// Final state of the [Trick].
///
/// Stores the winner of the [Trick] as a reference to a [Player] as received from the `cards_played` key.
pub struct Finished<'a> {
    winner: &'a Box<dyn Player>,
}

pub trait TrickState {}
impl<'a> TrickState for Playing<'a> {}
impl<'a> TrickState for Scoring<'a> {}
impl<'a> TrickState for Finished<'a> {}

impl<'a, T: TrickState> Trick<T> {
    pub fn new(
        trump_card: &'a Card,
        players: &'a Vec<Box<dyn Player>>,
        player_hands: &'a mut PlayerHands<'a>,
    ) -> Trick<Playing<'a>> {
        Trick {
            extra: Playing {
                players,
                trump_card,
                player_hands,
                cards_played: HashMap::with_capacity(players.len()),
            },
        }
    }
}

impl<'a> Trick<Playing<'a>> {
    pub fn play_trick(self) -> Trick<Scoring<'a>> {
        let mut player_hands = self.extra.player_hands;
        let players = self.extra.players;
        let trump_card = self.extra.trump_card;

        let mut cards_played = HashMap::with_capacity(players.len());

        for player in players.iter() {
            let card: Card;
            let new_hand: Vec<Card>;
            if cards_played.is_empty() {
                let player_hand = player_hands.get(player).unwrap().to_owned();
                (card, new_hand) = player.play_card(self.extra.trump_card, None, player_hand);
            } else {
                let first_player = players.first().unwrap();
                let first_card = cards_played.get(first_player).unwrap();
                let player_hand = player_hands.get(player).unwrap().to_owned();
                (card, new_hand) =
                    player.play_card(self.extra.trump_card, Some(first_card), player_hand);
            }
            cards_played.insert(player, card);
            player_hands.insert(player, new_hand);
        }

        Trick {
            extra: Scoring {
                cards_played,
                players,
                trump_card,
            },
        }
    }
}

impl<'a> Trick<Scoring<'a>> {
    pub fn determine_winner(self) -> Trick<Finished<'a>> {
        let players = self.extra.players;
        let cards_played = self.extra.cards_played;
        // Set up the trump and led suit
        let (_, trump_suit) = self.extra.trump_card.get_value();
        let lead_player = players.get(0).unwrap();
        let (_, led_suit) = cards_played.get(lead_player).unwrap().get_value();

        // Assign point values to the trump and led suit for ease of comparison
        let tuples = [(trump_suit, 3), (led_suit, 2)];
        let points: HashMap<&Suit, i32> = tuples.into_iter().collect();
        let create_sortable_tuples =
            |e: (&&'a Box<dyn Player>, &Card)| -> (i32, i32, &'a Box<dyn Player>) {
                let (rank, suit) = e.1.get_value();
                let suit_value = points.get(suit).unwrap_or(&1);

                (*suit_value, rank.get_numerical_rank(true), e.0)
            };
        // Create the card mapping and sort them by suit then rank
        // where trump suit > led suit > others
        let mut cards: Vec<(i32, i32, &'a Box<dyn Player>)> =
            cards_played.iter().map(create_sortable_tuples).collect();
        let sort_tuples = |a: &(i32, i32, &'a Box<dyn Player>),
                           b: &(i32, i32, &'a Box<dyn Player>)| {
            let (a_suit, a_rank, _) = a;
            let (b_suit, b_rank, _) = b;
            let new_a = (a_suit, a_rank);
            let new_b = (b_suit, b_rank);

            new_b.cmp(&new_a)
        };
        cards.sort_by(sort_tuples);

        // Set the winner and return the new state
        let winner = cards.first().unwrap().2;

        Trick {
            extra: Finished { winner },
        }
    }
}

impl<'a> Trick<Finished<'a>> {
    pub fn display_trick(&self) {
        println!("{} is the winner!", self.extra.winner);
    }
}

// impl<'a> fmt::Display for Trick<Finished<'a>> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.extra.winner)
//     }
// }
