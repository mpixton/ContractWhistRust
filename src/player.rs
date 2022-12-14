//! Functionality related to a [Player] of a Card game, such as bidding and
//! playing cards from a hand.
//!
//! Contains two types of Players, Human and AI. Human players ask for input
//! from stdinput and AI players make plays based on pre-decided logic.
//!
//! AI bidding logic is simple. AI players bid one for each card in trump. At a
//! future point, they will also take into account if they are the lead player
//! or not and adjust their bid accordingly.
//!
//! AI playing logic is also simple. They prefer playing high in the lead suit.
//! If no card in the led suit is found, they switch over to trump, playing low
//! to high. Otherwise, they play the highest slough card with no respect to
//! rank.
//!
//! Human players are asked for their bid and play from stdinput. Constraints
//! are placed so that a human player may not bid higher than the number of
//! tricks, a Card may not be played that they don't have, and that they must
//! follow suit if they have a card in the led suit.
//!
//! # Todo
//! - [ ] Update documentation
//! - [ ] Add lead player to bidding logic

use std::{fmt, hash, io};

use crate::{card::Card, MAX_DISPLAY_WIDTH};

/// Trait defining base Player behavior.
///
/// All that is expected is that the Player will store a `name`. This
/// name is used as a key in HashMaps for storing the cards in a Player's hand
/// and the points for the hand and game. The Hash, PartialEq, and Eq trait
/// implementations rely only on the player's name, so two players with the same
/// name are equal to each other and will have the same hash.
pub trait Player {
    /// Returns the name of the Player.
    ///
    /// Used in the Hash, PartialEq, and Eq implementation.
    fn get_name(&self) -> &String;
    /// Returns a card selected from the Player's hand.
    fn play_card(&self, trump: &Card, led: Option<&Card>, cards: Vec<Card>) -> (Card, Vec<Card>);
    /// Displays the hand of the Player.
    fn display_hand(&self, cards: &[Card]);
    /// Returns the Player's bid.
    fn get_player_bid(&self, trump: &Card, tricks_this_bid: &usize, cards: &[Card]) -> isize;
    /// Used to implement the Clone trait.
    fn clone_dyn(&self) -> Box<dyn Player>;
}

impl fmt::Display for dyn Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl fmt::Debug for dyn Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

impl hash::Hash for Box<dyn Player> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.get_name().hash(state);
    }
}

impl PartialEq for Box<dyn Player> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl Eq for Box<dyn Player> {}

impl Clone for Box<dyn Player> {
    fn clone(&self) -> Box<dyn Player> {
        self.clone_dyn()
    }
}

/// The Human implementation of the Player trait.
///
/// Asks the user for their input for bids and card plays.
#[derive(Debug, Clone)]
pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    /// Creates a new HumanPlayer.
    pub fn new(name: String) -> HumanPlayer {
        HumanPlayer { name }
    }
}

impl Player for HumanPlayer {
    fn clone_dyn(&self) -> Box<dyn Player> {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn display_hand(&self, cards: &[Card]) {
        println!("Index Card");
        println!("--------------------");
        for (index, card) in cards.iter().enumerate() {
            println!("{:^5}{:^2$}", index, card, MAX_DISPLAY_WIDTH - 5);
        }
    }

    fn get_player_bid(&self, trump: &Card, tricks_this_hand: &usize, cards: &[Card]) -> isize {
        let bid: isize;
        let max_bids: usize = *tricks_this_hand;

        println!();

        loop {
            let mut input = String::new();
            println!("Trump this hand is: {}", &trump);
            println!();
            self.display_hand(cards);
            println!();
            println!("What do you bid?");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().parse::<usize>() {
                        Ok(num) => {
                            if num <= max_bids {
                                let num: isize = num.try_into().unwrap();
                                bid = num;
                                break;
                            } else {
                                println!("Your bid exceeds the number of tricks in the hand.");
                                println!("Please bid between 0 and {}", max_bids);
                            }
                        }
                        Err(_) => println!("The value you provided is not a number."),
                    };
                }
                Err(_) => println!("Error attempting to read input."),
            };
        }
        bid
    }

    fn play_card(
        &self,
        trump: &Card,
        led: Option<&Card>,
        mut cards: Vec<Card>,
    ) -> (Card, Vec<Card>) {
        println!();
        println!("Here is your hand");
        self.display_hand(&cards);
        println!();
        println!("Trump is: {:>1$}", &trump, MAX_DISPLAY_WIDTH - 10);
        println!();
        if let Some(card) = led {
            println!("Led Card is: {:>1$}", &card, MAX_DISPLAY_WIDTH - 13);
        } else {
            println!("You are the lead player");
        }
        println!();
        println!("What card would you like to play?");

        // println!();
        // println!("HumanPlayer play_card called");
        // println!("{:#?}", cards);
        // println!();

        loop {
            let mut index = String::new();
            io::stdin().read_line(&mut index).unwrap_or(usize::MAX);
            let index: usize = match index.trim().parse() {
                Ok(num) => num,
                Err(_) => usize::MAX,
            };

            if index < cards.len() {
                if let Some(led_card) = led {
                    let (_, led_suit) = led_card.get_value();
                    let has_cards_in_led_suit =
                        cards.iter().filter(|e| e.get_value().1 == led_suit).count() > 0;
                    let chosen_card_is_in_led_suit =
                        cards.get(index).unwrap().get_value().1 == led_suit;

                    // println!();
                    // println!("Has card in led suit: {}", has_cards_in_led_suit);
                    // println!("Chosen card is in led suit: {}", chosen_card_is_in_led_suit);
                    // println!();

                    if has_cards_in_led_suit {
                        if chosen_card_is_in_led_suit {
                            return (cards.swap_remove(index), cards);
                        } else {
                            println!("You must follow suit");
                        }
                    } else {
                        return (cards.swap_remove(index), cards);
                    }
                } else {
                    return (cards.swap_remove(index), cards);
                }
            } else {
                println!("Tried selecting a card you don't have.");
                println!("Here is your hand.");
                self.display_hand(&cards);
                println!("What card would you like to play?");
            };
        }
    }
}

