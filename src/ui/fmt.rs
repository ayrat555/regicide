//! Shared terminal layout helpers for consistent spacing and separators.

use colored::{Color, Colorize};
use std::fmt::Display;

pub const LINE_WIDTH: usize = 44;

pub fn blank() {
    println!();
}

pub fn hrule() {
    println!("{}", "━".repeat(LINE_WIDTH).bright_black());
}

pub fn thin_rule() {
    println!("{}", "─".repeat(LINE_WIDTH).bright_black());
}

pub fn double_rule() {
    println!("{}", "═".repeat(LINE_WIDTH));
}

/// Center `text` in a field of `width` columns (Unicode char count).
pub fn pad_center(text: &str, width: usize) -> String {
    let len = text.chars().count();
    if len >= width {
        return text.chars().take(width).collect();
    }
    let left = (width - len) / 2;
    let right = width - len - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

/// Box banner used for title / victory / defeat.
///
/// Borders use `border`; the title line uses `title_color`.
pub fn banner(title: &str, subtitle: Option<&str>, border: Color, title_color: Color) {
    let inner = LINE_WIDTH.saturating_sub(2);
    let bar = "═".repeat(inner);
    let top = format!("╔{bar}╗");
    let bottom = format!("╚{bar}╝");
    println!("{}", top.color(border));
    println!(
        "{}{}{}",
        "║".color(border),
        pad_center(title, inner).bold().color(title_color),
        "║".color(border)
    );
    if let Some(sub) = subtitle {
        println!(
            "{}{}{}",
            "║".color(border),
            pad_center(sub, inner).bright_black(),
            "║".color(border)
        );
    }
    println!("{}", bottom.color(border));
}

/// Print success/info event lines with ▸ chrome and blank lines around the block.
pub fn print_event_lines<I, S>(msgs: I)
where
    I: IntoIterator<Item = S>,
    S: Display,
{
    let msgs: Vec<S> = msgs.into_iter().collect();
    if msgs.is_empty() {
        return;
    }
    blank();
    for m in &msgs {
        println!("  {} {m}", "▸".bright_yellow());
    }
    blank();
}
