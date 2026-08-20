# Subsy
Manage all your digital Subscriptions / auto discovery / auto reminders / Deals Registry

## Quick start

```bash
cargo build --release
./target/release/subsy path
./target/release/subsy import examples/fixtures/sample.md
./target/release/subsy list
./target/release/subsy summary
./target/release/subsy doctor
cargo run --release -p subsy-tui    # launch TUI
```

## Workspace

- `crates/subsy-core` — domain model + SQLite + import/export + config
- `crates/subsy-cli` — `subsy` binary
- `crates/subsy-tui` — ratatui TUI
- `crates/subsy-tauri` — Tauri 2.0 desktop/mobile/web shell with React + Tailwind

## Tauri app

```bash
cd crates/subsy-tauri
npm install
npm run tauri dev       # desktop dev
npm run tauri android dev
npm run tauri ios dev
npm run tauri build     # produce .app / .dmg / installers
```

The Tauri shell uses the same `subsy-core` crate. Frontend: React 19 + Tailwind CSS v4.

## Data

- CLI/TUI: `subsy.db` next to the binary (app-internal; delete app = delete data)
- Tauri: platform app data dir (e.g. `~/Library/Application Support/dev.musichen.subsy-tauri`)
- Override: `SUBSY_DATA_DIR=/my/path`

## Import / export

YAML, JSON, CSV, and Markdown are supported.

```bash
./target/release/subsy import examples/fixtures/sample.md
./target/release/subsy export subs.json
./target/release/subsy export subs.csv
```
