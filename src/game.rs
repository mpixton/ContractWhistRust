use std::io;

use crate::player::{AIPlayer, HumanPlayer, Player};
use crate::MAX_DISPLAY_WIDTH;

pub struct MormonBridgeGame {
    pub players: Vec<Box<dyn Player>>,
}

impl MormonBridgeGame {
    pub fn new() -> Self {
        println!();
        println!("{:^1$}", "Welcome to Mormon Bridge!", MAX_DISPLAY_WIDTH);
        println!();

        let num_players: usize;
        let player_name: String;

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

        loop {
            let mut input = String::new();
            println!("What is your name?");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let name = input.trim();
                    match name.is_empty() {
                        false => {
                            player_name = name.to_string();
                            break;
                        }
                        true => println!("Please provide a name!"),
                    }
                }
                Err(_) => println!("There was an error attemping to read your input."),
            }
        }

        println!();
        println!();

        let mut players: Vec<Box<dyn Player>> = Vec::with_capacity(num_players + 1);
        let human_player = HumanPlayer::new(player_name);
        players.push(Box::new(human_player));

        for i in 0..num_players {
            let ai = AIPlayer::new(
                MormonBridgeGame::AI_PLAYER_NAMES
                    .get(i)
                    .unwrap()
                    .to_string(),
            );
            players.push(Box::new(ai));
        }

        MormonBridgeGame { players }
    }

    pub fn display_players(&self) {
        println!("{:^1$}", "Players", MAX_DISPLAY_WIDTH);
        println!("{:-<1$}", "", MAX_DISPLAY_WIDTH);

        for player in &self.players {
            println!("{}", player.get_name());
        }
    }

    const AI_PLAYER_NAMES: [&'static str; 6] = [
        "Mickey Mouse",
        "Minnie Mouse",
        "Donald Duck",
        "Daffy Duck",
        "Goofy Dog",
        "Pluto Dog",
    ];

    const TRICKS_PER_HAND: [i8; 13] = [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1];
}
