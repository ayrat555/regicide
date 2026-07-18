use super::helpers::sort_hand;
use super::state::{Game, GameStatus, TurnReport};
use crate::card::Card;
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::Rng;

impl Game {
    pub(super) fn resolve_hearts(&mut self, n: u32, rng: &mut impl Rng) -> u32 {
        if self.discard.is_empty() || n == 0 {
            return 0;
        }
        self.discard.shuffle(rng);
        let take = (n as usize).min(self.discard.len());
        let moved: Vec<Card> = self.discard.drain(0..take).collect();
        let count = moved.len() as u32;
        let mut new_tavern = moved;
        new_tavern.append(&mut self.tavern);
        self.tavern = new_tavern;
        count
    }

    pub(super) fn resolve_diamonds(&mut self, n: u32) -> Vec<(String, Card)> {
        let mut drawn = Vec::new();
        let mut draws_left = n;
        let start = self.current_player;
        let mut p = start;
        let mut guard = 0;
        while draws_left > 0 && guard < 10_000 {
            guard += 1;
            if self.tavern.is_empty() {
                break;
            }
            if self.players[p].hand.len() < self.max_hand_size {
                let card = self.tavern.pop().unwrap();
                let name = self.players[p].name.clone();
                self.players[p].hand.push(card);
                sort_hand(&mut self.players[p].hand);
                drawn.push((name, card));
                draws_left -= 1;
            }
            p = (p + 1) % self.num_players();
            if p == start {
                let anyone = self
                    .players
                    .iter()
                    .any(|pl| pl.hand.len() < self.max_hand_size);
                if !anyone || self.tavern.is_empty() {
                    break;
                }
            }
        }
        drawn
    }

    pub(super) fn resolve_defeat(
        &mut self,
        exact: bool,
        report: &mut TurnReport,
        _rng: &mut impl Rng,
    ) {
        let enemy = self.enemy.take().unwrap();
        report.messages.push(format!(
            "{} {}!{}",
            "Defeated".bright_green().bold(),
            enemy.short_colored(),
            if exact {
                " Exact kill — enemy goes on top of the Tavern."
                    .bright_cyan()
                    .to_string()
            } else {
                " Enemy goes to the discard.".bright_black().to_string()
            }
        ));

        if exact {
            self.tavern.push(enemy);
        } else {
            self.discard.push(enemy);
        }

        let played = std::mem::take(&mut self.played_against_enemy);
        self.discard.extend(played);

        self.damage_dealt = 0;
        self.shield = 0;
        self.pending_spade_shield = 0;
        self.clubs_damage_before_jester = 0;
        self.immunity_cancelled = false;

        if let Some(next) = self.castle.pop() {
            self.enemy = Some(next);
            report.messages.push(format!(
                "Next enemy: {} (ATK {}, HP {}).",
                next.short_colored(),
                next.enemy_attack(),
                next.enemy_health()
            ));
            report.messages.push(format!(
                "{} plays again against the new enemy.",
                self.current().name
            ));
        } else {
            self.enemy = None;
            self.status = GameStatus::Won {
                jesters_used: self.solo_jesters_used,
            };
            report.messages.push("All monarchs defeated — victory!".into());
        }
    }
}
