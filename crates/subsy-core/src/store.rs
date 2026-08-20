use crate::model::*;
use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use uuid::Uuid;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("creating data dir {}", parent.display())
            })?;
        }
        let conn = Connection::open(path)
            .with_context(|| format!("opening db {}", path.display()))?;
        conn.execute_batch(include_str!("schema.sql"))?;
        Ok(Self { conn })
    }

    pub fn open_default() -> Result<Self> {
        let p = crate::paths::db_path();
        Self::open(&p)
    }

    pub fn add(&self, sub: &Subscription) -> Result<()> {
        self.conn.execute(
            r#"INSERT INTO subscriptions
            (id, name, provider, account, plan, price, currency, billing_cycle, status,
             start_date, end_date, next_renewal, credits_remaining, url, notes, tags, source,
             created_at, updated_at)
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)"#,
            params![
                sub.id.to_string(),
                sub.name,
                sub.provider,
                sub.account,
                sub.plan,
                sub.price.map(|d| d.to_string()),
                sub.currency,
                sub.billing_cycle.as_str(),
                sub.status.as_str(),
                sub.start_date.map(|d| d.to_string()),
                sub.end_date.map(|d| d.to_string()),
                sub.next_renewal.map(|d| d.to_string()),
                sub.credits_remaining.map(|c| c as i64),
                sub.url,
                sub.notes,
                sub.tags.join(","),
                sub.source.as_str(),
                sub.created_at.to_rfc3339(),
                sub.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Subscription>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, provider, account, plan, price, currency, billing_cycle, status,
                    start_date, end_date, next_renewal, credits_remaining, url, notes, tags, source,
                    created_at, updated_at
             FROM subscriptions ORDER BY name COLLATE NOCASE",
        )?;
        let rows = stmt.query_map([], row_to_sub)?;
        let mut out = vec![];
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn get(&self, id: &Uuid) -> Result<Option<Subscription>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, provider, account, plan, price, currency, billing_cycle, status,
                    start_date, end_date, next_renewal, credits_remaining, url, notes, tags, source,
                    created_at, updated_at
             FROM subscriptions WHERE id = ?1",
        )?;
        let r = stmt
            .query_row(params![id.to_string()], row_to_sub)
            .optional()?;
        Ok(r)
    }

    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let n = self
            .conn
            .execute("DELETE FROM subscriptions WHERE id = ?1", params![id.to_string()])?;
        Ok(n > 0)
    }

    pub fn update(&self, sub: &Subscription) -> Result<bool> {
        let n = self.conn.execute(
            r#"UPDATE subscriptions SET
                name=?2, provider=?3, account=?4, plan=?5, price=?6, currency=?7,
                billing_cycle=?8, status=?9, start_date=?10, end_date=?11, next_renewal=?12,
                credits_remaining=?13, url=?14, notes=?15, tags=?16, source=?17, updated_at=?18
              WHERE id = ?1"#,
            params![
                sub.id.to_string(),
                sub.name,
                sub.provider,
                sub.account,
                sub.plan,
                sub.price.map(|d| d.to_string()),
                sub.currency,
                sub.billing_cycle.as_str(),
                sub.status.as_str(),
                sub.start_date.map(|d| d.to_string()),
                sub.end_date.map(|d| d.to_string()),
                sub.next_renewal.map(|d| d.to_string()),
                sub.credits_remaining.map(|c| c as i64),
                sub.url,
                sub.notes,
                sub.tags.join(","),
                sub.source.as_str(),
                sub.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(n > 0)
    }

    pub fn count(&self) -> Result<i64> {
        let n: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM subscriptions", [], |r| r.get(0))?;
        Ok(n)
    }
}

fn row_to_sub(r: &rusqlite::Row) -> rusqlite::Result<Subscription> {
    let id_str: String = r.get(0)?;
    let id = Uuid::parse_str(&id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let price: Option<String> = r.get(5)?;
    let price = price
        .as_deref()
        .and_then(|s| s.parse::<rust_decimal::Decimal>().ok());
    let bc: String = r.get(7)?;
    let status: String = r.get(8)?;
    let start: Option<String> = r.get(9)?;
    let end: Option<String> = r.get(10)?;
    let nxt: Option<String> = r.get(11)?;
    let credits: Option<i64> = r.get(12)?;
    let tags: String = r.get(15)?;
    let source: String = r.get(16)?;
    let created: String = r.get(17)?;
    let updated: String = r.get(18)?;
    Ok(Subscription {
        id,
        name: r.get(1)?,
        provider: r.get(2)?,
        account: r.get(3)?,
        plan: r.get(4)?,
        price,
        currency: r.get(6)?,
        billing_cycle: BillingCycle::from_str(&bc),
        status: Status::from_str(&status),
        start_date: start.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        end_date: end.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        next_renewal: nxt.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        credits_remaining: credits.map(|c| c as u64),
        url: r.get(13)?,
        notes: r.get(14)?,
        tags: if tags.is_empty() { vec![] } else { tags.split(',').map(|s| s.to_string()).collect() },
        source: Source::from_str(&source),
        created_at: chrono::DateTime::parse_from_rfc3339(&created)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}
