use crate::model::*;
use chrono::NaiveDate;
use rust_decimal::Decimal;

pub fn monthly_cost(sub: &Subscription) -> Option<Decimal> {
    let price = sub.price?;
    let monthly = match sub.billing_cycle {
        BillingCycle::Monthly => Some(price),
        BillingCycle::Yearly => Some(price / Decimal::from(12)),
        BillingCycle::Weekly => Some(price * Decimal::from(52) / Decimal::from(12)),
        BillingCycle::OneTime => None,
        BillingCycle::Credits => None,
        BillingCycle::Custom | BillingCycle::Unknown => None,
    };
    monthly
}

pub fn total_monthly(subs: &[Subscription]) -> Decimal {
    subs.iter().filter_map(monthly_cost).sum()
}

pub fn total_yearly(subs: &[Subscription]) -> Decimal {
    subs.iter().filter_map(monthly_cost).map(|m| m * Decimal::from(12)).sum()
}

pub fn days_until_next_renewal(sub: &Subscription, today: NaiveDate) -> Option<i64> {
    sub.next_renewal.map(|d| (d - today).num_days())
}

pub fn is_ending_soon(sub: &Subscription, today: NaiveDate, threshold_days: i64) -> bool {
    if !matches!(sub.status, Status::Active | Status::PreCanceled | Status::Trial) {
        return false;
    }
    match days_until_next_renewal(sub, today) {
        Some(n) => n <= threshold_days && n >= 0,
        None => false,
    }
}
