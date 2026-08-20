use std::path::PathBuf;
use std::sync::Mutex;
use subsy_core::{calc, model::*, Store};
use tauri::Manager;

struct AppState {
    store: Mutex<Store>,
}

#[derive(serde::Serialize)]
struct SubscriptionView {
    #[serde(flatten)]
    sub: Subscription,
    paid_so_far: String,
}

#[tauri::command]
fn list_subscriptions(state: tauri::State<'_, AppState>) -> Result<Vec<SubscriptionView>, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    let subs = store.list().map_err(|e| e.to_string())?;
    let mut out = vec![];
    for sub in subs {
        let paid = store.paid_so_far(&sub.id).unwrap_or_default().to_string();
        out.push(SubscriptionView { sub, paid_so_far: paid });
    }
    Ok(out)
}

#[tauri::command]
fn add_subscription(
    name: String,
    provider: Option<String>,
    price: Option<String>,
    currency: Option<String>,
    billing_cycle: Option<String>,
    category: Option<String>,
    payment_method: Option<String>,
    next_renewal: Option<String>,
    tags: Option<String>,
    state: tauri::State<'_, AppState>,
) -> Result<Subscription, String> {
    let mut s = Subscription::new(name);
    s.provider = provider;
    if let Some(p) = price {
        s.price = p.parse().ok();
    }
    s.currency = currency;
    s.billing_cycle = billing_cycle.map(|b| BillingCycle::from_str(&b)).unwrap_or_default();
    s.category = category;
    s.payment_method = payment_method;
    s.next_renewal = next_renewal.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    s.tags = tags.map(|t| t.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()).unwrap_or_default();
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store.add(&s).map_err(|e| e.to_string())?;
    Ok(s)
}

#[tauri::command]
fn delete_subscription(id: String, state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let uid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let store = state.store.lock().map_err(|e| e.to_string())?;
    store.delete(&uid).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_summary(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    let subs = store.list().map_err(|e| e.to_string())?;
    let m = calc::total_monthly(&subs);
    let y = calc::total_yearly(&subs);
    let active = subs.iter().filter(|s| matches!(s.status, Status::Active | Status::Trial)).count();
    Ok(serde_json::json!({
        "count": subs.len(),
        "active": active,
        "monthly": m.to_string(),
        "yearly": y.to_string(),
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir: PathBuf = app.path().app_data_dir().map_err(|e| e.to_string())?;
            std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
            let db_path = app_dir.join("subsy.db");
            let store = Store::open(&db_path).map_err(|e| e.to_string())?;
            app.manage(AppState { store: Mutex::new(store) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_subscriptions, add_subscription, delete_subscription, get_summary])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
