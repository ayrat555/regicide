# Regicide (CLI)

```text
╔══════════════════════════════════╗
║          R E G I C I D E         ║
║     cooperative card battler     ║
╚══════════════════════════════════╝
```

A terminal version of [Regicide](https://badgersfrommars.com/en-eu/pages/learn-to-play-regicide), the cooperative card game. Defeat the Jacks, Queens, and Kings with ASCII cards, suit powers, and save/continue support.

## Requirements

- [Rust](https://www.rust-lang.org/) (edition 2024 / recent toolchain)
- A terminal that supports ANSI colors and Unicode box-drawing / suit symbols

## Run

```bash
cargo run --release
```

Set `NO_COLOR=1` for plain (uncolored) output.

## Example

Mid-game solo board (colors omitted):

```text
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Turn 3  │  Tavern 28  │  Discard 2  │  Castle 10
Solo jesters: 2  used: 0

  ENEMY
┌─────┐
│J    │
│  ♠  │
│    J│
└─────┘
  ATK 5 (base 10 - shield 5)  HP ████████░░░░ 13/20
  Immunity: ♠ Spades
  Played: 5♠ 2♥

  YOUR HAND
┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐
│A    │ │5    │ │7    │ │10   │ │3    │ │9    │ │2    │ │4    │
│  ♠  │ │  ♥  │ │  ♣  │ │  ♦  │ │  ♠  │ │  ♥  │ │  ♣  │ │  ♦  │
│    A│ │    5│ │    7│ │   10│ │    3│ │    9│ │    2│ │    4│
└─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘
   1       2       3       4       5       6       7       8   
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ♥ heal  ♦ draw  ♣ double  ♠ shield
  Commands: play (p) <n…> | yield (y) | jester (j) | rules (r) | save (s) | menu (m) | quit (q) | help (h)
```

## Play

From the main menu:

1. **New solo game** - 8-card hand, two side Jesters
2. **New multiplayer** - hot-seat, 2-4 players
3. **Continue saved game**
4. **Rules** - in-game rulebook summary
5. **Quit**

### In-game commands

| Command | Action |
|---------|--------|
| `play` / `p` / `3` | Play card(s) by number |
| `play 1 4` | Animal companion pair, or a combo |
| `yield` / `y` | Skip attack; take enemy damage |
| `jester` / `j` | Solo: discard hand and refill (2 per game) |
| `rules` / `r` | Full rulebook summary |
| `save` / `s` | Write the save file |
| `menu` / `m` | Return to the main menu (prompts to save) |
| `quit` / `q` | Quit the game (prompts to save) |
| `help` / `h` | Command help |

When defending, enter card numbers or `auto` for a suggested discard.

### Suit powers

| Suit | Effect |
|------|--------|
| ♥ Hearts | Move cards from discard under the Tavern |
| ♦ Diamonds | Draw cards |
| ♣ Clubs | Double damage |
| ♠ Spades | Shield (reduce enemy attack) |

Enemies are immune to their own suit’s power unless a Jester cancels immunity.

## Save / continue

Progress autosaves after each turn. The default path is:

```text
~/.local/share/regicide/save.json
```

Choose **Continue saved game** from the main menu to resume.

## Project layout

```text
src/
  main.rs       # entry point
  lib.rs        # crate root
  card/         # suits, ranks, cards, ASCII rendering
  game/         # rules engine (setup, play, defend, powers)
  save.rs       # JSON persistence
  ui/           # menus, board, play session
```

```bash
cargo test
```

## Credits

Regicide was designed by Paul Abrahams, Luke Badger, and Andy Richdale ([Badgers from Mars](https://badgersfrommars.com/)).

Official rules: [Learn to Play](https://badgersfrommars.com/en-eu/pages/learn-to-play-regicide) · [PDF](https://www.regicidegame.com/site_files/33132/upload_files/RegicideRulesA4.pdf)

This is an unofficial fan implementation for personal / educational use.
