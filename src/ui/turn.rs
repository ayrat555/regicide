use super::fmt::{blank, print_event_lines, thin_rule};
use super::input::{parse_indices, pause, read_line};
use crate::card::render_hand;
use crate::game::{Game, GameStatus, TurnReport};
use crate::save;
use anyhow::{bail, Result};
use colored::Colorize;
use rand::rngs::StdRng;
use std::io::{self, Write};
use std::path::Path;

pub(super) fn finish_turn(
    game: &mut Game,
    report: TurnReport,
    save_path: &Path,
    rng: &mut StdRng,
    next_after_jester: Option<usize>,
) -> Result<()> {
    print_event_lines(&report.messages);

    if matches!(game.status, GameStatus::Won { .. } | GameStatus::Lost { .. }) {
        save::save_game(game, save_path)?;
        thin_rule();
        pause()?;
        return Ok(());
    }

    if report.defeated_enemy {
        save::save_game(game, save_path)?;
        thin_rule();
        pause()?;
        return Ok(());
    }

    if report.chose_next {
        let next = next_after_jester.unwrap_or(game.current_player);
        game.advance_to(next);
        save::save_game(game, save_path)?;
        thin_rule();
        pause()?;
        return Ok(());
    }

    if report.need_defend {
        if game.can_use_solo_jester() {
            blank();
            print!(
                "  Defend against {} damage. Use solo jester first? [y/N]: ",
                report.damage_to_defend
            );
            io::stdout().flush()?;
            let ans = read_line()?.trim().to_lowercase();
            if ans == "y" || ans == "yes" {
                match game.use_solo_jester(rng) {
                    Ok(msgs) => print_event_lines(msgs),
                    Err(e) => println!("  {} {e}", "✗".bright_red().bold()),
                }
            }
        }

        loop {
            match resolve_defend(game) {
                Ok(()) => break,
                Err(e) => {
                    if matches!(game.status, GameStatus::Lost { .. }) {
                        println!("  {} {e}", "✗".bright_red().bold());
                        save::save_game(game, save_path)?;
                        thin_rule();
                        pause()?;
                        return Ok(());
                    }
                    println!(
                        "  {} Invalid defend: {e}",
                        "✗".bright_red().bold()
                    );
                    println!("  Try again.");
                    blank();
                }
            }
        }

        game.advance_next();
    }

    save::save_game(game, save_path)?;
    thin_rule();
    pause()?;
    Ok(())
}

fn resolve_defend(game: &mut Game) -> Result<()> {
    if game.enemy_attack_current() == 0 {
        print_event_lines(["No damage to defend."]);
        return Ok(());
    }

    let needed = game.enemy_attack_current();
    let hand_total: u32 = game.current().hand.iter().map(|c| c.value()).sum();
    if hand_total < needed {
        game.mark_lost_cannot_defend();
        bail!(
            "{} cannot cover {needed} damage (hand total {hand_total}) — defeat!",
            game.current().name
        );
    }

    blank();
    println!(
        "  {} must discard cards totaling at least {needed}.",
        game.current().name
    );
    blank();
    println!("{}", render_hand(&game.current().hand));
    if let Some(sug) = game.suggest_defend() {
        let labels: Vec<String> = sug.iter().map(|i| (i + 1).to_string()).collect();
        println!("  Suggestion: {} (or `auto`)", labels.join(" "));
    }
    print!("  Discard cards (numbers / auto): ");
    io::stdout().flush()?;
    let line = read_line()?;
    let trimmed = line.trim().to_lowercase();
    let indices = if trimmed == "auto" || trimmed == "a" {
        game.suggest_defend()
            .ok_or_else(|| anyhow::anyhow!("cannot auto-defend"))?
    } else {
        parse_indices(&trimmed)?
    };

    let msgs = game.defend(&indices)?;
    print_event_lines(msgs);
    Ok(())
}

pub(super) fn ask_next_player(game: &Game) -> Result<usize> {
    if game.is_solo() {
        return Ok(0);
    }
    blank();
    println!("  Choose who goes next:");
    for (i, p) in game.players.iter().enumerate() {
        println!("    {}) {} ({} cards)", i + 1, p.name, p.hand.len());
    }
    print!("  Player: ");
    io::stdout().flush()?;
    let n: usize = read_line()?.trim().parse().unwrap_or(0);
    if n == 0 || n > game.num_players() {
        println!("  Invalid — current player continues.");
        Ok(game.current_player)
    } else {
        Ok(n - 1)
    }
}
