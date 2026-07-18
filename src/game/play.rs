use super::helpers::unique_suits;
use super::state::{Game, GameStatus, TurnReport};
use crate::card::{Card, Suit};
use anyhow::{bail, Result};
use colored::Colorize;
use rand::Rng;

impl Game {
    pub fn validate_play(&self, indices: &[usize]) -> Result<Vec<Card>> {
        if !matches!(self.status, GameStatus::Playing) {
            bail!("game is over");
        }
        if self.enemy.is_none() {
            bail!("no enemy");
        }
        if indices.is_empty() {
            bail!("select at least one card");
        }

        let hand = &self.current().hand;
        let mut seen = std::collections::HashSet::new();
        let mut cards = Vec::new();
        for &idx in indices {
            if idx >= hand.len() {
                bail!("invalid card index {}", idx + 1);
            }
            if !seen.insert(idx) {
                bail!("duplicate card index {}", idx + 1);
            }
            cards.push(hand[idx]);
        }

        if cards.len() == 1 {
            return Ok(cards);
        }

        if cards.iter().any(|c| c.is_jester()) {
            bail!("the Jester must be played alone");
        }

        if cards.len() == 2 && cards.iter().any(|c| c.is_animal()) {
            return Ok(cards);
        }

        if cards.iter().any(|c| c.is_animal()) {
            bail!("animal companions can only be paired with one other card");
        }
        if !(2..=4).contains(&cards.len()) {
            bail!("invalid combination");
        }
        let rank = cards[0].rank;
        if !rank.is_number() || cards.iter().any(|c| c.rank != rank) {
            bail!("combos must be 2–4 cards of the same number");
        }
        let total: u32 = cards.iter().map(|c| c.value()).sum();
        if total > 10 {
            bail!("combo total must be 10 or less (got {total})");
        }
        Ok(cards)
    }

