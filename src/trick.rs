//! A play of a Trick of Contract Whist, consisting of asking players for their plays.
//!
//! The Trick progresses through four states. The first is the Start state, where player
//! hands, the player order, and the trump card are passed into the struct. This is
//! initiated by calling the `new()` function on the Trick struct. This state is used to
//! create a Playing state. The Playing state is used to ask each player, in order, for
//! their plays. Once all plays are collected, the trick enters the Scoring state to
//! determine the winner. After scoring, the trick comes to rest in the Finished state,
//! which is used to get the trick winner.
//!
//! # States
//! [Playing]: Asks each player in order for their [Card]s. <br>
//! [Scoring]: Determines the winner of the [Trick] based on the Trump and Led suit. <br>
//!
//! # Todo
//! - [ ] Update documentation <br>

use std::collections::HashMap;

use crate::card::Card;
use crate::player::Player;
use crate::suit::Suit;
use crate::PlayerHands;

/// Trick struct for using Generic Type Parameters.
pub struct Trick<'a> {
    winner: &'a Box<dyn Player>,
}

/// Struct to carry a Trick from start to finish.
///
/// Only stores the data needed to take it through the playing states.
pub struct InProgressTrick<T: TrickState> {
    extra: T,
}

/// State of the [Trick] while being played.
///
/// Asks [Player]s for their [Card] and moves the [Trick] to the [Scoring] state.
///
/// The `'a`, `'b`, and `'c` lifetimes are used to help the compiler with lifetimes.
/// Lifetime `'a` is the longest, and is used for the reference to a [Player], which
/// are instantiated when the [crate::game::ContractWhistGame] is created. Lifetime `'b`
/// is used for a reference to a trump card, which is owned by the [crate::hand::Hand]
/// that is playing the [Trick]. Lifetime `'c'` is used to denote the lifetime of the
/// `player_hands` borrow which is also owned by the [crate::hand::Hand] playing the
/// [Trick]. `player_hands` is borrowed mutably to allow for the hand to change between
/// plays of the [Trick].  
pub struct Playing<'a, 'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
    players: Vec<&'a Box<dyn Player>>,
    trump_card: &'b Card,
    player_hands: &'c mut PlayerHands<'a>,
}

/// State of the [Trick] while determing the winner.
///
/// Determines the winner of the [Trick] based on the Trump and Led suit.
///
/// See [Playing] for a discussion on the lifetimes.
pub struct Scoring<'a, 'b>
where
    'a: 'b,
{
    cards_played: HashMap<&'a Box<dyn Player>, Card>,
    trump_card: &'b Card,
    players: Vec<&'a Box<dyn Player>>,
}

pub trait TrickState {}
impl<'a, 'b, 'c> TrickState for Playing<'a, 'b, 'c> {}
impl<'a, 'b, 'c> TrickState for Scoring<'a, 'b> {}

impl<'a, 'b, 'c> Trick<'a> {
    /// Creates a new [Trick] and returns the [Playing] state.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        trump_card: &'b Card,
        players: Vec<&'a Box<dyn Player>>,
        player_hands: &'c mut PlayerHands<'a>,
    ) -> InProgressTrick<Playing<'a, 'b, 'c>> {
        InProgressTrick {
            extra: Playing {
                trump_card,
                player_hands,
                players,
            },
        }
    }

    /// Returns the winner of the [Trick].
    pub fn get_winner(&self) -> &'a Box<dyn Player> {
        self.winner
    }

    pub fn display_trick(&self) {
        println!();
        println!("{} is the winner!", self.winner);
    }
}

impl<'a, 'b, 'c> InProgressTrick<Playing<'a, 'b, 'c>> {
    /// Asks [Player]s for their [Card]s and returns the [Scoring] state.
    pub fn play_trick(self) -> InProgressTrick<Scoring<'a, 'b>> {
        let player_hands = self.extra.player_hands;
        let players = self.extra.players;
        let trump_card: &'b Card = self.extra.trump_card;

        let mut cards_played: HashMap<&Box<dyn Player>, Card> =
            HashMap::with_capacity(players.len());

        for player in &players {
            let card: Card;
            let new_hand: Vec<Card>;
            if cards_played.is_empty() {
                let player_hand = player_hands.get(*player).unwrap().to_owned();
                (card, new_hand) = player.play_card(self.extra.trump_card, None, player_hand);
            } else {
                let first_player = players.first().unwrap();
                let first_card = cards_played.get(first_player).unwrap();
                let player_hand = player_hands.get(player).unwrap().to_owned();
                (card, new_hand) =
                    player.play_card(self.extra.trump_card, Some(first_card), player_hand);
            }
            println!("{} played the {}", player, &card);
            cards_played.insert(player, card);
            player_hands.insert(player, new_hand);
        }

        // for (player, card) in cards_played.iter() {
        //     println!("{} played the {}", player, card);
        // }

        InProgressTrick {
            extra: Scoring {
                cards_played,
                players,
                trump_card,
            },
        }
    }
}

impl<'a, 'b, 'c> InProgressTrick<Scoring<'a, 'b>> {
    /// Determines the winner and returns the [Finished] state.
    pub fn determine_winner(self) -> Trick<'a> {
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

        Trick { winner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::AIPlayer;
    use crate::rank::Rank;
    use crate::suit::Suit;

    fn setup_trump() -> Card {
        Card::new(Rank::Ace, Suit::Hearts)
    }

    // #[test]
    // fn creating_new_trick_returns_playing_status() {
    //     let trump_card = setup_trump();
    //     let players: Vec<&Box<dyn Player>> = vec![
    //         &Box::new(AIPlayer::new("Mickey".to_string())),
    //         &Box::new(AIPlayer::new("Minnie".to_string())),
    //         &Box::new(AIPlayer::new("Donald".to_string())),
    //         &Box::new(AIPlayer::new("Daffy".to_string())),
    //     ];
    //     let player_hands: PlayerHands = HashMap::with_capacity(4);

    //     for (index, player) in players.iter().enumerate() {
    //         player_hands.insert(
    //             &player,
    //             vec![Card::new(Rank::VALUES[index % 3], Suit::VALUES[index % 3])],
    //         );
    //     }
    //     let trick = Trick::new(&trump_card, players, &mut player_hands);
    // }
}
