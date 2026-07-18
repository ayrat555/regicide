use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rank {
    Ace, // Animal Companion
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Jester,
}

impl Rank {
    pub fn label(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Jester => "*",
        }
    }

    /// Attack / discard value of this rank.
    pub fn value(self) -> u32 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack => 10,
            Rank::Queen => 15,
            Rank::King => 20,
            Rank::Jester => 0,
        }
    }

    pub fn is_animal(self) -> bool {
        self == Rank::Ace
    }

    pub fn is_number(self) -> bool {
        matches!(
            self,
            Rank::Two
                | Rank::Three
                | Rank::Four
                | Rank::Five
                | Rank::Six
                | Rank::Seven
                | Rank::Eight
                | Rank::Nine
                | Rank::Ten
        )
    }
}
