# Competitor insight: Subby

Source: `https://play.google.com/store/apps/details?id=com.slapp.subby` (screenshots in `.competitors-reference/subby/ui-screens-subby/`).

## What Subby does well (ideas worth stealing)

### 1. Dashboard as a "money-at-a-glance" surface
- Big monthly/yearly toggle + total spend headliner.
- Active subscription count under the total.
- Mini bar chart per month (Feb→Jul) showing spend trajectory.
- "Next payments" horizontal strip with relative due dates: **Today**, **In 2 days**, **In 5 days**, **In 8 days**.
- "Recent payments" list with checkmark + date.
- Category insight card: "Entertainment is 32% of spend — $55.05 across 4 subscriptions".

### 2. Subscriptions list sorted by urgency
- List rows: logo/letter icon, name, next payment date, payment method, price, billing-cycle badge.
- Highlighted row when due today.
- Sort/filter dropdown: "User defined" (later: sort by date, price, name, status).
- Search over a catalog of 602 brands.

### 3. Detail view shows total cost of ownership
- Price + cycle (e.g. `$139.00 Yearly`).
- Normalized equivalent (`$11.58/mo`, `$139.00/yr`).
- Paid so far (`$139.00 · 1 payment`).
- Next payment / first payment dates.
- Reminder rule (`1 week before`).
- Category + payment method chips.
- Payment history with "View all".

### 4. Insights screen
- Month/Year toggle.
- Tabs: General / Categories / Payment methods.
- Donut: Due vs Paid for selected month.
- Totals row: due this month / paid this month / total this month.
- Bar chart: last 6 months.
- Average monthly / average yearly / projected this year.

### 5. Add flow
- Stepper: Service → Price → Extras.
- Brand catalog search (602 brands, logo/color auto-fill).
- "Add from a screenshot" Pro feature.
- Custom icon via first letter + own color.

### 6. Visual
- Clean card-based layout.
- True-black dark mode.
- Status/date badges with soft colors.
- Large + FAB for add.

## What Subsy should do differently

- **Local-first / no account** — Subby is a mobile app; Subsy starts as TUI/CLI with all data on disk.
- **No bank sync** — keep Phase-1 manual/import only; avoid privacy surface.
- **No AI screenshot scanning** — keep Phase-1 import to YAML/JSON/CSV/Markdown; AI scan is Phase-2 fog.
- **Developer ergonomics** — keyboard-first TUI, scriptable CLI, single static binary.

## Concrete takeaways for Phase-1 specs

| Subby pattern | Subsy Phase-1 translation |
|---------------|---------------------------|
| Monthly/yearly spend toggle | Dashboard summary cards + CLI `--month` / `--year` flags |
| "Active subscriptions" count | Summary stat, status-aware (exclude Ended) |
| Relative due dates (`in N days`, `Today`) | TUI list column + CLI text formatting |
| "Due today" highlight | Status indicator / row background in TUI |
| Payment method chip (`Visa`, `PayPal`, `Apple Pay`) | Add `payment_method` field to domain model? (needs ticket) |
| Category chip (`Entertainment`, `Music`, `Gaming`) | Category vs Tags: decide if category is first-class or just a tag |
| Normalized monthly/yearly in detail | Calculation spec for detail view |
| Paid so far / payment history | Payment history entity: decision needed |
| Reminder rule (`1 week before`) | Reminder/notification contract |
| Last-6-months bar chart | Spend analytics screen (nice-to-have) |
| Dark mode | Theme strategy incl. high-contrast + monochrome |
| Brand catalog search | Discovery engine: Phase-2 fog |

## Open questions to resolve in the Wayfinder map

1. Do we track **payment methods** as a first-class field, or only as free-form notes/tags?
2. Do we track **payment history** (paid-so-far, per-payment log) in Phase-1, or just current/next renewal?
3. Is **category** distinct from **tags**, or is category just a convention like `tag:category/entertainment`?
4. What is the **reminder** model — TUI highlight only, or user-configurable `N days before`?
5. Do we want a **monthly spend trajectory mini-chart** on the dashboard, or keep charts only in the analytics screen?

Map: https://github.com/musichen/subsy/issues/1
