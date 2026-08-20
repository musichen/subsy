# Research — Rust stack spike for Subsy single-binary TUI

> Ticket: https://github.com/musichen/subsy/issues/3 · Map: https://github.com/musichen/subsy/issues/1
> Status: **stub — subagent will replace with primary-source findings**

## Question

Does Rust + ratatui/crossterm/clap/rusqlite/serde/XDG satisfy Subsy non-negotiables (single static binary, <50ms start, SQLite at `~/.local/share/subsy/subsy.db` via XDG, serde TOML/JSON/YAML, clap CLI, ratatui UI like lazygit/k9s) without hidden weight or platform traps? What versions/features and fallback choices keep the binary lean while preserving the WASM/Tauri path?

## Findings

_TBD by research subagent — per-crate findings with primary-source citations._

- ratatui + crossterm —
- clap —
- rusqlite + bundled SQLite —
- serde / serde_json / toml / serde_yaml —
- directories / dirs —
- chrono vs time —
- binary size + startup —

## Recommendation (what to ADR)

_TBD_

## Sources

_TBD — every claim cites the doc/page that owns it._
