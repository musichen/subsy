use crate::model::*;
use anyhow::Result;
use std::path::Path;

pub fn import_yaml(path: &Path) -> Result<Vec<Subscription>> {
    let s = std::fs::read_to_string(path)?;
    let v: serde_yaml::Value = serde_yaml::from_str(&s)?;
    from_yaml(&v)
}

pub fn import_json(path: &Path) -> Result<Vec<Subscription>> {
    let s = std::fs::read_to_string(path)?;
    let v: serde_json::Value = serde_json::from_str(&s)?;
    let arr = v
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("JSON must be an array of subscriptions"))?;
    let mut out = vec![];
    for item in arr {
        out.push(serde_json::from_value(item.clone())?);
    }
    Ok(out)
}

fn from_yaml(v: &serde_yaml::Value) -> Result<Vec<Subscription>> {
    let arr = v
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("YAML must be a sequence of subscriptions"))?;
    let mut out = vec![];
    for item in arr {
        let s: Subscription = serde_yaml::from_value(item.clone())?;
        out.push(s);
    }
    Ok(out)
}

pub fn import_markdown(path: &Path) -> Result<Vec<Subscription>> {
    let s = std::fs::read_to_string(path)?;
    Ok(crate::import_md::parse_markdown(&s))
}
