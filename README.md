# Subsy

![subsy logo](assets/subsy-logo.png)

Manage all your digital Subscriptions / auto discovery / auto reminders / Deals Registry

## Requirements

- A Rust toolchain (`cargo`) for the CLI and TUI. Install it with [rustup](https://rustup.rs/).
- Node.js and `npm` only for the optional Tauri desktop app under `crates/subsy-tauri`.

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

## CLI commands

The `subsy` binary (clap subcommands in `crates/subsy-cli/src/main.rs`) provides:

| Command | Description |
| --- | --- |
| `subsy list [--json] [--status <s>] [--category <c>] [--tag <t>]` | List subscriptions, optionally filtered |
| `subsy add <name> [--provider <p>] [--price <p>] [--currency <c>] [--cycle <c>]` | Add a subscription |
| `subsy edit <id> [--name <n>] [--price <p>] [--status <s>]` | Edit a subscription by id |
| `subsy show <id>` / `subsy rm <id>` | Show or remove one subscription |
| `subsy search <query> [--json]` | Search subscriptions |
| `subsy summary [--json]` | Totals and summary (also in Quick start) |
| `subsy import <file>` / `subsy export <file>` | Import or export YAML, JSON, CSV, or Markdown |
| `subsy path` | Print the data file location |
| `subsy tui` | Launch the terminal UI (same as `cargo run -p subsy-tui`) |
| `subsy doctor` | Diagnose the local setup |
| `subsy config show|set` | Show or set currency, reminder days, and theme |
| `subsy payment add ...` | Record a payment for a subscription |

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

## License

MIT. See [LICENSE](LICENSE).
