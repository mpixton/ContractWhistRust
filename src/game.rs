//! Main struct of the crate and entrypoint for playing a game.
//!
//! # Todo
//! [] Update documentation
//! [x] Call a series of hands automatically
//! [] Add typestate pattern

use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use crate::hand::Hand;
use crate::player::{AIPlayer, HumanPlayer, Player};
use crate::MAX_DISPLAY_WIDTH;

/// Struct of the Game.
pub struct MormonBridgeGame {}

impl MormonBridgeGame {
    pub fn play(debug: bool) {
        println!();
        println!("{:^1$}", "Welcome to Mormon Bridge!", MAX_DISPLAY_WIDTH);
        println!();

        let num_players = MormonBridgeGame::get_number_of_players();
        let player_name = MormonBridgeGame::get_human_player_name();

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

        let tricks = match debug {
            false => MormonBridgeGame::TRICKS_PER_HAND.to_vec(),
            true => MormonBridgeGame::DEBUG_TRICKS.to_vec(),
        };

        let mut cumulative_points: HashMap<&Box<dyn Player>, isize> =
            HashMap::with_capacity(players.len());

        for (index, trick_num) in tricks.iter().enumerate() {
            let hand = Hand::new(&players, *trick_num, &players[index % players.len()])
                .deal_players_in()
                .get_player_bids()
                .play_tricks()
                .score_hand();
            println!();
            println!("Points for Hand {}", index + 1);
            println!();
            hand.display_points();

            for (player, points) in hand.get_scores().iter() {
                cumulative_points
                    .entry(player)
                    .and_modify(|e| *e += points)
                    .or_insert(*points);
            }

            println!("Points through Hand {}", index + 1);
            MormonBridgeGame::display_cumulative_points(&cumulative_points, &players);
        }

        println!();
        println!("Final Scores");

        MormonBridgeGame::display_cumulative_points(&cumulative_points, &players);
    }

    fn display_players(players: Vec<Box<dyn Player>>) {
        println!("{:^1$}", "Players", MAX_DISPLAY_WIDTH);
        println!("{}", "-".repeat(MAX_DISPLAY_WIDTH));

        for player in players.iter() {
            println!("{}", player.get_name());
        }
    }

    fn display_cumulative_points(
        cumulative_points: &HashMap<&Box<dyn Player>, isize>,
        players: &[Box<dyn Player>],
    ) {
        println!();
        println!("     Player         Score");
        println!("{}", "-".repeat(26));
        for player in players.iter() {
            let points = cumulative_points.get(player).unwrap();
            println!("{:<20} {:^5}", format!("{}", player), points);
        }
    }

    fn get_number_of_players() -> usize {
        loop {
            let mut input = String::new();
            println!("How many computer opponents would you like to play with?");
            println!("Choose a number between 1 and 6.");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().parse() {
                        Ok(num) => match num {
                            1..=6 => return num,
                            _ => println!("{} is not between 1 and 6", num),
                        },
                        Err(_) => println!("The value you provided is not a number!"),
                    };
                }
                Err(_) => println!("Error attempting to read input."),
            };
        }
    }

    fn get_human_player_name() -> String {
        loop {
            let mut input = String::new();
            println!("What is your name?");
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let name = input.trim();
                    match name.is_empty() {
                        false => return name.to_string(),
                        true => println!("Please provide a name!"),
                    }
                }
                Err(_) => println!("There was an error attemping to read your input."),
            }
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

    const TRICKS_PER_HAND: [usize; 13] = [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1];
    const DEBUG_TRICKS: [usize; 5] = [1, 3, 5, 7, 1];
}
