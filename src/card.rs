use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace,
}

impl Rank {
    pub const VALUES: [Self; 13] = [
        Self::King,
        Self::Queen,
        Self::Jack,
        Self::Ten,
        Self::Nine,
        Self::Eight,
        Self::Seven,
        Self::Six,
        Self::Five,
        Self::Four,
        Self::Three,
        Self::Two,
        Self::Ace,
    ];

    #[allow(dead_code)]
    fn get_value(&self) -> i8 {
        match self {
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
            Rank::Ace => 1,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::King => write!(f, "King"),
            Rank::Queen => write!(f, "Queen"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Ten => write!(f, "10"),
            Rank::Nine => write!(f, "9"),
            Rank::Eight => write!(f, "8"),
            Rank::Seven => write!(f, "7"),
            Rank::Six => write!(f, "6"),
            Rank::Five => write!(f, "5"),
            Rank::Four => write!(f, "4"),
            Rank::Three => write!(f, "3"),
            Rank::Two => write!(f, "2"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suit {
    pub const VALUES: [Self; 4] = [Self::Hearts, Self::Spades, Suit::Diamonds, Self::Clubs];

    #[allow(dead_code)]
    fn get_value(&self) -> String {
        match self {
            Suit::Hearts => String::from("Hearts"),
            Suit::Spades => String::from("Spades"),
            Suit::Diamonds => String::from("Diamonds"),
            Suit::Clubs => String::from("Clubs"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> (i8, String) {
        (self.rank.get_value(), self.suit.get_value())
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
