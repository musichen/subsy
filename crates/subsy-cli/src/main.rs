use anyhow::Result;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use subsy_core::{calc, export, import, model::*, paths, Store};

#[derive(Parser)]
#[command(name = "subsy", version, about = "local-first subscription tracker")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    /// Use a different DB file (default: $XDG_DATA_HOME/subsy/subsy.db)
    #[arg(long, global = true)]
    db: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Cmd {
    /// List subscriptions
    List {
        /// Show JSON output
        #[arg(long)]
        json: bool,
    },
    /// Add a subscription interactively or from flags
    Add {
        /// Name of the subscription
        name: String,
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        account: Option<String>,
        #[arg(long)]
        plan: Option<String>,
        #[arg(long)]
        price: Option<String>,
        #[arg(long)]
        currency: Option<String>,
        #[arg(long, value_parser = parse_cycle, alias = "cycle")]
        billing_cycle: Option<BillingCycle>,
        #[arg(long, value_parser = parse_status)]
        status: Option<Status>,
        #[arg(long)]
        start: Option<String>,
        #[arg(long)]
        end: Option<String>,
        #[arg(long)]
        next: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Show a single subscription
    Show { id: String },
    /// Remove a subscription
    Rm { id: String },
    /// Search by name (case-insensitive substring)
    Search { query: String, #[arg(long)] json: bool },
    /// Show spend summary
    Summary {
        /// JSON output
        #[arg(long)]
        json: bool,
    },
    /// Import from file
    Import {
        /// Path to file (.yaml/.yml/.json/.md)
        file: PathBuf,
    },
    /// Export to file
    Export {
        /// Path to file (.yaml/.yml/.json/.csv)
        file: PathBuf,
    },
    /// Print where data lives
    Path,
    /// Launch the TUI
    Tui,
}

fn parse_cycle(s: &str) -> Result<BillingCycle, String> {
    Ok(BillingCycle::from_str(s))
}
fn parse_status(s: &str) -> Result<Status, String> {
    Ok(Status::from_str(s))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = match &cli.db {
        Some(p) => Store::open(p)?,
        None => Store::open_default()?,
    };
    match cli.cmd {
        Cmd::Path => {
            println!("app dir: {}", paths::app_dir().display());
            println!("db:      {}", paths::db_path().display());
            println!("config:  {}", paths::config_path().display());
        }
        Cmd::List { json } => {
            let subs = store.list()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&subs)?);
            } else {
                print_table(&subs);
            }
        }
        Cmd::Add {
            name,
            provider,
            account,
            plan,
            price,
            currency,
            billing_cycle,
            status,
            start,
            end,
            next,
            tags,
            url,
            notes,
        } => {
            let mut s = Subscription::new(name);
            s.provider = provider;
            s.account = account;
            s.plan = plan;
            s.price = price.and_then(|p| p.parse().ok());
            s.currency = currency;
            if let Some(b) = billing_cycle {
                s.billing_cycle = b;
            }
            if let Some(st) = status {
                s.status = st;
            }
            s.start_date = start.as_deref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
            s.end_date = end.as_deref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
            s.next_renewal = next.as_deref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
            s.tags = tags
                .map(|t| t.split(',').map(|x| x.trim().to_string()).collect())
                .unwrap_or_default();
            s.url = url;
            s.notes = notes;
            s.source = Source::Manual;
            store.add(&s)?;
            println!("added {} ({})", s.name, s.id);
        }
        Cmd::Show { id } => {
            let uid = uuid::Uuid::parse_str(&id)?;
            match store.get(&uid)? {
                Some(s) => println!("{}", serde_json::to_string_pretty(&s)?),
                None => anyhow::bail!("not found: {}", id),
            }
        }
        Cmd::Rm { id } => {
            let uid = uuid::Uuid::parse_str(&id)?;
            if store.delete(&uid)? {
                println!("removed {}", id);
            } else {
                anyhow::bail!("not found: {}", id);
            }
        }
        Cmd::Search { query, json } => {
            let q = query.to_lowercase();
            let mut hits: Vec<_> = store
                .list()?
                .into_iter()
                .filter(|s| {
                    s.name.to_lowercase().contains(&q)
                        || s.provider
                            .as_deref()
                            .map(|p| p.to_lowercase().contains(&q))
                            .unwrap_or(false)
                        || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
                })
                .collect();
            hits.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            if json {
                println!("{}", serde_json::to_string_pretty(&hits)?);
            } else {
                print_table(&hits);
            }
        }
        Cmd::Summary { json } => {
            let subs = store.list()?;
            let m = calc::total_monthly(&subs);
            let y = calc::total_yearly(&subs);
            let count = subs.len();
            let active = subs
                .iter()
                .filter(|s| matches!(s.status, Status::Active | Status::Trial))
                .count();
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "count": count,
                        "active": active,
                        "monthly": m.to_string(),
                        "yearly": y.to_string(),
                    })
                );
            } else {
                println!("active:       {active} / {count}");
                println!("monthly total: {m}");
                println!("yearly total:  {y}");
            }
        }
        Cmd::Import { file } => {
            let ext = file
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            let subs = match ext.as_str() {
                "yaml" | "yml" => import::import_yaml(&file)?,
                "json" => import::import_json(&file)?,
                "md" | "markdown" => import::import_markdown(&file)?,
                _ => anyhow::bail!("unsupported import extension: {ext}"),
            };
            for s in &subs {
                let _ = store.add(s);
            }
            println!("imported {} subscriptions", subs.len());
        }
        Cmd::Export { file } => {
            let subs = store.list()?;
            let ext = file
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            match ext.as_str() {
                "yaml" | "yml" => export::export_yaml(&subs, &file)?,
                "json" => export::export_json(&subs, &file)?,
                "csv" => export::export_csv(&subs, &file)?,
                _ => anyhow::bail!("unsupported export extension: {ext}"),
            }
            println!("exported {} subscriptions -> {}", subs.len(), file.display());
        }
        Cmd::Tui => {
            println!("launch TUI: cargo run -p subsy-tui --release");
        }
    }
    Ok(())
}

fn print_table(subs: &[Subscription]) {
    println!(
        "{:<28} {:<14} {:<10} {:<10} {:<12} {:<8}",
        "name", "provider", "status", "cycle", "renewal", "price"
    );
    for s in subs {
        let price = s
            .price
            .map(|p| match &s.currency {
                Some(c) => format!("{p} {c}"),
                None => p.to_string(),
            })
            .unwrap_or_else(|| "-".into());
        let renewal = s
            .next_renewal
            .map(|d| d.to_string())
            .unwrap_or_else(|| "-".into());
        println!(
            "{:<28} {:<14} {:<10} {:<10} {:<12} {:<8}",
            truncate(&s.name, 28),
            truncate(s.provider.as_deref().unwrap_or("-"), 14),
            s.status.as_str(),
            s.billing_cycle.as_str(),
            renewal,
            price,
        );
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}
