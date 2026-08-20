# SQLite Schema

File: `crates/subsy-core/src/schema.sql`

## Migrations

Schema version tracked via `PRAGMA user_version`. Current version: **1**.

Migration from v0 adds:
- `category`, `payment_method`, `reminder_days` columns on `subscriptions`
- `payments` table

## Tables

### `subscriptions`

Primary entity. Columns match `Subscription` domain model.

Indexes:
- `idx_subs_status`
- `idx_subs_renewal`
- `idx_subs_name`
- `idx_subs_category`

### `payments`

Historical payments per subscription.

Indexes:
- `idx_payments_sub`
- `idx_payments_date`

## Storage location

Next to the binary: `subsy.db`. Override with `SUBSY_DATA_DIR`.
