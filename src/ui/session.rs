use super::board::{print_board, print_help_brief};
use super::input::{parse_indices, read_line};
use super::rules::print_help_full;
use super::turn::{ask_next_player, finish_turn};
use crate::game::{Game, GameStatus, LossReason};
use crate::save;
use anyhow::Result;
use colored::Colorize;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::io::{self, Write};
use std::path::PathBuf;

use super::input::pause;

pub fn play_loop(mut game: Game, save_path: &PathBuf) -> Result<()> {
    let mut rng = StdRng::from_os_rng();

    loop {
        match game.status {
            GameStatus::Won { jesters_used } => {
                print_board(&game);
                println!("{}", "╔══════════════════════════════════╗".bright_green());
                println!(
                    "{}{}{}",
                    "║".bright_green(),
                    "            VICTORY!              ".bold().bright_green(),
                    "║".bright_green()
                );
                println!("{}", "╚══════════════════════════════════╝".bright_green());
                if game.is_solo() {
                    let medal = match jesters_used {
                        0 => "Gold".bright_yellow().bold().to_string(),
                        1 => "Silver".bright_white().bold().to_string(),
                        _ => "Bronze".truecolor(205, 127, 50).bold().to_string(),
                    };
                    println!(
                        "Solo medal: {medal} {} {}",
                        "(jesters used:".bright_black(),
                        format!("{jesters_used})").bright_black()
                    );
                }
                let _ = std::fs::remove_file(save_path);
                pause()?;
                return Ok(());
            }
            GameStatus::Lost { reason } => {
                print_board(&game);
                println!("{}", "╔══════════════════════════════════╗".bright_red());
                println!(
                    "{}{}{}",
                    "║".bright_red(),
                    "             DEFEAT               ".bold().bright_red(),
                    "║".bright_red()
                );
                println!("{}", "╚══════════════════════════════════╝".bright_red());
                match reason {
                    LossReason::CannotDefend => {
                        println!(
                            "{}",
                            "A player could not discard enough to survive the attack."
                                .bright_red()
                        )
                    }
                    LossReason::CannotAct => {
                        println!(
                            "{}",
                            "A player could not play a card or yield.".bright_red()
                        )
                    }
                }
                let _ = std::fs::remove_file(save_path);
                pause()?;
                return Ok(());
            }
            GameStatus::Playing => {}
        }

        print_board(&game);
        print_help_brief(&game);

        print!(
            "{} {} ",
            format!("[{}]", game.current().name).bold().bright_cyan(),
            "command:".bright_black()
        );
        io::stdout().flush()?;
        let input = read_line()?;
        let cmd = input.trim().to_lowercase();
        let mut parts = cmd.split_whitespace();
        let verb = parts.next().unwrap_or("");

        match verb {
            "" => continue,
            "h" | "help" | "?" => {
                print_help_full();
            }
            "s" | "save" => {
                save::save_game(&game, save_path)?;
                println!("Saved to {}.", save_path.display());
            }
            "q" | "quit" | "exit" => {
                print!("Save before quitting? [Y/n]: ");
                io::stdout().flush()?;
                let ans = read_line()?.trim().to_lowercase();
                if ans.is_empty() || ans == "y" || ans == "yes" {
                    save::save_game(&game, save_path)?;
                    println!("Saved to {}.", save_path.display());
                }
                return Ok(());
            }
            "j" | "jester" | "solo" => {
                if !game.can_use_solo_jester() {
                    println!("No solo jesters available.");
                    continue;
                }
                match game.use_solo_jester(&mut rng) {
                    Ok(msgs) => {
                        for m in msgs {
                            println!("  {m}");
                        }
                        save::save_game(&game, save_path)?;
                    }
                    Err(e) => println!("  {e}"),
                }
            }
            "y" | "yield" => {
                if !game.can_yield() {
                    println!("You cannot yield right now (consecutive yield limit).");
                    continue;
                }
                let report = game.yield_turn()?;
                finish_turn(&mut game, report, save_path, &mut rng, None)?;
            }
            "p" | "play" => {
                let rest: String = parts.collect::<Vec<_>>().join(" ");
                let indices = if rest.is_empty() {
                    print!("Card numbers (e.g. 1  or  1 3  or  2 5): ");
                    io::stdout().flush()?;
                    parse_indices(&read_line()?)?
                } else {
                    parse_indices(&rest)?
                };
                handle_play(&mut game, &indices, save_path, &mut rng)?;
            }
            other if other.chars().next().is_some_and(|c| c.is_ascii_digit()) => {
                let indices = parse_indices(&cmd)?;
                handle_play(&mut game, &indices, save_path, &mut rng)?;
            }
            _ => {
                println!("Unknown command. Type `help` for commands.");
            }
        }
    }
}

fn handle_play(
    game: &mut Game,
    indices: &[usize],
    save_path: &PathBuf,
    rng: &mut StdRng,
) -> Result<()> {
    match game.play_cards(indices, rng) {
        Ok(report) => {
            let next_choice = if report.chose_next {
                Some(ask_next_player(game)?)
            } else {
                None
            };
            finish_turn(game, report, save_path, rng, next_choice)?;
        }
        Err(e) => {
            println!("  Cannot play: {e}");
        }
    }
    Ok(())
}
