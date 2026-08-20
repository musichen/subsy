# ADR-0001: Rust workspace with platform-agnostic core

- **Date:** 2026-08-20
- **Status:** Accepted
- **Ticket:** #3, #4

## Context

Subsy must run as a fast local-first CLI/TUI today and become a Tauri desktop/mobile/web app tomorrow. The core logic and storage must not depend on any one surface.

## Decision

Use a Rust workspace:

- `subsy-core` — domain model, SQLite storage, import/export, calculations, config. No terminal or web dependencies.
- `subsy-cli` — `clap` CLI binary.
- `subsy-tui` — `ratatui` + `crossterm` binary.
- `subsy-tauri` (future) — Tauri 2.0 shell with React + Tailwind frontend.

## Consequences

- Core compiles to native library, WASM, or Tauri commands.
- Single static binaries stay small.
- UI surfaces are thin; all business rules live in one place.

## Alternatives considered

- Go + Bubble Tea — good, but Rust's WASM/Tauri path and binary size win.
- TypeScript + Bun — faster web, worse single-binary story and memory footprint.
