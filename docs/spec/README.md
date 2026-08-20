# Specs

Phase-1 specs for Subsy MVP. Each file is the source of truth that code follows (SDD).

| Spec | Decides | Ticket |
|------|---------|--------|
| `domain-model.md` | Subscription struct, enums, optionality | #5 |
| `schema.md` | SQLite schema, indexes, migrations | #5 |
| `calculations.md` | Spend/renewal/ending-soon math | #6 |
| `config.md` | XDG paths + `config.toml` | #7 |
| `import-export.md` | YAML/JSON/CSV/Markdown contracts | #8 |
| `cli.md` | Command surface (`add`/`list`/`import`/`doctor`…) | #9 |
| `tui-ia.md` | Screens + keyboard navigation | #10 |
| `tui-visual.md` | Status, theming, charts | #11 |
| `doctor-and-errors.md` | Validation + `subsy doctor` | #12 |
| `fixtures.md` | Acceptance corpus | #13 |

`docs/SUBSY-SPEC.md` is the loose input spec; these files supersede it once frozen.
Map: https://github.com/musichen/subsy/issues/1
