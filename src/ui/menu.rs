use super::input::{pause, read_line};
use super::rules::print_rules;
use super::session::play_loop;
use crate::game::Game;
use crate::save;
use anyhow::Result;
use colored::Colorize;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::io::{self, Write};

pub fn run() -> Result<()> {
    let save_path = save::default_save_path();
    println!();
    println!("{}", "╔══════════════════════════════════╗".bright_yellow());
    println!(
        "{}{}{}",
        "║".bright_yellow(),
        "          R E G I C I D E         ".bold().bright_red(),
        "║".bright_yellow()
    );
    println!(
        "{}{}{}",
        "║".bright_yellow(),
        "     cooperative card battler     ".bright_black(),
        "║".bright_yellow()
    );
    println!("{}", "╚══════════════════════════════════╝".bright_yellow());
    println!();
    println!(
        "{} {}",
        "Save file:".bright_black(),
        save_path.display().to_string().cyan()
    );
    println!();

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
        println!("  {} Rules", "4)".cyan().bold());
        println!("  {} Quit", "5)".cyan().bold());
        print!("\n{} ", ">".bright_yellow().bold());
        io::stdout().flush()?;

        match read_line()?.trim() {
            "1" => {
                let mut rng = StdRng::from_os_rng();
                let game = Game::new(1, &mut rng)?;
                play_loop(game, &save_path)?;
            }
            "2" => {
                print!("Players (2–4): ");
                io::stdout().flush()?;
                let n: usize = read_line()?.trim().parse().unwrap_or(0);
                if !(2..=4).contains(&n) {
                    println!("Invalid player count.\n");
                    continue;
                }
                let mut rng = StdRng::from_os_rng();
                let game = Game::new(n, &mut rng)?;
                play_loop(game, &save_path)?;
            }
            "3" => {
                if !save::save_exists(&save_path) {
                    println!("No save file found.\n");
                    continue;
                }
                let game = save::load_game(&save_path)?;
                println!("Loaded saved game.\n");
                play_loop(game, &save_path)?;
            }
            "4" | "r" | "rules" => {
                print_rules();
                pause()?;
            }
            "5" | "q" | "quit" => {
                println!("Farewell, regicide.");
                return Ok(());
            }
            _ => println!("Unknown option.\n"),
        }
    }
}
