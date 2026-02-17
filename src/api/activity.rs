use anyhow::Result;
use quick_xml::de::from_str;

use super::client::CodebaseClient;
use super::models::*;

pub async fn account_activity(
    client: &CodebaseClient,
    raw: bool,
    since: Option<&str>,
    page: Option<u32>,
) -> Result<Vec<Event>> {
    let path = build_activity_path("/activity", raw, since, page);
    let xml = client.get(&path).await?;
    let events: Events = from_str(&xml)?;
    Ok(events.events)
}

pub async fn project_activity(
    client: &CodebaseClient,
    project: &str,
    raw: bool,
    since: Option<&str>,
    page: Option<u32>,
) -> Result<Vec<Event>> {
    let path = build_activity_path(&format!("/{}/activity", project), raw, since, page);
    let xml = client.get(&path).await?;
    let events: Events = from_str(&xml)?;
    Ok(events.events)
}

fn build_activity_path(base: &str, raw: bool, since: Option<&str>, page: Option<u32>) -> String {
    let mut params = Vec::new();
    if raw {
        params.push("raw=true".to_string());
    }
    if let Some(s) = since {
        params.push(format!("since={}", s));
    }
    if let Some(p) = page {
        params.push(format!("page={}", p));
    }
    if params.is_empty() {
        base.to_string()
    } else {
        format!("{}?{}", base, params.join("&"))
    }
}