#[derive(Clone, Debug)]
pub struct AIPlayer {
    name: String,
}

impl AIPlayer {
    pub fn new(name: String) -> Self {
        AIPlayer { name }
    }
}

impl Player for AIPlayer {
    /// Used in the trait Player impl of Clone
    fn clone_dyn(&self) -> Box<dyn Player> {
        Box::new(self.clone())
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn display_hand(&self, cards: &[Card]) {
        for card in cards {
            println!("{}", card)
        }
    }

    #[allow(unused_variables)]
    fn get_player_bid(&self, trump: &Card, tricks_this_bid: &usize, cards: &[Card]) -> isize {
        cards
            .iter()
            .filter(|e| e.get_value().1 == trump.get_value().1)
            .count()
            .try_into()
            .unwrap()
    }

    /// Logic for playing a Card
    /// # Logic
    /// If there has been a led card:
    ///   Must follow suit and play the lowest ranking card in the led suit
    ///   If no cards in the led suit:
    ///     Play the lowest card in trump suit
    ///     If no cards in trump suit:
    ///       Play highest ranking card in whatever suit
    /// If no led card:
    ///   Lead with highest trump
    ///   If no cards in trump:
    ///    Play highest ranking card
    ///
    fn play_card(
        &self,
        trump: &Card,
        led: Option<&Card>,
        mut cards: Vec<Card>,
    ) -> (Card, Vec<Card>) {
        // Closure to map Card ranks to integers for easy sorting
        let rank_cards = |e: &Card| e.get_value().0.get_numerical_rank(true);
        // Since the led card may be either None (current player is the leader) or Some (current player is following)
        // check for those two states and determine playing logic
        match led {
            // Logic for following in a Trick
            Some(card) => {
                // Current player is following another player so is bound to the led suit if they have it
                // Closure to determine if the Card is in the led suit
                let is_in_led = |e: &&Card| e.get_value().1 == card.get_value().1;
                let mut led_suit_cards: Vec<i32> =
                    cards.iter().filter(is_in_led).map(rank_cards).collect();
                // If player has a led suit card, play the lowest possible
                if !led_suit_cards.is_empty() {
                    led_suit_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .position(|e| {
                            e.get_value().1 == card.get_value().1
                                && e.get_value().0.get_numerical_rank(true)
                                    == *led_suit_cards.iter().last().unwrap()
                        })
                        .unwrap();
                    (cards.swap_remove(card_to_play), cards)
                // Player has no led suit, so play the lowest trump suit card
                } else {
                    // Closure to determine if a Card is in the Trump suit
                    let is_in_trump = |e: &&Card| e.get_value().1 == trump.get_value().1;
                    let mut trump_suit_cards: Vec<i32> =
                        cards.iter().filter(is_in_trump).map(rank_cards).collect();
                    if !trump_suit_cards.is_empty() {
                        trump_suit_cards.sort_by(|a, b| b.cmp(a));
                        let card_to_play = cards
                            .iter()
                            .position(|e| {
                                e.get_value().1 == trump.get_value().1
                                    && e.get_value().0.get_numerical_rank(true)
                                        == *trump_suit_cards.iter().last().unwrap()
                            })
                            .unwrap();
                        (cards.swap_remove(card_to_play), cards)
                    // Player has no cards in trump, so play the highest card in whatever suit
                    } else {
                        let mut other_cards: Vec<i32> = cards.iter().map(rank_cards).collect();
                        other_cards.sort_by(|a, b| b.cmp(a));
                        let card_to_play = cards
                            .iter()
                            .position(|e| {
                                e.get_value().0.get_numerical_rank(true)
                                    == *other_cards.first().unwrap()
                            })
                            .unwrap();
                        (cards.swap_remove(card_to_play), cards)
                    }
                }
            }
            // Logic for leading a trick
            None => {
                // Closure to determine if a Card is in the Trump suit
                let is_in_trump = |e: &&Card| e.get_value().1 == trump.get_value().1;
                let mut trump_suit_cards: Vec<i32> =
                    cards.iter().filter(is_in_trump).map(rank_cards).collect();
                if !trump_suit_cards.is_empty() {
                    trump_suit_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .position(|e| {
                            e.get_value().1 == trump.get_value().1
                                && e.get_value().0.get_numerical_rank(true)
                                    == *trump_suit_cards.iter().last().unwrap()
                        })
                        .unwrap();
                    (cards.swap_remove(card_to_play), cards)
                // Player has no cards in trump, so play the highest card in whatever suit
                } else {
                    let mut other_cards: Vec<i32> = cards.iter().map(rank_cards).collect();
                    other_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .position(|e| {
                            e.get_value().0.get_numerical_rank(true)
                                == *other_cards.first().unwrap()
                        })
                        .unwrap();
                    (cards.swap_remove(card_to_play), cards)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    use crate::rank::Rank;
    use crate::suit::Suit;

    fn setup() -> (Card, Card, AIPlayer) {
        (
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Three, Suit::Clubs),
            AIPlayer::new(String::from("Tester")),
        )
    }

    #[test]
    fn ai_player_follows_suit() {
        let (trump_card, led_card, player) = setup();
        let expected_play = Card::new(Rank::Queen, led_card.suit());
        let other_card = Card::new(Rank::Two, Suit::Spades);
        let player_hand = vec![expected_play, other_card];

        let (card, new_hand) = player.play_card(&trump_card, Some(&led_card), player_hand);
        assert_eq!(card, expected_play);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_sloughs_highest_card_when_no_other_play() {
        let (trump_card, led_card, player) = setup();
        let slough_card = Card::new(Rank::Jack, Suit::Diamonds);
        let other_card = Card::new(Rank::Four, Suit::Spades);
        let player_hand = vec![slough_card, other_card];

        let (played, new_hand) = player.play_card(&trump_card, Some(&led_card), player_hand);

        assert_eq!(slough_card, played);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_trumps_if_given_chance() {
        let (trump_card, led_card, player) = setup();
        let in_trump_play = Card::new(Rank::Jack, trump_card.suit());
        let other_card = Card::new(Rank::Four, Suit::Spades);
        let player_hand = vec![in_trump_play, other_card];

        let (played, new_hand) = player.play_card(&trump_card, Some(&led_card), player_hand);

        assert_eq!(in_trump_play, played);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_plays_lowest_trump_if_multiple_trump() {
        let (trump_card, led_card, player) = setup();
        let in_trump_play = Card::new(Rank::Four, trump_card.suit());
        let other_card = Card::new(Rank::Jack, trump_card.suit());
        let player_hand = vec![in_trump_play, other_card];

        let (played, new_hand) = player.play_card(&trump_card, Some(&led_card), player_hand);

        assert_eq!(in_trump_play, played);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_plays_lowest_led_if_multiple_led() {
        let (trump_card, led_card, player) = setup();
        let in_trump_play = Card::new(Rank::Four, led_card.suit());
        let other_card = Card::new(Rank::Jack, led_card.suit());
        let player_hand = vec![in_trump_play, other_card];

        let (played, new_hand) = player.play_card(&trump_card, Some(&led_card), player_hand);

        assert_eq!(in_trump_play, played);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_leads_with_highest_trump() {
        let (trump_card, _, player) = setup();
        let led_card = Card::new(Rank::Four, trump_card.suit());
        let other_card = Card::new(Rank::Jack, trump_card.suit());
        let player_hand = vec![led_card, other_card];

        let (played, new_hand) = player.play_card(&trump_card, None, player_hand);

        assert_eq!(played, led_card);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_leads_with_highest_non_trump() {
        let (trump_card, _, player) = setup();
        let led_card = Card::new(Rank::Queen, Suit::Clubs);
        let other_card = Card::new(Rank::Five, Suit::Clubs);
        let player_hand = vec![led_card, other_card];

        let (played, new_hand) = player.play_card(&trump_card, None, player_hand);

        assert_eq!(played, led_card);
        assert_eq!(new_hand.len(), 1);
        assert!(new_hand.contains(&other_card));
    }

    #[test]
    fn ai_player_bids_one_for_each_card_in_trump() {
        let (trump_card, _, player) = setup();
        let in_trump_play = Card::new(Rank::Four, trump_card.suit());
        let other_card = Card::new(Rank::Jack, trump_card.suit());
        let player_hand = vec![in_trump_play, other_card];

        let bid = player.get_player_bid(&trump_card, &2, &player_hand);

        assert_eq!(bid, 2);
    }

    #[test]
    fn ai_player_stores_name() {
        let player_name = "Tester";
        let player = AIPlayer {
            name: player_name.to_string(),
        };

        assert_eq!(player.get_name(), player_name);
    }
}
