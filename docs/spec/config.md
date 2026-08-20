# Config

File: `subsy-config.toml` next to the binary.

Override: `SUBSY_DATA_DIR` changes the directory where both DB and config live.

## Fields

```toml
default_currency = "USD"
default_reminder_days = 7
theme = "default" # default | dark | light | high_contrast | monochrome
```

Defaults:
- `default_currency`: none
- `default_reminder_days`: 0 (no reminders)
- `theme`: `default`
