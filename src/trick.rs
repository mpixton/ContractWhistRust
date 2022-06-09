use crate::card::Card;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Trick<'a> {
    cards_played: Vec<(Card, &'a String)>,
    trump_card: Card,
    winner: Option<&'a String>,
}

impl<'a> Trick<'a> {
    pub fn new(cards_played: Vec<(Card, &'a String)>, trump_card: Card) -> Trick {
        Trick {
            trump_card,
            cards_played,
            winner: None,
        }
    }

    // Determines and print the winner of the trick
    pub fn determine_winner(&mut self) {
        // Get the trump suit and led suit
        let (_, trump_suit) = self.trump_card.get_value();
        let (_, led_suit) = self.cards_played.get(0).unwrap().0.get_value();
        // Add the trump and led suit to the HashMap to map suits to integers for easier sorting
        let mut points = HashMap::with_capacity(2);
        points.insert(trump_suit, 3);
        points.insert(led_suit, 2);
        // Closure that maps the tuple of (Card, Players) to something more sort friendly
        let create_sortable_tuples = |e: &(Card, &'a String)| -> (i32, i32, &'a String) {
            let (rank, suit) = e.0.get_value();
            let suit_value = match points.get(suit) {
                Some(i) => i,
                None => &1,
            };

            (*suit_value, rank.get_numerical_rank(true), e.1)
        };
        // Create the card mapping and sort them by suit then rank
        // where trump suit > led suit > others
        let mut cards: Vec<(i32, i32, &String)> = self
            .cards_played
            .iter()
            .map(create_sortable_tuples)
            .collect();
        cards.sort_by(|a, b| b.cmp(a));
        // Set the winner and get the winning card from the cards played
        self.winner = Some(cards.first().unwrap().2);
        let winning_card: Result<&Card, ()> = match self
            .cards_played
            .iter()
            .find(|e| e.1 == self.winner.unwrap_or(&String::from("")))
        {
            Some(e) => Ok(&e.0),
            None => Err(()),
        };
        println!(
            "Winner is {} with the {}",
            self.winner.unwrap(),
            winning_card.unwrap()
        )
    }

    /// Display all Cards played in the Trick
    pub fn display_trick(&self) {
        println!("Trump Card: {}", self.trump_card);
        for (i, play) in self.cards_played.iter().enumerate() {
            match i {
                0 => println!("{} led with the {}", &play.1, play.0),
                _ => println!("{} played the {}", &play.1, play.0),
            }
        }
    }

    pub fn show_results(&mut self) {
        self.display_trick();
        println!("");
        self.determine_winner();
    }
}

// pub fn get_sorting_value(&self, suit_mapping: Option<&HashMap<Suit, isize>>) -> (isize, isize) {
//     (
//         match suit_mapping {
//             Some(map) => *self.suit.get_value(map),
//             None => 1,
//         },
//         self.rank.get_value(true),
//     )
// }
