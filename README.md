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

- DB: `subsy.db` next to the binary (app-internal; delete app = delete data)
- Config: `subsy-config.toml` next to the binary
- Override: `SUBSY_DATA_DIR=/my/path`
