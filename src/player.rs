use std::fmt;
use std::io;

use crate::card::Card;

struct Player {
    pub hand: Vec<Card>,
    name: String,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            hand: Vec::new(),
            name,
        }
    }

    fn play_card(&mut self) -> Result<Card, &str> {
        if self.hand.len() == 0 {
            println!("No cards to play!");
            return Err("This is an error");
        }

        println!("Here is your hand.");
        self.print_hand();
        println!("What card would you like to play?");

        loop {
            let mut index = String::new();
            io::stdin().read_line(&mut index).unwrap_or(usize::MAX);
            let index: usize = match index.trim().parse() {
                Ok(num) => num,
                Err(_) => usize::MAX,
            };

            if index < self.hand.len() {
                return Ok(self.hand.swap_remove(index));
            } else {
                println!("Tried selecting a card you don't have.");
                println!("Here is your hand.");
                self.print_hand();
                println!("What card would you like to play?");
            };
        }
    }

    fn print_hand(&self) {
        for (i, card) in self.hand.iter().enumerate() {
            println!("{} - {}", i, &card);
        }
    }

    fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[allow(dead_code)]
pub struct AIPlayer {
    player: Player,
}

impl AIPlayer {
    pub fn new(name: String) -> AIPlayer {
        AIPlayer {
            player: Player::new(name),
        }
    }

    pub fn play_card(&mut self) -> Result<Card, &str> {
        self.player.play_card()
    }

    pub fn print_hand(&self) {
        self.player.print_hand()
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.player.add_card_to_hand(card)
    }

    #[allow(dead_code)]
    const AI_NAMES: [&'static str; 6] = [
        "Mickey Mouse",
        "Minnie Mouse",
        "Donald Duck",
        "Daffy Duck",
        "Goofy Dog",
        "Pluto Dog",
    ];

    pub fn create_ai_opponents(num_players: usize) -> Vec<AIPlayer> {
        let mut ai_players = Vec::with_capacity(num_players);

        for index in 0..num_players {
            ai_players.push(AIPlayer::new(AIPlayer::AI_NAMES[index].to_string()));
        }

        ai_players
    }
}

impl fmt::Display for AIPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.player)
    }
}

pub struct HumanPlayer {
    player: Player,
}

impl HumanPlayer {
    pub fn new(name: String) -> HumanPlayer {
        HumanPlayer {
            player: Player::new(name),
        }
    }

    pub fn play_card(&mut self) -> Result<Card, &str> {
        self.player.play_card()
    }

    pub fn print_hand(&self) {
        self.player.print_hand()
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.player.add_card_to_hand(card)
    }
}

impl fmt::Display for HumanPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.player)
    }
}
