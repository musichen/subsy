**Technical Specification: Subsy**

**Repository:** `github.com/musichen/subsy`  
**Status:** Open Source  
**Primary goal:** Extremely lightweight, local-first subscription discovery + management tool with a strong developer experience.

---

### 1. Vision

Subsy is a **local-first, privacy-respecting** tool that helps users **discover, track, visualize, and get reminded** about all their digital subscriptions (especially AI/LLM, SaaS, tools, etc.).

It starts as a beautiful **TUI/CLI** (highest priority for developers) and is designed so the same core logic can later power:

- Web dashboard
- Chrome extension / widget
- Desktop app
- Mobile

**Core philosophy:**
- Super lightweight
- Local-first (no account required)
- Privacy by default
- Extensible discovery
- Beautiful terminal experience first

---

### 2. Core Principles

1. **Core is platform-agnostic** — pure business logic + storage.
2. **Start with TUI/CLI** — this is the first and most important surface.
3. **Extremely lightweight** — single binary preferred, minimal dependencies.
4. **Local-first** — all data stays on the user’s machine by default.
5. **Discovery > Manual entry** — the tool should help *find* subscriptions, not just store what the user already knows.
6. **Progressive enhancement** — core works offline and with zero external services.

---

### 3. Recommended Tech Stack (Lightweight)

**Primary recommendation (strongly preferred):**

| Layer              | Choice                          | Reason |
|--------------------|----------------------------------|------|
| Language           | **Rust**                        | Smallest binaries, excellent performance, great for CLI tools |
| TUI                | **ratatui** + crossterm         | Modern, fast, beautiful terminal UIs |
| CLI parsing        | **clap**                        | Standard, powerful |
| Storage            | **SQLite** (via `rusqlite`)     | Zero-config, single file, fast, reliable |
| Serialization      | `serde` + `serde_json` / `toml` | |
| Date/Time          | `chrono` or `time`              | |
| Charts (TUI)       | `ratatui` widgets + simple ascii/unicode charts | Keep it light |
| Config             | XDG + TOML                      | |

**Why Rust over Go/TypeScript?**
- Smallest binary size and memory footprint
- Best long-term path to WASM (web) and Tauri (desktop)
- Matches “super lightweight” requirement best

**Acceptable alternatives** (if team strongly prefers):
- Go + Bubble Tea / Lip Gloss (Charm ecosystem) — also excellent
- TypeScript + Bun + Ink — only if web/Chrome extension must come very early

---

### 4. Domain Model (Core)

