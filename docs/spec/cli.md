# CLI Surface

Binary: `subsy`

## Global flag

- `--db <path>` — use a different SQLite file.

## Commands

| Command | Purpose |
|---------|---------|
| `path` | Show app dir, db, config paths. |
| `list [--json] [--status X] [--category Y] [--tag Z]` | List subscriptions with filters. |
| `add <name> [flags]` | Add a subscription. |
| `edit <id> [flags]` | Edit a subscription. |
| `show <id>` | Show JSON + payment history. |
| `rm <id>` | Delete a subscription. |
| `search <query> [--json]` | Search name/provider/category/tags. |
| `summary [--json]` | Active count + monthly/yearly totals. |
| `import <file>` | Import YAML/JSON/CSV/Markdown. |
| `export <file>` | Export YAML/JSON/CSV. |
| `doctor` | Health check. |
| `config show` | Print current config. |
| `config set [--currency USD] [--reminder-days 7] [--theme dark]` | Set defaults. |
| `payment add --subscription-id <id> --amount 20 --date 2026-08-20 [--currency USD]` | Record a payment. |
| `payment list <subscription_id>` | List payments for a subscription. |
| `payment rm <id>` | Delete a payment. |
| `tui` | Print hint to launch TUI binary. |

## Common add/edit flags

`--provider`, `--account`, `--plan`, `--price`, `--currency`, `--billing-cycle` / `--cycle`, `--status`, `--category`, `--payment-method` / `--method`, `--reminder-days`, `--start`, `--end`, `--next`, `--tags`, `--url`, `--notes`.
