use anyhow::Result;
use quick_xml::de::from_str;

use super::client::CodebaseClient;
use super::models::*;

pub async fn list_milestones(client: &CodebaseClient, project: &str) -> Result<Vec<Milestone>> {
    let xml = client.get(&format!("/{}/milestones", project)).await?;
    let milestones: Milestones = from_str(&xml)?;
    Ok(milestones.milestones)
}

pub async fn create_milestone(
    client: &CodebaseClient,
    project: &str,
    name: &str,
    description: Option<&str>,
    start_at: Option<&str>,
    deadline: Option<&str>,
    responsible_user_id: Option<i64>,
    parent_id: Option<i64>,
    status: Option<&str>,
) -> Result<Milestone> {
    let mut fields = String::new();
    fields.push_str(&format!("<name>{}</name>", name));
    if let Some(d) = description {
        fields.push_str(&format!("<description>{}</description>", d));
    }
    if let Some(s) = start_at {
        fields.push_str(&format!("<start-at>{}</start-at>", s));
    }
    if let Some(d) = deadline {
        fields.push_str(&format!("<deadline>{}</deadline>", d));
    }
    if let Some(id) = responsible_user_id {
        fields.push_str(&format!(
            "<responsible-user-id>{}</responsible-user-id>",
            id
        ));
    }
    if let Some(id) = parent_id {
        fields.push_str(&format!("<parent-id>{}</parent-id>", id));
    }
    if let Some(s) = status {
        fields.push_str(&format!("<status>{}</status>", s));
    }
    let body = format!("<ticketing-milestone>{}</ticketing-milestone>", fields);
    let xml = client
        .post(&format!("/{}/milestones", project), body)
        .await?;
    let milestone: Milestone = from_str(&xml)?;
    Ok(milestone)
}

pub async fn update_milestone(
    client: &CodebaseClient,
    project: &str,
    milestone_id: i64,
    name: Option<&str>,
    description: Option<&str>,
    start_at: Option<&str>,
    deadline: Option<&str>,
    responsible_user_id: Option<i64>,
    parent_id: Option<i64>,
    status: Option<&str>,
) -> Result<Milestone> {
    let mut fields = String::new();
    if let Some(n) = name {
        fields.push_str(&format!("<name>{}</name>", n));
    }
    if let Some(d) = description {
        fields.push_str(&format!("<description>{}</description>", d));
    }
    if let Some(s) = start_at {
        fields.push_str(&format!("<start-at>{}</start-at>", s));
    }
    if let Some(d) = deadline {
        fields.push_str(&format!("<deadline>{}</deadline>", d));
    }
    if let Some(id) = responsible_user_id {
        fields.push_str(&format!(
            "<responsible-user-id>{}</responsible-user-id>",
            id
        ));
    }
    if let Some(id) = parent_id {
        fields.push_str(&format!("<parent-id>{}</parent-id>", id));
    }
    if let Some(s) = status {
        fields.push_str(&format!("<status>{}</status>", s));
    }
    let body = format!("<ticketing-milestone>{}</ticketing-milestone>", fields);
    let xml = client
        .put(&format!("/{}/milestones/{}", project, milestone_id), body)
        .await?;
    let milestone: Milestone = from_str(&xml)?;
    Ok(milestone)
}
