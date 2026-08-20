# Subsy — Ubiquitous Language

This file records the canonical terms for Subsy Phase-1. Use these exact words in issue titles, specs, code, and tests.

## Core entities

- **Subscription** — a recurring or one-time charge the user pays for. The central entity.
- **Provider** — the company or service behind the subscription (e.g. OpenAI, Cursor, Netflix).
- **Account** — the user's identity with the provider, usually an email or username.
- **Plan** — the tier or SKU (e.g. Plus, Starter, 200 GB).
- **Price** — the amount paid per billing period.
- **Currency** — ISO code (USD, EUR, etc.).
- **Billing cycle** — how often the charge repeats: `monthly`, `yearly`, `weekly`, `onetime`, `credits`, `custom`.
- **Status** — lifecycle state: `active`, `precanceled`, `ended`, `trial`, `unknown`.
- **Next renewal** — the next date the subscription will charge.
- **End date** — when a `precanceled`, `trial`, or `ended` subscription stops.
- **Credits remaining** — token/credit balance for credit-based services.

## Classification & reminders

- **Category** — a single high-level bucket (e.g. `ai`, `entertainment`, `storage`). Distinct from tags.
- **Tags** — many free-form labels attached to a subscription.
- **Payment method** — how the user pays (e.g. `Visa`, `PayPal`, `Apple Pay`).
- **Reminder days** — how many days before renewal the user wants a reminder.

## Money & history

- **Monthly cost** — normalized equivalent cost per month, used for totals.
- **Yearly cost** — monthly cost × 12.
- **Payment** — a recorded historical charge for a subscription.
- **Paid so far** — sum of recorded payments for a subscription.

## Sources

- **Manual** — entered by the user.
- **Import** — loaded from YAML/JSON/CSV/Markdown.
- **Discovered** — found by the discovery engine (Phase-2+).
- **Extension** — pushed by a browser extension (Phase-2+).

## Synonyms to avoid

- Don't use "service" when you mean **provider** or **subscription**.
- Don't use "billing" as a noun; use **billing cycle** or **renewal**.
- Don't use "cancelled"; use **precanceled** or **ended**.
