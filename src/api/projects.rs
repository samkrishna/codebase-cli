use anyhow::Result;
use quick_xml::de::from_str;

use super::client::CodebaseClient;
use super::models::*;

pub async fn list_projects(client: &CodebaseClient) -> Result<Vec<Project>> {
    let xml = client.get("/projects").await?;
    let projects: Projects = from_str(&xml)?;
    Ok(projects.projects)
}

pub async fn show_project(client: &CodebaseClient, permalink: &str) -> Result<Project> {
    let xml = client.get(&format!("/{}", permalink)).await?;
    let project: Project = from_str(&xml)?;
    Ok(project)
}

pub async fn create_project(client: &CodebaseClient, name: &str) -> Result<Project> {
    let body = format!("<project><name>{}</name></project>", name);
    let xml = client.post("/create_project", body).await?;
    let project: Project = from_str(&xml)?;
    Ok(project)
}

pub async fn update_project(
    client: &CodebaseClient,
    project_id: &str,
    name: Option<&str>,
    status: Option<&str>,
) -> Result<Project> {
    let mut fields = String::new();
    if let Some(n) = name {
        fields.push_str(&format!("<name>{}</name>", n));
    }
    if let Some(s) = status {
        fields.push_str(&format!("<status>{}</status>", s));
    }
    let body = format!("<project>{}</project>", fields);
    let xml = client
        .put(&format!("/project/{}", project_id), body)
        .await?;
    let project: Project = from_str(&xml)?;
    Ok(project)
}

pub async fn delete_project(client: &CodebaseClient, permalink: &str) -> Result<()> {
    client.delete(&format!("/{}", permalink)).await?;
    Ok(())
}

pub async fn list_project_groups(client: &CodebaseClient) -> Result<Vec<ProjectGroup>> {
    let xml = client.get("/project_groups").await?;
    let groups: ProjectGroups = from_str(&xml)?;
    Ok(groups.groups)
}

pub async fn list_project_users(
    client: &CodebaseClient,
    project: &str,
) -> Result<Vec<ProjectUser>> {
    let xml = client.get(&format!("/{}/assignments", project)).await?;
    let users: ProjectUsers = from_str(&xml)?;
    Ok(users.users)
}

pub async fn assign_project_users(
    client: &CodebaseClient,
    project: &str,
    user_ids: &[i64],
) -> Result<()> {
    let users_xml: String = user_ids
        .iter()
        .map(|id| format!("<user><id>{}</id></user>", id))
        .collect();
    let body = format!("<users>{}</users>", users_xml);
    client
        .post(&format!("/{}/assignments", project), body)
        .await?;
    Ok(())
}
