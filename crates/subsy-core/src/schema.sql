CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    provider TEXT,
    account TEXT,
    plan TEXT,
    price TEXT,
    currency TEXT,
    billing_cycle TEXT NOT NULL DEFAULT 'unknown',
    status TEXT NOT NULL DEFAULT 'unknown',
    start_date TEXT,
    end_date TEXT,
    next_renewal TEXT,
    credits_remaining INTEGER,
    url TEXT,
    notes TEXT,
    tags TEXT NOT NULL DEFAULT '',
    source TEXT NOT NULL DEFAULT 'manual',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_subs_status ON subscriptions(status);
CREATE INDEX IF NOT EXISTS idx_subs_renewal ON subscriptions(next_renewal);
CREATE INDEX IF NOT EXISTS idx_subs_name ON subscriptions(name COLLATE NOCASE);
