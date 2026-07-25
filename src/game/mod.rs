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

    #[test]
    fn solo_jester_refills_to_max_hand_only() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut g = Game::new(1, &mut rng).unwrap();
        let tavern_before = g.tavern.len();
        let hand_before = g.players[0].hand.len();
        assert_eq!(hand_before, g.max_hand_size);

        g.use_solo_jester(&mut rng).unwrap();

        assert_eq!(g.players[0].hand.len(), g.max_hand_size);
        assert_eq!(g.tavern.len(), tavern_before.saturating_sub(g.max_hand_size));
        assert_eq!(g.discard.len(), hand_before);
        assert_eq!(g.solo_jesters_remaining, 1);
        assert_eq!(g.solo_jesters_used, 1);
    }
}
