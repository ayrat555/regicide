use crate::card::{render_enemy, render_hand, Suit};
use crate::game::Game;
use colored::Colorize;

pub fn print_board(game: &Game) {
    println!();
    println!("{}", "━".repeat(44).bright_black());
    println!(
        "{} {}  {}  {} {}  {}  {} {}  {}  {} {}",
        "Turn".bright_black(),
        game.turn.to_string().bold().white(),
        "│".bright_black(),
        "Tavern".bright_black(),
        game.tavern.len().to_string().cyan().bold(),
        "│".bright_black(),
        "Discard".bright_black(),
        game.discard.len().to_string().yellow(),
        "│".bright_black(),
        "Castle".bright_black(),
        game.castle.len().to_string().magenta().bold(),
    );
    if game.is_solo() {
        let j = game.solo_jesters_remaining;
        let j_col = if j > 0 {
            j.to_string().bright_yellow().bold().to_string()
        } else {
            j.to_string().bright_black().to_string()
        };
        println!(
            "{} {}  {} {}",
            "Solo jesters:".bright_black(),
            j_col,
            "used:".bright_black(),
            game.solo_jesters_used.to_string().bright_black()
        );
    }
    println!();

    if let Some(enemy) = game.enemy {
        println!("{}", "  ENEMY".bold().bright_yellow());
        println!("{}", render_enemy(&[enemy]));
        let hp = game.enemy_health_remaining();
        let max_hp = enemy.enemy_health();
        let atk = game.enemy_attack_current();
        let immune = if game.immunity_cancelled {
            "none (Jester)".bright_yellow().to_string()
        } else {
            enemy
                .suit
                .map(|s| format!("{} {}", s.colored_symbol(), s.colored_name()))
                .unwrap_or_else(|| "—".into())
        };
        println!(
            "  {} {} {}  {} {}",
            "ATK".bright_red().bold(),
            atk.to_string().bright_red().bold(),
            format!("(base {} − shield {})", enemy.enemy_attack(), game.shield).bright_black(),
            "HP".bright_green().bold(),
            hp_bar(hp, max_hp),
        );
        println!("  {} {}", "Immunity:".bright_black(), immune);
        if !game.played_against_enemy.is_empty() {
            println!(
                "  {} {}",
                "Played:".bright_black(),
                game.played_against_enemy
                    .iter()
                    .map(|c| c.short_colored())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    } else {
        println!("{}", "  No enemy.".bright_black());
    }

    println!();
    if game.num_players() > 1 {
        for (i, p) in game.players.iter().enumerate() {
            if i == game.current_player {
                println!(
                    "  {} {} {}",
                    "▶".bright_cyan().bold(),
                    p.name.bold().bright_cyan(),
                    format!("({} cards)", p.hand.len()).bright_black()
                );
            } else {
                println!(
                    "    {} {}",
                    p.name.white(),
                    format!("({} cards)", p.hand.len()).bright_black()
                );
            }
        }
        println!();
    }

    if game.is_solo() {
        println!("  {}", "YOUR HAND".bold().bright_cyan());
    } else {
        println!(
            "  {}",
            format!("{}'S HAND", game.current().name.to_uppercase())
                .bold()
                .bright_cyan()
        );
    }
    println!("{}", render_hand(&game.current().hand));
    println!("{}", "━".repeat(44).bright_black());
}

pub fn print_help_brief(game: &Game) {
    println!(
        "  {} {}  {} {}  {} {}  {} {}",
        Suit::Hearts.colored_symbol(),
        "heal".bright_black(),
        Suit::Diamonds.colored_symbol(),
        "draw".bright_black(),
        Suit::Clubs.colored_symbol(),
        "double".bright_black(),
        Suit::Spades.colored_symbol(),
        "shield".bright_black(),
    );
    let mut extras = Vec::new();
    if game.can_yield() {
        extras.push("yield (y)".yellow().to_string());
    }
    if game.can_use_solo_jester() {
        extras.push("jester (j)".bright_yellow().to_string());
    }
    extras.push("rules (r)".bright_black().to_string());
    extras.push("save (s)".cyan().to_string());
    extras.push("menu (m)".bright_black().to_string());
    extras.push("quit (q)".bright_black().to_string());
    extras.push("help (h)".bright_black().to_string());
    println!(
        "  {} {} {}",
        "Commands:".bright_black(),
        "play (p) <n…>".green(),
        format!("| {}", extras.join(" | ")).bright_black()
    );
}

fn hp_bar(current: u32, max: u32) -> String {
    let width = 12usize;
    let filled = if max == 0 {
        0
    } else {
        ((current as usize * width) + (max as usize / 2)) / max as usize
    };
    let filled = filled.min(width);
    let empty = width - filled;
    let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
    let ratio = if max == 0 {
        0.0
    } else {
        current as f64 / max as f64
    };
    let colored = if ratio > 0.6 {
        bar.bright_green().to_string()
    } else if ratio > 0.3 {
        bar.bright_yellow().to_string()
    } else {
        bar.bright_red().to_string()
    };
    format!("{colored} {}/{}", current.to_string().bold(), max)
}
