# ADR-0002: App-internal SQLite storage

- **Date:** 2026-08-20
- **Status:** Accepted
- **Ticket:** #7

## Context

Subsy is local-first, OSS, and self-contained. Users should be able to delete the app folder and have all data disappear.

## Decision

- Store `subsy.db` and `subsy-config.toml` in the same directory as the running binary.
- Allow override via `SUBSY_DATA_DIR` for tests or portable installs.
- Use `rusqlite` with bundled SQLite.

## Consequences

- No hidden files in `~/.local/share` or `~/Library`.
- App is fully portable: copy the folder, move it, delete it.
- Backups are just file copies.

## Alternatives considered

- XDG paths (`~/.local/share/subsy`) — standard on Linux, but violates self-contained goal.
- Cloud sync — out of scope for v1.
