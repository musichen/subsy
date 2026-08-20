# ADR-0003: Categories, payment methods, and reminders are first-class fields

- **Date:** 2026-08-20
- **Status:** Accepted
- **Tickets:** #14, #15, #16

## Context

Subby shows category chips, payment method badges, and per-subscription reminder rules. Subsy needs these for insights and UX parity.

## Decision

- `category` — single optional string on `Subscription` (distinct from many `tags`).
- `payment_method` — single optional string (e.g. `Visa`, `PayPal`).
- `reminder_days` — optional integer, default from config.
- `payments` table — historical payments per subscription, with `paid_so_far` aggregate.

## Consequences

- Dashboard can show spend by category.
- Detail view can show total cost of ownership.
- Reminders can be surfaced in TUI/CLI and later OS notifications.

## Alternatives considered

- Category as a tag convention (`category:ai`) — simpler but harder to query and display.
- Payment history derived from billing cycle — inaccurate for one-time top-ups and trials.
