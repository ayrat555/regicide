# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-07-25

### Changed

- Clearer terminal spacing and separators around the board, turn feedback, banners, and prompts

### Fixed

- Solo jester drained the entire tavern instead of refilling only to max hand size

## [0.2.0] - 2026-07-21

### Added

- Single-letter command shortcuts and in-game rules access during play
- Clearer feedback when a move is invalid
- GitHub Actions CI for build, test, and clippy

### Fixed

- Clippy warnings across the UI and game helpers

## [0.1.0] - 2026-07-19

### Added

- Regicide game engine: setup, combat, suit powers, and win/lose conditions
- ASCII card rendering with ANSI colors
- Interactive terminal UI: main menu, solo and hot-seat multiplayer, play loop
- JSON save and continue for in-progress games
- In-game rules summary and README with build/play docs

[0.2.1]: https://github.com/ayrat555/regicide/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/ayrat555/regicide/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/ayrat555/regicide/releases/tag/0.1.0
