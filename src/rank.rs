use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
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
}

impl Rank {
    pub const VALUES: [Rank; 13] = [
        Self::Ace,
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
    ];

    pub fn get_numerical_rank(&self, aces_high: bool) -> i32 {
        match aces_high {
            true => self._aces_high_mapping(),
            false => self._aces_low_mapping(),
        }
    }

    fn _aces_high_mapping(&self) -> i32 {
        match &self {
            Rank::Ace => 14,
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
        }
    }

    fn _aces_low_mapping(&self) -> i32 {
        match &self {
            Rank::Ace => 14,
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
        }
    }
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "Ace"),
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
        }
    }
}
