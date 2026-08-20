# Spend, Renewal & Ending-Soon Calculations

## Monthly cost normalization

| Cycle | Formula |
|-------|---------|
| `monthly` | `price` |
| `yearly` | `price / 12` |
| `weekly` | `price * 52 / 12` |
| `onetime` | excluded |
| `credits` | excluded |
| `custom` / `unknown` | excluded |

## Totals

- `total_monthly` = sum of normalized monthly costs.
- `total_yearly` = `total_monthly * 12`.

## Renewal window

- `days_until_next_renewal(sub, today) = next_renewal - today`.
- `is_ending_soon(sub, today, threshold)` is true when status is `active`, `precanceled`, or `trial`, and `0 <= days <= threshold`.

Default reminder threshold is configurable; default 7 days.

## Active count

Counts subscriptions with status `active` or `trial`. `precanceled` and `ended` are excluded.
