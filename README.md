# Subsy
Manage all your digital Subscriptions / auto discovery / auto reminders / Deals Registry

## Quick start

```bash
cargo build --release
./target/release/subsy path
./target/release/subsy add "ChatGPT Plus" --provider OpenAI --price 20 --currency USD --cycle monthly --next 2026-09-15 --tags ai,llm
./target/release/subsy list
./target/release/subsy summary
./target/release/subsy tui
cargo run --release -p subsy-tui
```

## Import / export

```bash
./target/release/subsy import examples/fixtures/sample.md
./target/release/subsy export subs.json
./target/release/subsy export subs.csv
```

YAML, JSON, CSV, and Markdown are supported.

## Workspace

- `crates/subsy-core` — domain model + SQLite + import/export
- `crates/subsy-cli` — `subsy` binary
- `crates/subsy-tui` — ratatui TUI

## Data

- DB: `~/.local/share/subsy/subsy.db` (override: `XDG_DATA_HOME` or `SUBSY_DATA_DIR`)
- Config: `~/.config/subsy/config.toml` (override: `XDG_CONFIG_HOME` or `SUBSY_CONFIG_DIR`)
