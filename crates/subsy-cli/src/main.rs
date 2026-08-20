use anyhow::Result;
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use subsy_core::{
    calc, export, import, model::*, paths, Config, Payment, Store, Theme,
};

#[derive(Parser)]
#[command(name = "subsy", version, about = "local-first subscription tracker")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    #[arg(long, global = true)]
    db: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Cmd {
    List {
        #[arg(long)]
        json: bool,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        tag: Option<String>,
    },
    Add {
        name: String,
        #[arg(long)] provider: Option<String>,
        #[arg(long)] account: Option<String>,
        #[arg(long)] plan: Option<String>,
        #[arg(long)] price: Option<String>,
        #[arg(long)] currency: Option<String>,
        #[arg(long, value_parser = parse_cycle, alias = "cycle")]
        billing_cycle: Option<BillingCycle>,
        #[arg(long, value_parser = parse_status)]
        status: Option<Status>,
        #[arg(long)] category: Option<String>,
        #[arg(long, alias = "method")]
        payment_method: Option<String>,
        #[arg(long)] reminder_days: Option<i64>,
        #[arg(long)] start: Option<String>,
        #[arg(long)] end: Option<String>,
        #[arg(long)] next: Option<String>,
        #[arg(long)] tags: Option<String>,
        #[arg(long)] url: Option<String>,
        #[arg(long)] notes: Option<String>,
    },
    Edit {
        id: String,
        #[arg(long)] name: Option<String>,
        #[arg(long)] provider: Option<String>,
        #[arg(long)] account: Option<String>,
        #[arg(long)] plan: Option<String>,
        #[arg(long)] price: Option<String>,
        #[arg(long)] currency: Option<String>,
        #[arg(long, value_parser = parse_cycle, alias = "cycle")]
        billing_cycle: Option<BillingCycle>,
        #[arg(long, value_parser = parse_status)]
        status: Option<Status>,
        #[arg(long)] category: Option<String>,
        #[arg(long, alias = "method")]
        payment_method: Option<String>,
        #[arg(long)] reminder_days: Option<i64>,
        #[arg(long)] start: Option<String>,
        #[arg(long)] end: Option<String>,
        #[arg(long)] next: Option<String>,
        #[arg(long)] tags: Option<String>,
        #[arg(long)] url: Option<String>,
        #[arg(long)] notes: Option<String>,
    },
    Show { id: String },
    Rm { id: String },
    Search { query: String, #[arg(long)] json: bool },
    Summary { #[arg(long)] json: bool },
    Import { file: PathBuf },
    Export { file: PathBuf },
    Path,
    Tui,
    Doctor,
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
    Payment {
        #[command(subcommand)]
        cmd: PaymentCmd,
    },
}

#[derive(Subcommand)]
enum ConfigCmd {
    Show,
    Set {
        #[arg(long)] currency: Option<String>,
        #[arg(long)] reminder_days: Option<i64>,
        #[arg(long)] theme: Option<String>,
    },
}

#[derive(Subcommand)]
enum PaymentCmd {
    Add {
        subscription_id: String,
        #[arg(long)] amount: String,
        #[arg(long)] currency: Option<String>,
        #[arg(long)] date: String,
        #[arg(long)] notes: Option<String>,
    },
    List { subscription_id: String },
    Rm { id: String },
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
    let config = Config::load(&paths::config_path()).unwrap_or_default();
    match cli.cmd {
        Cmd::Path => {
            println!("app dir: {}", paths::app_dir().display());
            println!("db:      {}", paths::db_path().display());
            println!("config:  {}", paths::config_path().display());
        }
        Cmd::List { json, status, category, tag } => {
            let mut subs = store.list()?;
            if let Some(s) = status {
                subs.retain(|x| x.status.as_str() == s.to_lowercase());
            }
            if let Some(c) = category {
                subs.retain(|x| x.category.as_deref().map(|v| v.to_lowercase() == c.to_lowercase()).unwrap_or(false));
            }
            if let Some(t) = tag {
                subs.retain(|x| x.tags.iter().any(|v| v.to_lowercase() == t.to_lowercase()));
            }
            if json {
                println!("{}", serde_json::to_string_pretty(&subs)?);
            } else {
                print_table(&subs);
            }
        }
        Cmd::Add { name, provider, account, plan, price, currency, billing_cycle, status, category, payment_method, reminder_days, start, end, next, tags, url, notes } => {
            let mut s = Subscription::new(name);
            apply_common(&mut s, provider, account, plan, price, currency, billing_cycle, status, category, payment_method, reminder_days, start, end, next, tags, url, notes, &config);
            s.source = Source::Manual;
            store.add(&s)?;
            println!("added {} ({})", s.name, s.id);
        }
        Cmd::Edit { id, name, provider, account, plan, price, currency, billing_cycle, status, category, payment_method, reminder_days, start, end, next, tags, url, notes } => {
            let uid = uuid::Uuid::parse_str(&id)?;
            let mut s = match store.get(&uid)? {
                Some(x) => x,
                None => anyhow::bail!("not found: {}", id),
            };
            if let Some(n) = name { s.name = n; }
            apply_common(&mut s, provider, account, plan, price, currency, billing_cycle, status, category, payment_method, reminder_days, start, end, next, tags, url, notes, &config);
            s.updated_at = chrono::Utc::now();
            store.update(&s)?;
            println!("updated {}", id);
        }
        Cmd::Show { id } => {
            let uid = uuid::Uuid::parse_str(&id)?;
            match store.get(&uid)? {
                Some(s) => {
                    println!("{}", serde_json::to_string_pretty(&s)?);
                    let payments = store.list_payments(&uid)?;
                    if !payments.is_empty() {
                        println!("\npayments:");
                        for p in payments {
                            println!("  {}  {}  {}", p.date, p.amount, p.currency.as_deref().unwrap_or(""));
                        }
                        println!("paid so far: {}", store.paid_so_far(&uid)?);
                    }
                }
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
                        || s.provider.as_deref().map(|p| p.to_lowercase().contains(&q)).unwrap_or(false)
                        || s.category.as_deref().map(|p| p.to_lowercase().contains(&q)).unwrap_or(false)
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
            let active = subs.iter().filter(|s| matches!(s.status, Status::Active | Status::Trial)).count();
            if json {
                println!("{}", serde_json::json!({"count": count, "active": active, "monthly": m.to_string(), "yearly": y.to_string()}));
            } else {
                println!("active:       {active} / {count}");
                println!("monthly total: {m}");
                println!("yearly total:  {y}");
            }
        }
        Cmd::Import { file } => {
            let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
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
            let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            match ext.as_str() {
                "yaml" | "yml" => export::export_yaml(&subs, &file)?,
                "json" => export::export_json(&subs, &file)?,
                "csv" => export::export_csv(&subs, &file)?,
                _ => anyhow::bail!("unsupported export extension: {ext}"),
            }
            println!("exported {} subscriptions -> {}", subs.len(), file.display());
        }
        Cmd::Tui => println!("launch TUI: cargo run -p subsy-tui --release"),
        Cmd::Doctor => run_doctor(&store, &config)?,
        Cmd::Config { cmd } => match cmd {
            ConfigCmd::Show => println!("{}", toml::to_string_pretty(&config)?),
            ConfigCmd::Set { currency, reminder_days, theme } => {
                let mut c = config;
                if let Some(x) = currency { c.default_currency = Some(x); }
                if let Some(x) = reminder_days { c.default_reminder_days = x; }
                if let Some(x) = theme { c.theme = Theme::from_str(&x); }
                c.save(&paths::config_path())?;
                println!("config saved");
            }
        },
        Cmd::Payment { cmd } => match cmd {
            PaymentCmd::Add { subscription_id, amount, currency, date, notes } => {
                let sid = uuid::Uuid::parse_str(&subscription_id)?;
                let p = Payment {
                    id: uuid::Uuid::new_v4(),
                    subscription_id: sid,
                    date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")?,
                    amount: amount.parse()?,
                    currency: currency.or_else(|| config.default_currency.clone()),
                    notes,
                    created_at: chrono::Utc::now(),
                };
                store.add_payment(&p)?;
                println!("added payment {}", p.id);
            }
            PaymentCmd::List { subscription_id } => {
                let sid = uuid::Uuid::parse_str(&subscription_id)?;
                for p in store.list_payments(&sid)? {
                    println!("{} {} {} {}", p.id, p.date, p.amount, p.currency.as_deref().unwrap_or(""));
                }
                println!("paid so far: {}", store.paid_so_far(&sid)?);
            }
            PaymentCmd::Rm { id } => {
                let uid = uuid::Uuid::parse_str(&id)?;
                if store.delete_payment(&uid)? {
                    println!("removed payment {}", id);
                } else {
                    anyhow::bail!("not found: {}", id);
                }
            }
        },
    }
    Ok(())
}

fn apply_common(
    s: &mut Subscription,
    provider: Option<String>,
    account: Option<String>,
    plan: Option<String>,
    price: Option<String>,
    currency: Option<String>,
    billing_cycle: Option<BillingCycle>,
    status: Option<Status>,
    category: Option<String>,
    payment_method: Option<String>,
    reminder_days: Option<i64>,
    start: Option<String>,
    end: Option<String>,
    next: Option<String>,
    tags: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    config: &Config,
) {
    if provider.is_some() { s.provider = provider; }
    if account.is_some() { s.account = account; }
    if plan.is_some() { s.plan = plan; }
    if let Some(p) = price {
        let mut parts = p.split_whitespace();
        if let Some(n) = parts.next() {
            s.price = n.parse().ok();
            let cur = parts.collect::<Vec<_>>().join(" ");
            if !cur.is_empty() { s.currency = Some(cur); }
        }
    }
    if s.currency.is_none() { s.currency = config.default_currency.clone(); }
    if currency.is_some() { s.currency = currency; }
    if let Some(b) = billing_cycle { s.billing_cycle = b; }
    if let Some(st) = status { s.status = st; }
    if category.is_some() { s.category = category; }
    if payment_method.is_some() { s.payment_method = payment_method; }
    s.reminder_days = reminder_days.or(Some(config.default_reminder_days));
    if let Some(d) = start { s.start_date = NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok(); }
    if let Some(d) = end { s.end_date = NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok(); }
    if let Some(d) = next { s.next_renewal = NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok(); }
    if let Some(t) = tags { s.tags = t.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(); }
    if url.is_some() { s.url = url; }
    if notes.is_some() { s.notes = notes; }
}

fn run_doctor(store: &Store, config: &Config) -> Result<()> {
    println!("subsy doctor");
    let mut ok = true;
    let app_dir = paths::app_dir();
    println!("app dir: {}", app_dir.display());
    if std::fs::create_dir_all(&app_dir).is_ok() {
        println!("  [ok] app dir writable");
    } else {
        println!("  [err] app dir not writable");
        ok = false;
    }
    let db = paths::db_path();
    println!("db:      {}", db.display());
    if db.exists() {
        match store.count() {
            Ok(n) => println!("  [ok] db open, {} subscriptions", n),
            Err(e) => { println!("  [err] db open failed: {e}"); ok = false; }
        }
    } else {
        println!("  [ok] db does not exist yet (will be created on first write)");
    }
    let cfg = paths::config_path();
    println!("config:  {}", cfg.display());
    if cfg.exists() {
        println!("  [ok] config exists");
    } else {
        println!("  [info] config does not exist, using defaults");
    }
    println!("default currency:   {:?}", config.default_currency);
    println!("default reminder:   {} days", config.default_reminder_days);
    println!("theme:              {}", config.theme.as_str());
    if ok { println!("\nall checks passed"); } else { println!("\nproblems found"); }
    Ok(())
}

fn print_table(subs: &[Subscription]) {
    println!(
        "{:<28} {:<14} {:<12} {:<10} {:<12} {:<8}",
        "name", "provider", "category", "cycle", "renewal", "price"
    );
    for s in subs {
        let price = s.price.map(|p| match &s.currency { Some(c) => format!("{p} {c}"), None => p.to_string() }).unwrap_or_else(|| "-".into());
        let renewal = s.next_renewal.map(|d| d.to_string()).unwrap_or_else(|| "-".into());
        println!(
            "{:<28} {:<14} {:<12} {:<10} {:<12} {:<8}",
            truncate(&s.name, 28),
            truncate(s.provider.as_deref().unwrap_or("-"), 14),
            truncate(s.category.as_deref().unwrap_or("-"), 12),
            s.billing_cycle.as_str(),
            renewal,
            price,
        );
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else {
        let mut out: String = s.chars().take(n - 1).collect();
        out.push('…');
        out
    }
}
