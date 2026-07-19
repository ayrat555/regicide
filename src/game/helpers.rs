use crate::card::{Card, Suit};

pub(super) fn unique_suits(cards: &[Card]) -> Vec<Suit> {
    let mut out = Vec::new();
    for c in cards {
        if let Some(s) = c.suit
            && !out.contains(&s)
        {
            out.push(s);
        }
    }
    out
}

pub(super) fn sort_hand(hand: &mut [Card]) {
    hand.sort_by_key(|c| {
        let suit_ord = match c.suit {
            Some(Suit::Spades) => 0,
            Some(Suit::Hearts) => 1,
            Some(Suit::Diamonds) => 2,
            Some(Suit::Clubs) => 3,
            None => 4,
        };
        let rank_ord = c.value();
        (suit_ord, rank_ord, c.rank.label())
    });
}
