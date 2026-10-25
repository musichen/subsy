# Security Policy

## Reporting a vulnerability

Open a GitHub issue in this repository for non-sensitive reports. For a sensitive vulnerability, contact the maintainer privately first through the GitHub profile at https://github.com/musichen instead of posting details publicly.

## Notes for users

- Your subscription data lives in a local SQLite database (`subsy.db` next to the CLI binary, or the platform app-data dir for Tauri; override with `SUBSY_DATA_DIR`). Back it up like any personal finance data, and do not commit it.
- Exports (YAML, JSON, CSV, Markdown) contain the same data in plain text. Treat export files as private.
