use super::Card;
use colored::Colorize;

/// Visual column width of a rendered card (excluding ANSI codes).
pub const CARD_WIDTH: usize = 7;

fn border(s: &str, enemy: bool) -> String {
    if enemy {
        s.bright_yellow().bold().to_string()
    } else {
        s.bright_black().to_string()
    }
}

pub(crate) fn ascii_card(card: Card, enemy: bool) -> [String; 5] {
    let rank = card.rank.label();
    let sym = match card.suit {
        Some(suit) => suit.symbol(),
        None => "*",
    };

    // Fixed inner width of 5 so the whole card is always 7 columns wide.
    let top = if rank.len() == 1 {
        format!("{rank}    ")
    } else {
        format!("{rank}   ")
    };
    let mid = format!("  {sym}  ");
    let bot = if rank.len() == 1 {
        format!("    {rank}")
    } else {
        format!("   {rank}")
    };

    let v = border("│", enemy);
    [
        border("┌─────┐", enemy),
        format!("{v}{}{v}", card.paint_face(&top)),
        format!("{v}{}{v}", card.paint_face(&mid)),
        format!("{v}{}{v}", card.paint_face(&bot)),
        border("└─────┘", enemy),
    ]
}

/// Render enemy card(s) with a highlighted frame.
pub fn render_enemy(cards: &[Card]) -> String {
    render_cards_inner(cards, true, false)
}

/// Render cards with 1-based indices under them.
pub fn render_hand(cards: &[Card]) -> String {
    render_cards_inner(cards, false, true)
}

fn render_cards_inner(cards: &[Card], enemy: bool, with_index: bool) -> String {
    if cards.is_empty() {
        return if with_index {
            "(empty hand)".bright_black().italic().to_string()
        } else {
            "(none)".bright_black().italic().to_string()
        };
    }

    let mut rows = vec![String::new(); 5];
    let mut index_row = String::new();
    for (i, card) in cards.iter().enumerate() {
        let lines = ascii_card(*card, enemy);
        for (r, line) in lines.iter().enumerate() {
            if i > 0 {
                rows[r].push(' ');
            }
            rows[r].push_str(line);
        }
        if with_index {
            if i > 0 {
                index_row.push(' ');
            }
            let label = format!("{:^width$}", i + 1, width = CARD_WIDTH);
            index_row.push_str(&label.cyan().dimmed().to_string());
        }
    }

    if with_index {
        format!("{}\n{}", rows.join("\n"), index_row)
    } else {
        rows.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\u{1b}' {
                for c2 in chars.by_ref() {
                    if c2.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    #[test]
    fn card_frame_aligned() {
        for card in [
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::jester(),
        ] {
            for line in ascii_card(card, false) {
                assert_eq!(strip_ansi(&line).chars().count(), CARD_WIDTH);
            }
            for line in ascii_card(card, true) {
                assert_eq!(strip_ansi(&line).chars().count(), CARD_WIDTH);
            }
        }
    }
}
