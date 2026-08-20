# Import / Export Format

## Supported formats

- **YAML / JSON** — array of `Subscription` objects.
- **CSV** — header row + one row per subscription.
- **Markdown** — `# Title` followed by `## Subscription Name` sections with `- key: value` lines.

## Markdown keys

```md
## ChatGPT Plus
- provider: OpenAI
- account: me@gmail.com
- plan: Plus
- price: 20 USD
- billing_cycle: monthly
- status: active
- category: ai
- payment_method: PayPal
- reminder_days: 7
- start_date: 2024-01-15
- next_renewal: 2026-09-15
- credits_remaining: 0
- url: https://chatgpt.com
- notes: via referral
- tags: ai, llm
```

## Export columns

`name`, `provider`, `account`, `plan`, `price`, `currency`, `billing_cycle`, `status`, `category`, `payment_method`, `reminder_days`, `start_date`, `end_date`, `next_renewal`, `credits_remaining`, `url`, `notes`, `tags`.

Payments are not exported in CSV/Markdown yet; available via CLI `subsy payment` and TUI detail view.
