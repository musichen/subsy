# Contributing to Subsy

## Workspace

Subsy is a Rust workspace. The crates are:

- `crates/subsy-core` - domain model, SQLite, import/export, config
- `crates/subsy-cli` - the `subsy` binary
- `crates/subsy-tui` - ratatui terminal UI
- `crates/subsy-tauri` - Tauri 2 desktop/mobile/web shell (React and Tailwind)

Read [`docs/SUBSY-SPEC.md`](docs/SUBSY-SPEC.md) before changing `subsy-core`.

## Build

```bash
cargo build --release
./target/release/subsy doctor
```

For the Tauri shell:

```bash
cd crates/subsy-tauri
npm install
npm run tauri dev
```

## Pull requests

Create a branch from `main`, keep the change in the smallest crate that owns it, and describe how you verified it (commands you ran, or what you opened in the TUI or Tauri app).
