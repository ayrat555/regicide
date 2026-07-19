use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, Write};

pub fn read_line() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

pub fn pause() -> Result<()> {
    print!("\n{} ", "[Enter]".bright_black());
    io::stdout().flush()?;
    let _ = read_line()?;
    Ok(())
}

pub fn parse_indices(s: &str) -> Result<Vec<usize>> {
    let mut out = Vec::new();
    for part in s.split_whitespace() {
        let n: usize = part
            .parse()
            .map_err(|_| anyhow::anyhow!("not a number: {part}"))?;
        if n == 0 {
            bail!("card numbers are 1-based");
        }
        out.push(n - 1);
    }
    if out.is_empty() {
        bail!("no cards selected");
    }
    Ok(out)
}
