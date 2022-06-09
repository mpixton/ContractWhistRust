#![allow(dead_code)]

use std::io;

mod card;
mod deck;
mod game;
mod player;
mod rank;
mod suit;
mod trick;

use crate::card::Card;
use crate::deck::{Deck, DeckType};
use crate::game::MormonBridgeGame;
use crate::player::HumanPlayer;
use crate::trick::Trick;

fn main() {
    // let mut deck = Deck::new(DeckType::Full, true);

    // let players: [String; 4] = [
    //     String::from("Mickey Mouse"),
    //     String::from("Minnie Mouse"),
    //     String::from("Donald Duck"),
    //     String::from("Daffy Duck"),
    // ];

    // let mut cards_played: Vec<(Card, &String)> = Vec::with_capacity(4);

    // for player in players.iter() {
    //     cards_played.push((deck.deal(), player));
    // }

    // let mut trick = Trick::new(cards_played, deck.deal());

    // trick.show_results();

    println!("Welcome to Mormon Bridge!");

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    let num_players: usize;

    loop {
        let mut input = String::new();
        println!("How many computer opponents would you like to play with?");
        println!("Choose a number between 1 and 6.");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse() {
                    Ok(num) => match num {
                        1..=6 => {
                            num_players = num;
                            break;
                        }
                        _ => println!("{} is not between 1 and 6", num),
                    },
                    Err(_) => println!("The value you provided is not a number!"),
                };
            }
            Err(_) => println!("Error attempting to read input."),
        };
    }

    MormonBridgeGame::new(num_players);
}
