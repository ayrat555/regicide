use colored::Colorize;

fn main() {
    if let Err(e) = regicide::ui::run() {
        eprintln!("{} {:#}", "error:".bright_red().bold(), e);
        std::process::exit(1);
    }
}
