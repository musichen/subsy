use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Active,
    PreCanceled,
    Ended,
    Trial,
    #[default]
    Unknown,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::PreCanceled => "precanceled",
            Status::Ended => "ended",
            Status::Trial => "trial",
            Status::Unknown => "unknown",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => Status::Active,
            "precanceled" => Status::PreCanceled,
            "ended" => Status::Ended,
            "trial" => Status::Trial,
            _ => Status::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum BillingCycle {
    Monthly,
    Yearly,
    Weekly,
    OneTime,
    Credits,
    Custom,
    #[default]
    Unknown,
}

impl BillingCycle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BillingCycle::Monthly => "monthly",
            BillingCycle::Yearly => "yearly",
            BillingCycle::Weekly => "weekly",
            BillingCycle::OneTime => "onetime",
            BillingCycle::Credits => "credits",
            BillingCycle::Custom => "custom",
            BillingCycle::Unknown => "unknown",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "monthly" => BillingCycle::Monthly,
            "yearly" => BillingCycle::Yearly,
            "weekly" => BillingCycle::Weekly,
            "onetime" | "one_time" | "one-time" => BillingCycle::OneTime,
            "credits" => BillingCycle::Credits,
            "custom" => BillingCycle::Custom,
            _ => BillingCycle::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    Manual,
    Import,
    Discovered,
    Extension,
    #[default]
    Unknown,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Manual => "manual",
            Source::Import => "import",
            Source::Discovered => "discovered",
            Source::Extension => "extension",
            Source::Unknown => "unknown",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "manual" => Source::Manual,
            "import" => Source::Import,
            "discovered" => Source::Discovered,
            "extension" => Source::Extension,
            _ => Source::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub name: String,
    pub provider: Option<String>,
    pub account: Option<String>,
    pub plan: Option<String>,
    pub price: Option<Decimal>,
    pub currency: Option<String>,
    pub billing_cycle: BillingCycle,
    pub status: Status,
    pub category: Option<String>,
    pub payment_method: Option<String>,
    pub reminder_days: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub next_renewal: Option<NaiveDate>,
    pub credits_remaining: Option<u64>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub source: Source,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub currency: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Subscription {
    pub fn new(name: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            provider: None,
            account: None,
            plan: None,
            price: None,
            currency: None,
            billing_cycle: BillingCycle::Monthly,
            status: Status::Active,
            category: None,
            payment_method: None,
            reminder_days: None,
            start_date: None,
            end_date: None,
            next_renewal: None,
            credits_remaining: None,
            url: None,
            notes: None,
            tags: vec![],
            source: Source::Manual,
            created_at: now,
            updated_at: now,
        }
    }
}
