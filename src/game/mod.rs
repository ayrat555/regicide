//! Game rules engine: setup, turns, suit powers, win/lose.

mod defend;
mod helpers;
mod play;
mod query;
mod resolve;
mod setup;
mod state;

pub use state::{Game, GameStatus, LossReason, Player, TurnReport};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn solo_setup() {
        let mut rng = StdRng::seed_from_u64(1);
        let g = Game::new(1, &mut rng).unwrap();
        assert_eq!(g.players[0].hand.len(), 8);
        assert_eq!(g.solo_jesters_remaining, 2);
        assert!(g.enemy.unwrap().rank == Rank::Jack);
        assert_eq!(g.castle.len(), 11);
    }

    #[test]
    fn combo_validation() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut g = Game::new(1, &mut rng).unwrap();
        g.players[0].hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Five, Suit::Diamonds),
        ];
        assert!(g.validate_play(&[0, 1, 2]).is_ok());
        assert!(g.validate_play(&[0, 3]).is_err());
    }
}
