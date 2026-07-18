use colored::{ColoredString, Colorize};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    pub fn symbol(self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        }
    }

    pub fn paint(self, text: &str) -> ColoredString {
        match self {
            Suit::Hearts => text.bright_red().bold(),
            Suit::Diamonds => text.red().bold(),
            Suit::Clubs => text.bright_green().bold(),
            Suit::Spades => text.bright_white().bold(),
        }
    }

    pub fn colored_symbol(self) -> String {
        self.paint(self.symbol()).to_string()
    }

    pub fn colored_name(self) -> String {
        self.paint(self.name()).to_string()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.colored_symbol())
    }
}
