use super::helpers::sort_hand;
use super::state::{Game, GameStatus, LossReason, Player};
use crate::card::Card;
use anyhow::{bail, Result};
use rand::Rng;

impl Game {
    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn is_solo(&self) -> bool {
        self.num_players() == 1
    }

    pub fn current(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn current_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player]
    }

    pub fn enemy_health_remaining(&self) -> u32 {
        let Some(enemy) = self.enemy else {
            return 0;
        };
        enemy.enemy_health().saturating_sub(self.damage_dealt)
    }

    pub fn enemy_attack_current(&self) -> u32 {
        let Some(enemy) = self.enemy else {
            return 0;
        };
        enemy.enemy_attack().saturating_sub(self.shield)
    }

    pub fn can_yield(&self) -> bool {
        if self.status != GameStatus::Playing {
            return false;
        }
        // Solo: always allowed (no "other players" to have yielded).
        if self.is_solo() {
            return true;
        }
        let limit = (self.num_players() - 1) as u32;
        self.consecutive_yields < limit
    }

    pub fn can_use_solo_jester(&self) -> bool {
        self.is_solo()
            && self.solo_jesters_remaining > 0
            && matches!(self.status, GameStatus::Playing)
    }

    /// Discard hand and refill to max (solo jester). Does not count as diamond draws.
    pub fn use_solo_jester(&mut self, rng: &mut impl Rng) -> Result<Vec<String>> {
        if !self.can_use_solo_jester() {
            bail!("no solo jesters remaining");
        }
        self.solo_jesters_remaining -= 1;
        self.solo_jesters_used += 1;

        let mut msgs = vec![format!(
            "Solo Jester! Discard hand and refill to {} ({} left).",
            self.max_hand_size, self.solo_jesters_remaining
        )];

        let discarded = std::mem::take(&mut self.current_mut().hand);
        if !discarded.is_empty() {
            msgs.push(format!(
                "Discarded: {}",
                discarded
                    .iter()
                    .map(Card::short_colored)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            self.discard.extend(discarded);
        }

        let mut drawn = Vec::new();
        while self.current().hand.len() < self.max_hand_size {
            if self.tavern.is_empty() {
                break;
            }
            drawn.push(self.tavern.pop().unwrap());
        }
        let _ = rng;
        self.current_mut().hand.extend(drawn.iter().copied());
        sort_hand(&mut self.current_mut().hand);
        if drawn.is_empty() {
            msgs.push("Tavern empty — drew nothing.".into());
        } else {
            msgs.push(format!(
                "Drew: {}",
                drawn
                    .iter()
                    .map(Card::short_colored)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        Ok(msgs)
    }

    pub fn advance_to(&mut self, player: usize) {
        self.current_player = player % self.num_players();
        self.turn += 1;
        self.check_can_act();
    }

    pub fn advance_next(&mut self) {
        let next = (self.current_player + 1) % self.num_players();
        self.advance_to(next);
    }

    fn check_can_act(&mut self) {
        if !matches!(self.status, GameStatus::Playing) {
            return;
        }
        if self.current().hand.is_empty() && !self.can_yield() {
            self.status = GameStatus::Lost {
                reason: LossReason::CannotAct,
            };
        }
    }

    pub fn mark_lost_cannot_defend(&mut self) {
        self.status = GameStatus::Lost {
            reason: LossReason::CannotDefend,
        };
    }
}
