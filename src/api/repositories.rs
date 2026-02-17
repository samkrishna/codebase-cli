use anyhow::Result;
use quick_xml::de::from_str;

use super::client::CodebaseClient;
use super::models::*;

pub async fn list_repositories(client: &CodebaseClient, project: &str) -> Result<Vec<Repository>> {
    let xml = client.get(&format!("/{}/repositories", project)).await?;
    let repos: Repositories = from_str(&xml)?;
    Ok(repos.repositories)
}

pub async fn show_repository(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
) -> Result<Repository> {
    let xml = client.get(&format!("/{}/{}", project, repo)).await?;
    let repository: Repository = from_str(&xml)?;
    Ok(repository)
}

pub async fn create_repository(
    client: &CodebaseClient,
    project: &str,
    name: &str,
    scm: &str,
) -> Result<Repository> {
    let body = format!(
        "<repository><name>{}</name><scm>{}</scm></repository>",
        name, scm
    );
    let xml = client
        .post(&format!("/{}/repositories", project), body)
        .await?;
    let repository: Repository = from_str(&xml)?;
    Ok(repository)
}

pub async fn delete_repository(client: &CodebaseClient, project: &str, repo: &str) -> Result<()> {
    client.delete(&format!("/{}/{}", project, repo)).await?;
    Ok(())
}
