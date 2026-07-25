use super::fmt::{self, blank};
use super::input::{pause, read_line};
use super::rules::print_rules;
use super::session::{play_loop, SessionEnd};
use crate::game::Game;
use crate::save;
use anyhow::Result;
use colored::{Color, Colorize};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::io::{self, Write};
use std::path::Path;

pub fn run() -> Result<()> {
    let save_path = save::default_save_path();
    blank();
    fmt::banner(
        "R E G I C I D E",
        Some("cooperative card battler"),
        Color::BrightYellow,
        Color::BrightRed,
    );
    blank();
    println!(
        "  {} {}",
        "Save file:".bright_black(),
        save_path.display().to_string().cyan()
    );
    blank();

    loop {
        println!("  {} New solo game", "1)".cyan().bold());
        println!("  {} New multiplayer (hot-seat, 2–4)", "2)".cyan().bold());
        if save::save_exists(&save_path) {
            println!("  {} Continue saved game", "3)".cyan().bold());
        } else {
            println!(
                "  {} Continue saved game {}",
                "3)".cyan().bold(),
                "(none)".bright_black()
            );
        }
        println!("  {} Rules {}", "4)".cyan().bold(), "(r)".bright_black());
        println!("  {} Quit {}", "5)".cyan().bold(), "(q)".bright_black());
        blank();
        print!("{} ", ">".bright_yellow().bold());
        io::stdout().flush()?;

        match read_line()?.trim() {
            "1" => {
                let mut rng = StdRng::from_os_rng();
                let game = Game::new(1, &mut rng)?;
                if start_session(game, &save_path)? {
                    return Ok(());
                }
            }
            "2" => {
                print!("  Players (2–4): ");
                io::stdout().flush()?;
                let n: usize = read_line()?.trim().parse().unwrap_or(0);
                if !(2..=4).contains(&n) {
                    println!("  Invalid player count.");
                    blank();
                    continue;
                }
                let mut rng = StdRng::from_os_rng();
                let game = Game::new(n, &mut rng)?;
                if start_session(game, &save_path)? {
                    return Ok(());
                }
            }
            "3" => {
                if !save::save_exists(&save_path) {
                    println!("  No save file found.");
                    blank();
                    continue;
                }
                let game = save::load_game(&save_path)?;
                println!("  Loaded saved game.");
                blank();
                if start_session(game, &save_path)? {
                    return Ok(());
                }
            }
            "4" | "r" | "rules" => {
                print_rules();
                fmt::thin_rule();
                pause()?;
            }
            "5" | "q" | "quit" => {
                blank();
                println!("  Farewell, regicide.");
                return Ok(());
            }
            _ => {
                println!("  Unknown option.");
                blank();
            }
        }
    }
}

/// Runs a play session. Returns `true` if the app should exit.
fn start_session(game: Game, save_path: &Path) -> Result<bool> {
    if play_loop(game, save_path)? == SessionEnd::QuitApp {
        blank();
        println!("  Farewell, regicide.");
        Ok(true)
    } else {
        blank();
        Ok(false)
    }
}
