use std::{fmt, hash, io};

use crate::card::Card;

pub trait Player {
    fn get_name(&self) -> &String;
    fn play_card(&mut self, trump: &Card, led: Option<&Card>, cards: &Vec<Card>) -> Card;
    fn display_hand(&self, cards: &Vec<Card>);
    fn set_player_bid(&self, trump: &Card, tricks_this_bid: &usize, cards: &Vec<Card>) -> usize;
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

#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: String) -> HumanPlayer {
        HumanPlayer { name }
    }
}

impl Player for HumanPlayer {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn display_hand(&self, cards: &Vec<Card>) {
        for card in cards {
            println!("{}", card)
        }
    }

    fn set_player_bid(&self, trump: &Card, tricks_this_hand: &usize, cards: &Vec<Card>) -> usize {
        let bid: usize;
        let max_bids: usize = *tricks_this_hand;

        loop {
            let mut input = String::new();
            println!("Trump this hand is: {}", &trump);
            println!("What do you bid?");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().parse::<usize>() {
                        Ok(num) => {
                            if 0 < num && num < max_bids {
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

    fn play_card(&mut self, trump: &Card, led: Option<&Card>, cards: &Vec<Card>) -> Card {
        println!("Here is your hand.");
        self.display_hand(cards);
        println!("Trump is: {}", &trump);
        println!("What card would you like to play?");

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
                    if has_cards_in_led_suit && chosen_card_is_in_led_suit {
                        return cards.swap_remove(index);
                    } else {
                        println!("You must follow suit!");
                    }
                } else {
                    return cards.swap_remove(index);
                }
            } else {
                println!("Tried selecting a card you don't have.");
                println!("Here is your hand.");
                self.display_hand(cards);
                println!("What card would you like to play?");
            };
        }
    }
}

pub struct AIPlayer {
    name: String,
}

impl AIPlayer {
    pub fn new(name: String) -> Self {
        AIPlayer { name }
    }
}

impl Player for AIPlayer {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn display_hand(&self, cards: &Vec<Card>) {
        for card in cards {
            println!("{}", card)
        }
    }

    #[allow(unused_variables)]
    fn set_player_bid(&self, trump: &Card, tricks_this_bid: &usize, cards: &Vec<Card>) -> usize {
        cards
            .iter()
            .filter(|e| e.get_value().1 == trump.get_value().1)
            .count()
    }

    /// Logic for playing a Card
    /// # Logic
    /// ```
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
    /// ```
    fn play_card(&mut self, trump: &Card, led: Option<&Card>, cards: &Vec<Card>) -> Card {
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
                if led_suit_cards.iter().count() > 0 {
                    led_suit_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .find(|e| {
                            e.get_value().1 == card.get_value().1
                                && e.get_value().0.get_numerical_rank(true)
                                    == *led_suit_cards.iter().last().unwrap()
                        })
                        .unwrap();
                    *card_to_play
                // Player has no led suit, so play the lowest trump suit card
                } else {
                    // Closure to determine if a Card is in the Trump suit
                    let is_in_trump = |e: &&Card| e.get_value().1 == trump.get_value().1;
                    let mut trump_suit_cards: Vec<i32> =
                        cards.iter().filter(is_in_trump).map(rank_cards).collect();
                    if trump_suit_cards.iter().count() > 0 {
                        trump_suit_cards.sort_by(|a, b| b.cmp(a));
                        let card_to_play = cards
                            .iter()
                            .find(|e| {
                                e.get_value().1 == trump.get_value().1
                                    && e.get_value().0.get_numerical_rank(true)
                                        == *trump_suit_cards.iter().last().unwrap()
                            })
                            .unwrap();
                        *card_to_play
                    // Player has no cards in trump, so play the highest card in whatever suit
                    } else {
                        let mut other_cards: Vec<i32> = cards.iter().map(rank_cards).collect();
                        other_cards.sort_by(|a, b| b.cmp(a));
                        let card_to_play = cards
                            .iter()
                            .find(|e| {
                                e.get_value().0.get_numerical_rank(true)
                                    == *other_cards.first().unwrap()
                            })
                            .unwrap();
                        *card_to_play
                    }
                }
            }
            // Logic for leading a trick
            None => {
                // Closure to determine if a Card is in the Trump suit
                let is_in_trump = |e: &&Card| e.get_value().1 == trump.get_value().1;
                let mut trump_suit_cards: Vec<i32> =
                    cards.iter().filter(is_in_trump).map(rank_cards).collect();
                if trump_suit_cards.iter().count() > 0 {
                    trump_suit_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .find(|e| {
                            e.get_value().1 == trump.get_value().1
                                && e.get_value().0.get_numerical_rank(true)
                                    == *trump_suit_cards.iter().last().unwrap()
                        })
                        .unwrap();
                    *card_to_play
                // Player has no cards in trump, so play the highest card in whatever suit
                } else {
                    let mut other_cards: Vec<i32> = cards.iter().map(rank_cards).collect();
                    other_cards.sort_by(|a, b| b.cmp(a));
                    let card_to_play = cards
                        .iter()
                        .find(|e| {
                            e.get_value().0.get_numerical_rank(true)
                                == *other_cards.first().unwrap()
                        })
                        .unwrap();
                    *card_to_play
                }
            }
        }
    }
}
