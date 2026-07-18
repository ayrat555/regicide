use super::state::{Game, GameStatus, LossReason};
use anyhow::{bail, Result};

impl Game {
    pub fn defend(&mut self, indices: &[usize]) -> Result<Vec<String>> {
        let needed = self.enemy_attack_current();
        if needed == 0 {
            return Ok(vec![
                "Enemy attack reduced to 0 — no cards discarded.".into(),
            ]);
        }

        let hand = &self.current().hand;
        let mut seen = std::collections::HashSet::new();
        let mut total = 0u32;
        let mut pick = Vec::new();
        for &idx in indices {
            if idx >= hand.len() {
                bail!("invalid card index {}", idx + 1);
            }
            if !seen.insert(idx) {
                bail!("duplicate card index {}", idx + 1);
            }
            total += hand[idx].value();
            pick.push((idx, hand[idx]));
        }

        if total < needed {
            let all: u32 = hand.iter().map(|c| c.value()).sum();
            if all < needed {
                self.status = GameStatus::Lost {
                    reason: LossReason::CannotDefend,
                };
                bail!(
                    "cannot cover {needed} damage (hand total {all}) — you are defeated!"
                );
            }
            bail!("selected cards total {total}, need at least {needed}");
        }

        pick.sort_by_key(|(i, _)| *i);
        let discarded: Vec<String> = pick.iter().map(|(_, c)| c.short_colored()).collect();
        for (i, card) in pick.into_iter().rev() {
            self.current_mut().hand.remove(i);
            self.discard.push(card);
        }
        Ok(vec![format!(
            "{} discards {} to cover {needed} damage.",
            self.current().name,
            discarded.join(", ")
        )])
    }

    /// Auto-pick a minimal discard set (greedy by value descending, then trim).
    pub fn suggest_defend(&self) -> Option<Vec<usize>> {
        let needed = self.enemy_attack_current();
        if needed == 0 {
            return Some(Vec::new());
        }
        let hand = &self.current().hand;
        let total: u32 = hand.iter().map(|c| c.value()).sum();
        if total < needed {
            return None;
        }

        let mut idxs: Vec<usize> = (0..hand.len()).collect();
        idxs.sort_by_key(|&i| std::cmp::Reverse(hand[i].value()));

        let mut chosen = Vec::new();
        let mut sum = 0u32;
        for i in idxs {
            chosen.push(i);
            sum += hand[i].value();
            if sum >= needed {
                break;
            }
        }

        chosen.sort_by_key(|&i| hand[i].value());
        let mut sum: u32 = chosen.iter().map(|&i| hand[i].value()).sum();
        let mut keep = chosen.clone();
        for &i in &chosen {
            let v = hand[i].value();
            if sum - v >= needed {
                sum -= v;
                keep.retain(|&x| x != i);
            }
        }
        keep.sort();
        Some(keep)
    }
}