    pub fn play_cards(&mut self, indices: &[usize], rng: &mut impl Rng) -> Result<TurnReport> {
        let cards = self.validate_play(indices)?;
        let mut ordered: Vec<(usize, Card)> = indices
            .iter()
            .copied()
            .zip(cards.iter().copied())
            .collect();
        ordered.sort_by_key(|(i, _)| *i);
        for (i, _) in ordered.into_iter().rev() {
            self.current_mut().hand.remove(i);
        }

        let attack_value: u32 = cards.iter().map(|c| c.value()).sum();
        let mut report = TurnReport {
            messages: vec![format!(
                "{} plays {} (attack {}).",
                self.current().name,
                cards
                    .iter()
                    .map(Card::short_colored)
                    .collect::<Vec<_>>()
                    .join(" + "),
                attack_value
            )],
            defeated_enemy: false,
            exact_defeat: false,
            chose_next: false,
            need_defend: false,
            damage_to_defend: 0,
        };

        if cards.len() == 1 && cards[0].is_jester() {
            self.played_against_enemy.push(cards[0]);
            self.immunity_cancelled = true;
            if self.pending_spade_shield > 0 {
                self.shield += self.pending_spade_shield;
                report.messages.push(format!(
                    "Pending spade shield +{} now applies (shield {}).",
                    self.pending_spade_shield, self.shield
                ));
                self.pending_spade_shield = 0;
            }
            self.clubs_damage_before_jester = 0;
            report.messages.push(
                "Jester! Enemy immunity cancelled. Skip damage & enemy attack."
                    .bright_yellow()
                    .bold()
                    .to_string(),
            );
            report.chose_next = true;
            self.consecutive_yields = 0;
            return Ok(report);
        }

        let suits = unique_suits(&cards);
        let enemy_suit = self.enemy.and_then(|e| e.suit);

        let hearts = suits.contains(&Suit::Hearts)
            && (self.immunity_cancelled || enemy_suit != Some(Suit::Hearts));
        let diamonds = suits.contains(&Suit::Diamonds)
            && (self.immunity_cancelled || enemy_suit != Some(Suit::Diamonds));
        let clubs = suits.contains(&Suit::Clubs);
        let spades = suits.contains(&Suit::Spades);

        if hearts {
            let n = self.resolve_hearts(attack_value, rng);
            report.messages.push(format!(
                "{} Heal: moved {n} card(s) from discard under the Tavern.",
                Suit::Hearts.colored_symbol()
            ));
        } else if suits.contains(&Suit::Hearts) {
            report.messages.push(format!(
                "{} blocked by enemy immunity.",
                Suit::Hearts.colored_symbol()
            ));
        }

        if diamonds {
            let drawn = self.resolve_diamonds(attack_value);
            if drawn.is_empty() {
                report.messages.push(format!(
                    "{} Draw: no cards drawn (full hands or empty tavern).",
                    Suit::Diamonds.colored_symbol()
                ));
            } else {
                report.messages.push(format!(
                    "{} Draw: {}",
                    Suit::Diamonds.colored_symbol(),
                    drawn
                        .iter()
                        .map(|(p, c)| format!("{}→{}", p, c.short_colored()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        } else if suits.contains(&Suit::Diamonds) {
            report.messages.push(format!(
                "{} blocked by enemy immunity.",
                Suit::Diamonds.colored_symbol()
            ));
        }

        let clubs_active =
            clubs && (self.immunity_cancelled || enemy_suit != Some(Suit::Clubs));
        let spades_active =
            spades && (self.immunity_cancelled || enemy_suit != Some(Suit::Spades));

        if spades {
            if spades_active {
                self.shield += attack_value;
                report.messages.push(format!(
                    "{} Shield +{attack_value} (total shield {}).",
                    Suit::Spades.colored_symbol(),
                    self.shield
                ));
            } else {
                self.pending_spade_shield += attack_value;
                report.messages.push(format!(
                    "{} blocked by immunity (pending {attack_value} until Jester).",
                    Suit::Spades.colored_symbol()
                ));
            }
        }

        if clubs && !clubs_active {
            self.clubs_damage_before_jester += attack_value;
            report.messages.push(format!(
                "{} blocked by immunity (will not double even after Jester).",
                Suit::Clubs.colored_symbol()
            ));
        } else if clubs {
            report.messages.push(format!(
                "{} Double damage! ({attack_value} → {}).",
                Suit::Clubs.colored_symbol(),
                attack_value * 2
            ));
        }

        let mut damage = attack_value;
        if clubs_active {
            damage *= 2;
        }

        self.damage_dealt += damage;
        self.played_against_enemy.extend(cards);
        report.messages.push(format!(
            "{} {damage} {} (enemy {}/{} HP left).",
            "Dealt".bright_red().bold(),
            "damage".bright_red(),
            self.enemy_health_remaining().to_string().bold(),
            self.enemy.unwrap().enemy_health()
        ));

        self.consecutive_yields = 0;

        let health = self.enemy.unwrap().enemy_health();
        if self.damage_dealt >= health {
            let exact = self.damage_dealt == health;
            report.defeated_enemy = true;
            report.exact_defeat = exact;
            self.resolve_defeat(exact, &mut report, rng);
            return Ok(report);
        }

        report.need_defend = true;
        report.damage_to_defend = self.enemy_attack_current();
        Ok(report)
    }

    pub fn yield_turn(&mut self) -> Result<TurnReport> {
        if !self.can_yield() {
            bail!("cannot yield — too many consecutive yields");
        }
        self.consecutive_yields += 1;
        let dmg = self.enemy_attack_current();
        Ok(TurnReport {
            messages: vec![format!(
                "{} yields. Enemy attacks for {dmg}.",
                self.current().name
            )],
            defeated_enemy: false,
            exact_defeat: false,
            chose_next: false,
            need_defend: true,
            damage_to_defend: dmg,
        })
    }
}
