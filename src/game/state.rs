use crate::card::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Playing,
    Won { jesters_used: u8 },
    Lost { reason: LossReason },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LossReason {
    CannotDefend,
    CannotAct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub players: Vec<Player>,
    pub current_player: usize,
    /// Draw pile; last element is the top card.
    pub tavern: Vec<Card>,
    pub discard: Vec<Card>,
    /// Remaining face-down castle enemies; last element is drawn next.
    pub castle: Vec<Card>,
    pub enemy: Option<Card>,
    pub damage_dealt: u32,
    pub shield: u32,
    pub played_against_enemy: Vec<Card>,
    /// Spades played before a jester against a spades enemy become active after the jester.
    pub pending_spade_shield: u32,
    /// Clubs damage multiplier ignored until jester cancels clubs immunity.
    pub clubs_damage_before_jester: u32,
    pub immunity_cancelled: bool,
    pub consecutive_yields: u32,
    pub max_hand_size: usize,
    pub solo_jesters_remaining: u8,
    pub solo_jesters_used: u8,
    pub status: GameStatus,
    pub turn: u32,
}

#[derive(Debug, Clone)]
pub struct TurnReport {
    pub messages: Vec<String>,
    pub defeated_enemy: bool,
    pub exact_defeat: bool,
    pub chose_next: bool,
    pub need_defend: bool,
    pub damage_to_defend: u32,
}
