use std::io;

use crate::player::Player;

pub struct MormonBridgeGame {
    players: Vec<String>,
}

impl MormonBridgeGame {
    pub fn new(num_players: usize) -> () {
        let mut input = String::new();
        let player_name: String;

        loop {
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

        let ai_players: Vec<String> = MormonBridgeGame::AI_PLAYER_NAMES[..num_players]
            .iter()
            .map(|e| e.to_string())
            .collect();

        let human_player: Vec<String> = vec![player_name];

        let players = [human_player, ai_players].concat();

        println!("{:^25}", "Players");
        println!("{:-<1$}", "", 25);

        for player in players {
            println!("{}", player);
        }

        // let players = [MormonBridgeGame::AI_PLAYER_NAMES[0..4], ].concat()
        // MormonBridgeGame {
        //     players: Vec::with_capacity(num_players),
        // }
    }

    const AI_PLAYER_NAMES: [&'static str; 6] = [
        "Mickey Mouse",
        "Minnie Mouse",
        "Donald Duck",
        "Daffy Duck",
        "Goofy Dog",
        "Pluto Dog",
    ];
}
