use crate::model::*;

/// Parse a minimal Markdown tracking format into Subscriptions.
///
/// Format:
///
/// ```md
/// # Subsy
///
/// ## ChatGPT Plus
/// - provider: OpenAI
/// - account: me@gmail.com
/// - plan: Plus
/// - price: 20 USD
/// - billing_cycle: monthly
/// - status: active
/// - start_date: 2024-01-15
/// - next_renewal: 2026-09-15
/// - url: https://chatgpt.com
/// - notes: via referral
/// - tags: ai, llm
///
/// ## ElevenLabs
/// ...
/// ```
pub fn parse_markdown(s: &str) -> Vec<Subscription> {
    let mut out = vec![];
    let mut current: Option<Subscription> = None;

    for line in s.lines() {
        let t = line.trim();
        if let Some(name) = t.strip_prefix("## ") {
            if let Some(c) = current.take() {
                out.push(c);
            }
            current = Some(Subscription::new(name.trim()));
        } else if let Some(item) = current.as_mut() {
            if let Some(rest) = t.strip_prefix("- ") {
                if let Some((k, v)) = rest.split_once(':') {
                    let k = k.trim();
                    let v = v.trim();
                    apply_field(item, k, v);
                }
            }
        }
    }
    if let Some(c) = current.take() {
        out.push(c);
    }
    out
}

fn apply_field(s: &mut Subscription, k: &str, v: &str) {
    if v.is_empty() {
        return;
    }
    match k {
        "provider" => s.provider = Some(v.into()),
        "account" => s.account = Some(v.into()),
        "plan" => s.plan = Some(v.into()),
        "url" => s.url = Some(v.into()),
        "notes" => s.notes = Some(v.into()),
        "currency" => s.currency = Some(v.into()),
        "status" => s.status = Status::from_str(v),
        "billing_cycle" => s.billing_cycle = BillingCycle::from_str(v),
        "source" => s.source = Source::from_str(v),
        "category" => s.category = Some(v.into()),
        "payment_method" => s.payment_method = Some(v.into()),
        "reminder_days" => s.reminder_days = v.parse().ok(),
        "start_date" => s.start_date = parse_date(v),
        "end_date" => s.end_date = parse_date(v),
        "next_renewal" => s.next_renewal = parse_date(v),
        "credits_remaining" => s.credits_remaining = v.parse().ok(),
        "tags" => s.tags = v.split(',').map(|x| x.trim().to_string()).collect(),
        "price" => {
            if let Some((num, cur)) = split_price(v) {
                s.price = Some(num);
                if s.currency.is_none() && !cur.is_empty() {
                    s.currency = Some(cur);
                }
            }
        }
        _ => {}
    }
}

fn split_price(v: &str) -> Option<(rust_decimal::Decimal, String)> {
    let mut parts = v.split_whitespace();
    let n = parts.next()?.parse::<rust_decimal::Decimal>().ok()?;
    let cur = parts.collect::<Vec<_>>().join(" ");
    Some((n, cur))
}

fn parse_date(v: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_basic() {
        let md = r#"
# Subsy

## ChatGPT Plus
- provider: OpenAI
- price: 20 USD
- billing_cycle: monthly
- status: active
- start_date: 2024-01-15
- next_renewal: 2026-09-15
- tags: ai, llm
"#;
        let subs = parse_markdown(md);
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].name, "ChatGPT Plus");
        assert_eq!(subs[0].provider.as_deref(), Some("OpenAI"));
        assert_eq!(subs[0].tags, vec!["ai", "llm"]);
    }
}
