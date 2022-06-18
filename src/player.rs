use std::{fmt, hash, io};

use crate::{card::Card, MAX_DISPLAY_WIDTH};

pub trait Player {
    fn get_name(&self) -> &String;
    fn play_card(&self, trump: &Card, led: Option<&Card>, cards: Vec<Card>) -> (Card, Vec<Card>);
    fn display_hand(&self, cards: &[Card]);
    fn get_player_bid(&self, trump: &Card, tricks_this_bid: &usize, cards: &[Card]) -> isize;
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

#[derive(Debug, Clone)]
pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
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
            println!("{:^5}{:^2$}", index, card, MAX_DISPLAY_WIDTH - 6);
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
        println!("Here is your hand.");
        self.display_hand(&cards);
        println!("Trump is: {}", &trump);
        if let Some(card) = led {
            println!("{} was led.", &card);
        }
        println!();
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
                        return (cards.swap_remove(index), cards);
                    } else {
                        println!("You must follow suit!");
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
