use crate::model::*;
use anyhow::Result;
use std::path::Path;

pub fn export_yaml(subs: &[Subscription], path: &Path) -> Result<()> {
    let s = serde_yaml::to_string(subs)?;
    std::fs::write(path, s)?;
    Ok(())
}

pub fn export_json(subs: &[Subscription], path: &Path) -> Result<()> {
    let s = serde_json::to_string_pretty(subs)?;
    std::fs::write(path, s)?;
    Ok(())
}

pub fn export_csv(subs: &[Subscription], path: &Path) -> Result<()> {
    let mut w = csv::Writer::from_path(path)?;
    w.write_record([
        "name",
        "provider",
        "account",
        "plan",
        "price",
        "currency",
        "billing_cycle",
        "status",
        "category",
        "payment_method",
        "reminder_days",
        "start_date",
        "end_date",
        "next_renewal",
        "credits_remaining",
        "url",
        "notes",
        "tags",
    ])?;
    for s in subs {
        w.write_record([
            s.name.as_str(),
            s.provider.as_deref().unwrap_or(""),
            s.account.as_deref().unwrap_or(""),
            s.plan.as_deref().unwrap_or(""),
            &s.price.map(|d| d.to_string()).unwrap_or_default(),
            s.currency.as_deref().unwrap_or(""),
            s.billing_cycle.as_str(),
            s.status.as_str(),
            s.category.as_deref().unwrap_or(""),
            s.payment_method.as_deref().unwrap_or(""),
            &s.reminder_days.map(|d| d.to_string()).unwrap_or_default(),
            &s.start_date.map(|d| d.to_string()).unwrap_or_default(),
            &s.end_date.map(|d| d.to_string()).unwrap_or_default(),
            &s.next_renewal.map(|d| d.to_string()).unwrap_or_default(),
            &s.credits_remaining.map(|c| c.to_string()).unwrap_or_default(),
            s.url.as_deref().unwrap_or(""),
            s.notes.as_deref().unwrap_or(""),
            &s.tags.join(";"),
        ])?;
    }
    w.flush()?;
    Ok(())
}
