# Domain Model

Source of truth: `crates/subsy-core/src/model.rs`.

## `Subscription`

| Field | Type | Notes |
|-------|------|-------|
| `id` | `Uuid` | Generated on creation. |
| `name` | `String` | Display name, e.g. "ChatGPT Plus". |
| `provider` | `Option<String>` | Company/service. |
| `account` | `Option<String>` | Email/username. |
| `plan` | `Option<String>` | Tier/SKU. |
| `price` | `Option<Decimal>` | Amount per billing period. |
| `currency` | `Option<String>` | ISO code. |
| `billing_cycle` | `BillingCycle` | `monthly` / `yearly` / `weekly` / `onetime` / `credits` / `custom`. |
| `status` | `Status` | `active` / `precanceled` / `ended` / `trial` / `unknown`. |
| `category` | `Option<String>` | Single bucket, e.g. `ai`, `entertainment`. |
| `payment_method` | `Option<String>` | e.g. `Visa`, `PayPal`. |
| `reminder_days` | `Option<i64>` | Days before renewal to remind. |
| `start_date` | `Option<NaiveDate>` | First payment date. |
| `end_date` | `Option<NaiveDate>` | When a preCanceled/trial/ended sub stops. |
| `next_renewal` | `Option<NaiveDate>` | Next charge date. |
| `credits_remaining` | `Option<u64>` | Token/credit balance. |
| `url` | `Option<String>` | Dashboard URL. |
| `notes` | `Option<String>` | Free text. |
| `tags` | `Vec<String>` | Many labels. |
| `source` | `Source` | `manual` / `import` / `discovered` / `extension`. |

## `Payment`

| Field | Type | Notes |
|-------|------|-------|
| `id` | `Uuid` | Generated. |
| `subscription_id` | `Uuid` | Parent subscription. |
| `date` | `NaiveDate` | When paid. |
| `amount` | `Decimal` | Paid amount. |
| `currency` | `Option<String>` | ISO code. |
| `notes` | `Option<String>` | Free text. |
