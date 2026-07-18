use super::helpers::sort_hand;
use super::state::{Game, GameStatus, Player};
use crate::card::{Card, Rank, Suit};
use anyhow::{bail, Result};
use rand::seq::SliceRandom;
use rand::Rng;

impl Game {
    pub fn new(num_players: usize, rng: &mut impl Rng) -> Result<Self> {
        if !(1..=4).contains(&num_players) {
            bail!("player count must be 1–4");
        }

        let (jesters_in_tavern, max_hand_size, solo_jesters) = match num_players {
            1 => (0, 8, 2),
            2 => (0, 7, 0),
            3 => (1, 6, 0),
            4 => (2, 5, 0),
            _ => unreachable!(),
        };

        let mut jacks: Vec<Card> = Suit::ALL
            .iter()
            .map(|&s| Card::new(Rank::Jack, s))
            .collect();
        let mut queens: Vec<Card> = Suit::ALL
            .iter()
            .map(|&s| Card::new(Rank::Queen, s))
            .collect();
        let mut kings: Vec<Card> = Suit::ALL
            .iter()
            .map(|&s| Card::new(Rank::King, s))
            .collect();
        jacks.shuffle(rng);
        queens.shuffle(rng);
        kings.shuffle(rng);

        // Castle: bottom = kings, then queens, then jacks on top (draw from end).
        let mut castle = Vec::new();
        castle.extend(kings);
        castle.extend(queens);
        castle.extend(jacks);

        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Ace,
        ];
        let mut tavern: Vec<Card> = ranks
            .iter()
            .flat_map(|&r| Suit::ALL.iter().map(move |&s| Card::new(r, s)))
            .collect();
        for _ in 0..jesters_in_tavern {
            tavern.push(Card::jester());
        }
        tavern.shuffle(rng);

        let players: Vec<Player> = (0..num_players)
            .map(|i| Player {
                name: if num_players == 1 {
                    "You".to_string()
                } else {
                    format!("Player {}", i + 1)
                },
                hand: Vec::new(),
            })
            .collect();

        let enemy = castle.pop();

        let mut game = Self {
            players,
            current_player: 0,
            tavern,
            discard: Vec::new(),
            castle,
            enemy,
            damage_dealt: 0,
            shield: 0,
            played_against_enemy: Vec::new(),
            pending_spade_shield: 0,
            clubs_damage_before_jester: 0,
            immunity_cancelled: false,
            consecutive_yields: 0,
            max_hand_size,
            solo_jesters_remaining: solo_jesters,
            solo_jesters_used: 0,
            status: GameStatus::Playing,
            turn: 1,
        };

        // Deal up to max hand size, round-robin.
        'deal: loop {
            let mut dealt_any = false;
            for i in 0..num_players {
                if game.players[i].hand.len() >= max_hand_size {
                    continue;
                }
                if game.tavern.is_empty() {
                    break 'deal;
                }
                let card = game.tavern.pop().unwrap();
                game.players[i].hand.push(card);
                dealt_any = true;
            }
            if !dealt_any {
                break;
            }
            if game.players.iter().all(|p| p.hand.len() >= max_hand_size) {
                break;
            }
        }

        for p in &mut game.players {
            sort_hand(&mut p.hand);
        }

        Ok(game)
    }
}
