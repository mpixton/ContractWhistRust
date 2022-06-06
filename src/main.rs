use std::io;

mod card;
mod deck;
mod game;
mod player;

use crate::deck::Deck;
use crate::player::{AIPlayer, HumanPlayer};

fn main() {
    let mut deck = Deck::new(52);

    println!("Deck created with {} cards", deck.total_cards());

    deck.shuffle(Some(7));

    // deck.debug_deck();

    let mut player_name = String::new();

    println!("What is your name?");
    while player_name.trim().is_empty() {
        io::stdin().read_line(&mut player_name).unwrap();
        if player_name.trim().is_empty() {
            println!("No name recieved, please try again.");
            println!("What is your name?");
        }
    }

    let player_name = player_name.trim().to_string();

    let mut player = HumanPlayer::new(player_name);

    let mut opponent = AIPlayer::new(String::from("Minnie Mouse"));

    player.add_card_to_hand(deck.deal());
    opponent.add_card_to_hand(deck.deal());

    println!("Hello {}", player);
    println!("Player hand");
    player.print_hand();
    println!("Your opponent is {}", opponent);
    println!("Their hand is");
    opponent.print_hand();

    println!("You played the {}", player.play_card().unwrap());
}
