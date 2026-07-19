use crate::game::Game;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn default_save_path() -> PathBuf {
    if let Some(dir) = dirs::data_local_dir() {
        let folder = dir.join("regicide");
        return folder.join("save.json");
    }
    PathBuf::from("regicide_save.json")
}

pub fn save_game(game: &Game, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create save directory {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(game).context("serialize game")?;
    fs::write(path, json).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub fn load_game(path: &Path) -> Result<Game> {
    let data = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let game: Game = serde_json::from_str(&data).context("parse save file")?;
    Ok(game)
}

pub fn save_exists(path: &Path) -> bool {
    path.is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn roundtrip() {
        let mut rng = StdRng::seed_from_u64(42);
        let game = Game::new(1, &mut rng).unwrap();
        let dir = tempfile_dir();
        let path = dir.join("save.json");
        save_game(&game, &path).unwrap();
        let loaded = load_game(&path).unwrap();
        assert_eq!(loaded.tavern.len(), game.tavern.len());
        assert_eq!(loaded.players[0].hand, game.players[0].hand);
        assert_eq!(loaded.enemy, game.enemy);
    }

    fn tempfile_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("regicide-test-{}", std::process::id()));
        let _ = fs::create_dir_all(&dir);
        dir
    }
}
