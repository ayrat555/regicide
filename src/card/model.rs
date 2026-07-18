use super::{Rank, Suit};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Option<Suit>,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit: Some(suit),
        }
    }

    pub fn jester() -> Self {
        Self {
            rank: Rank::Jester,
            suit: None,
        }
    }

    pub fn value(self) -> u32 {
        self.rank.value()
    }

    pub fn is_jester(self) -> bool {
        self.rank == Rank::Jester
    }

    pub fn is_animal(self) -> bool {
        self.rank.is_animal()
    }

    pub fn enemy_attack(self) -> u32 {
        match self.rank {
            Rank::Jack => 10,
            Rank::Queen => 15,
            Rank::King => 20,
            _ => 0,
        }
    }

    pub fn enemy_health(self) -> u32 {
        match self.rank {
            Rank::Jack => 20,
            Rank::Queen => 30,
            Rank::King => 40,
            _ => 0,
        }
    }

    pub fn short(&self) -> String {
        match self.suit {
            Some(suit) => format!("{}{}", self.rank.label(), suit.symbol()),
            None => "Jester".to_string(),
        }
    }

    pub fn short_colored(&self) -> String {
        match self.suit {
            Some(suit) => suit.paint(&self.short()).to_string(),
            None => "Jester".bright_yellow().bold().to_string(),
        }
    }

    pub(crate) fn paint_face(&self, text: &str) -> String {
        match self.suit {
            Some(suit) => suit.paint(text).to_string(),
            None => text.bright_yellow().bold().to_string(),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_colored())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values() {
        assert_eq!(Card::new(Rank::Ace, Suit::Hearts).value(), 1);
        assert_eq!(Card::new(Rank::Jack, Suit::Spades).value(), 10);
        assert_eq!(Card::new(Rank::Queen, Suit::Clubs).value(), 15);
        assert_eq!(Card::new(Rank::King, Suit::Diamonds).value(), 20);
        assert_eq!(Card::jester().value(), 0);
    }
}