```rust
struct Subscription {
    id: Uuid,
    name: String,                    // "ChatGPT Plus", "Cursor Pro"
    provider: String,                // "OpenAI", "Cursor", "ElevenLabs"
    account: Option<String>,         // email or username
    plan: Option<String>,            // "Plus", "Starter", "Moderator"
    price: Option<Decimal>,
    currency: Option<String>,        // "USD", "EUR"
    billing_cycle: Option<BillingCycle>, // Monthly, Yearly, OneTime, Credits
    status: Status,                  // Active, PreCanceled, Ended, Trial, Unknown
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,     // or next renewal
    next_renewal: Option<NaiveDate>,
    credits_remaining: Option<u64>,  // for token-based services
    url: Option<String>,             // dashboard / spending page
    notes: Option<String>,
    tags: Vec<String>,               // ["ai", "llm", "tokens"]
    source: Source,                  // Manual, Import, Discovered, Extension
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

**Status values:**
- `Active`
- `PreCanceled` (canceled but still active until date)
- `Ended`
- `Trial`
- `Unknown`

**BillingCycle:**
- `Monthly`, `Yearly`, `Weekly`, `OneTime`, `Credits`, `Custom`

---

### 5. Features — Phased

#### Phase 1 — Core + TUI (MVP) — **Start here**

**Must have:**
- Full CRUD for subscriptions
- Local SQLite storage (`~/.local/share/subsy/subsy.db` or XDG)
- Beautiful TUI with:
  - Main list view (sortable, filterable)
  - Detail view
  - Calendar / upcoming renewals view
  - Simple spend overview (monthly/yearly total)
  - Status indicators (✅ active, ❌ ended, ⚠️ ending soon)
- Import from YAML / JSON / CSV / simple Markdown
- Export
- Search + tag filtering
- Basic reminders in TUI (highlight ending soon)
- Keyboard-driven, vim-like navigation where sensible

**Nice to have in Phase 1:**
- ASCII/Unicode charts (spend by category, monthly cost)
- Color themes (including high-contrast / monochrome)
- `subsy doctor` / health check

#### Phase 2 — Discovery Engine

- Built-in catalog of common AI/SaaS providers (Cursor, OpenAI, Anthropic, ElevenLabs, Kimi, MiniMax, etc.)
- Ability to mark a subscription as “discovered”
- Simple heuristics / URL pattern matching
- Import from browser history or bookmarks (optional)
- Later: browser extension can push discovered subscriptions to the core

#### Phase 3 — Multi-platform surfaces

- Web (WASM or lightweight backend + frontend)
- Chrome extension / side panel widget
- Desktop (Tauri recommended)
- Mobile (later)

---

### 6. TUI Design Goals (very important)

The TUI should feel like a modern developer tool (think `lazygit`, `k9s`, `bottom`, `yazi`).

**Recommended screens:**

1. **Dashboard** (default)
   - Summary cards: Total monthly / yearly cost, Active count, Ending soon
   - List of subscriptions (table)
   - Quick filters (Active / Ending soon / AI / All)

2. **Calendar / Timeline**
   - Monthly calendar highlighting renewal dates
   - Or vertical timeline of upcoming renewals

3. **Detail / Edit**
4. **Add / Discover**
5. **Spend Analytics** (simple charts)

Navigation should be fast and keyboard-first.

---

### 7. Data & Example Support

The tool must comfortably handle data like the example provided:

- Multiple accounts per service (webboxes.com@gmail.com vs webboxes.eu@gmail.com)
- Pre-canceled status with remaining time
- Credits remaining (ElevenLabs)
- Referral relationships
- Mixed pricing models (subscription + credits)
- Future end dates and reset dates

Import format should support a human-friendly YAML or Markdown structure that matches how people currently track this in notes.

---

### 8. Repository Structure (suggested)

```
subsy/
├── Cargo.toml
├── crates/
│   ├── subsy-core/          # Pure domain logic + storage
│   ├── subsy-cli/           # clap + entrypoint
│   └── subsy-tui/           # ratatui application
├── examples/
├── docs/
├── .github/
└── README.md
```

Later additional crates:
- `subsy-web`
- `subsy-extension`
- etc.

---

### 9. Non-Goals (for v1)

- Cloud sync / accounts (can come later as optional)
- Automatic payment / cancellation
- Heavy scraping of websites
- Complex financial reporting
- Mobile-first design

---

### 10. Success Criteria for MVP

- Single static binary
- Starts in < 50ms
- Can manage 100+ subscriptions smoothly
- Feels excellent in the terminal
- Developers actually enjoy using it daily
- Easy to import existing notes/data

---

### Next Steps for the Coding Agent

1. Create the repository structure with `subsy-core` + `subsy-cli` + `subsy-tui`.
2. Define the full domain model and SQLite schema.
3. Implement basic CRUD + import.
4. Build the first TUI screens (list + detail + upcoming).
5. Make the experience feel polished and fast.

Would you like me to also generate:

- The exact SQLite schema
- Suggested YAML import format based on your example data
- Ratatui screen layout mockups (text-based)
- Or a more detailed Phase 1 task breakdown for the coding agent?


**Technical Specification: Subsy**

**Repository:** `github.com/musichen/subsy`  
**Status:** Open Source  
**Primary goal:** Extremely lightweight, local-first subscription discovery + management tool with a strong developer experience.

---

### 1. Vision

Subsy is a **local-first, privacy-respecting** tool that helps users **discover, track, visualize, and get reminded** about all their digital subscriptions (especially AI/LLM, SaaS, tools, etc.).

It starts as a beautiful **TUI/CLI** (highest priority for developers) and is designed so the same core logic can later power:

- Web dashboard
- Chrome extension / widget
- Desktop app
- Mobile

**Core philosophy:**
- Super lightweight
- Local-first (no account required)
- Privacy by default
- Extensible discovery
- Beautiful terminal experience first

---

### 2. Core Principles

1. **Core is platform-agnostic** — pure business logic + storage.
2. **Start with TUI/CLI** — this is the first and most important surface.
3. **Extremely lightweight** — single binary preferred, minimal dependencies.
4. **Local-first** — all data stays on the user’s machine by default.
5. **Discovery > Manual entry** — the tool should help *find* subscriptions, not just store what the user already knows.
6. **Progressive enhancement** — core works offline and with zero external services.

---

### 3. Recommended Tech Stack (Lightweight)

**Primary recommendation (strongly preferred):**

| Layer              | Choice                          | Reason |
|--------------------|----------------------------------|------|
| Language           | **Rust**                        | Smallest binaries, excellent performance, great for CLI tools |
| TUI                | **ratatui** + crossterm         | Modern, fast, beautiful terminal UIs |
| CLI parsing        | **clap**                        | Standard, powerful |
| Storage            | **SQLite** (via `rusqlite`)     | Zero-config, single file, fast, reliable |
| Serialization      | `serde` + `serde_json` / `toml` | |
| Date/Time          | `chrono` or `time`              | |
| Charts (TUI)       | `ratatui` widgets + simple ascii/unicode charts | Keep it light |
| Config             | XDG + TOML                      | |

**Why Rust over Go/TypeScript?**
- Smallest binary size and memory footprint
- Best long-term path to WASM (web) and Tauri (desktop)
- Matches “super lightweight” requirement best

**Acceptable alternatives** (if team strongly prefers):
- Go + Bubble Tea / Lip Gloss (Charm ecosystem) — also excellent
- TypeScript + Bun + Ink — only if web/Chrome extension must come very early

---

### 4. Domain Model (Core)

```rust
struct Subscription {
    id: Uuid,
    name: String,                    // "ChatGPT Plus", "Cursor Pro"
    provider: String,                // "OpenAI", "Cursor", "ElevenLabs"
    account: Option<String>,         // email or username
    plan: Option<String>,            // "Plus", "Starter", "Moderator"
    price: Option<Decimal>,
    currency: Option<String>,        // "USD", "EUR"
    billing_cycle: Option<BillingCycle>, // Monthly, Yearly, OneTime, Credits
    status: Status,                  // Active, PreCanceled, Ended, Trial, Unknown
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,     // or next renewal
    next_renewal: Option<NaiveDate>,
    credits_remaining: Option<u64>,  // for token-based services
    url: Option<String>,             // dashboard / spending page
    notes: Option<String>,
    tags: Vec<String>,               // ["ai", "llm", "tokens"]
    source: Source,                  // Manual, Import, Discovered, Extension
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

**Status values:**
- `Active`
- `PreCanceled` (canceled but still active until date)
- `Ended`
- `Trial`
- `Unknown`

**BillingCycle:**
- `Monthly`, `Yearly`, `Weekly`, `OneTime`, `Credits`, `Custom`

---

### 5. Features — Phased

#### Phase 1 — Core + TUI (MVP) — **Start here**

**Must have:**
- Full CRUD for subscriptions
- Local SQLite storage (`~/.local/share/subsy/subsy.db` or XDG)
- Beautiful TUI with:
  - Main list view (sortable, filterable)
  - Detail view
  - Calendar / upcoming renewals view
  - Simple spend overview (monthly/yearly total)
  - Status indicators (✅ active, ❌ ended, ⚠️ ending soon)
- Import from YAML / JSON / CSV / simple Markdown
- Export
- Search + tag filtering
- Basic reminders in TUI (highlight ending soon)
- Keyboard-driven, vim-like navigation where sensible

**Nice to have in Phase 1:**
- ASCII/Unicode charts (spend by category, monthly cost)
- Color themes (including high-contrast / monochrome)
- `subsy doctor` / health check

#### Phase 2 — Discovery Engine

- Built-in catalog of common AI/SaaS providers (Cursor, OpenAI, Anthropic, ElevenLabs, Kimi, MiniMax, etc.)
- Ability to mark a subscription as “discovered”
- Simple heuristics / URL pattern matching
- Import from browser history or bookmarks (optional)
- Later: browser extension can push discovered subscriptions to the core

#### Phase 3 — Multi-platform surfaces

- Web (WASM or lightweight backend + frontend)
- Chrome extension / side panel widget
- Desktop (Tauri recommended)
- Mobile (later)

---

### 6. TUI Design Goals (very important)

The TUI should feel like a modern developer tool (think `lazygit`, `k9s`, `bottom`, `yazi`).

**Recommended screens:**

1. **Dashboard** (default)
   - Summary cards: Total monthly / yearly cost, Active count, Ending soon
   - List of subscriptions (table)
   - Quick filters (Active / Ending soon / AI / All)

2. **Calendar / Timeline**
   - Monthly calendar highlighting renewal dates
   - Or vertical timeline of upcoming renewals

3. **Detail / Edit**
4. **Add / Discover**
5. **Spend Analytics** (simple charts)

Navigation should be fast and keyboard-first.

---

### 7. Data & Example Support

The tool must comfortably handle data like the example provided:

- Multiple accounts per service (webboxes.com@gmail.com vs webboxes.eu@gmail.com)
- Pre-canceled status with remaining time
- Credits remaining (ElevenLabs)
- Referral relationships
- Mixed pricing models (subscription + credits)
- Future end dates and reset dates

Import format should support a human-friendly YAML or Markdown structure that matches how people currently track this in notes.

---

### 8. Repository Structure (suggested)

```
subsy/
├── Cargo.toml
├── crates/
│   ├── subsy-core/          # Pure domain logic + storage
│   ├── subsy-cli/           # clap + entrypoint
│   └── subsy-tui/           # ratatui application
├── examples/
├── docs/
├── .github/
└── README.md
```

Later additional crates:
- `subsy-web`
- `subsy-extension`
- etc.

---

### 9. Non-Goals (for v1)

- Cloud sync / accounts (can come later as optional)
- Automatic payment / cancellation
- Heavy scraping of websites
- Complex financial reporting
- Mobile-first design

---

### 10. Success Criteria for MVP

- Single static binary
- Starts in < 50ms
- Can manage 100+ subscriptions smoothly
- Feels excellent in the terminal
- Developers actually enjoy using it daily
- Easy to import existing notes/data

---

### Next Steps for the Coding Agent

1. Create the repository structure with `subsy-core` + `subsy-cli` + `subsy-tui`.
2. Define the full domain model and SQLite schema.
3. Implement basic CRUD + import.
4. Build the first TUI screens (list + detail + upcoming).
5. Make the experience feel polished and fast.



