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

// ── Commits ──

pub async fn list_commits(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    git_ref: &str,
) -> Result<Vec<Commit>> {
    let xml = client
        .get(&format!("/{}/{}/commits/{}", project, repo, git_ref))
        .await?;
    let commits: Commits = from_str(&xml)?;
    Ok(commits.commits)
}

pub async fn list_commits_path(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    git_ref: &str,
    path: &str,
) -> Result<Vec<Commit>> {
    let xml = client
        .get(&format!(
            "/{}/{}/commits/{}/{}",
            project, repo, git_ref, path
        ))
        .await?;
    let commits: Commits = from_str(&xml)?;
    Ok(commits.commits)
}

// ── Deployments ──

pub async fn create_deployment(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    branch: &str,
    revision: &str,
    servers: &str,
    environment: Option<&str>,
) -> Result<()> {
    let mut fields = String::new();
    fields.push_str(&format!("<branch>{}</branch>", branch));
    fields.push_str(&format!("<revision>{}</revision>", revision));
    fields.push_str(&format!("<servers>{}</servers>", servers));
    if let Some(env) = environment {
        fields.push_str(&format!("<environment>{}</environment>", env));
    }
    let body = format!("<deployment>{}</deployment>", fields);
    client
        .post(&format!("/{}/{}/deployments", project, repo), body)
        .await?;
    Ok(())
}

// ── Files ──

pub async fn get_file(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    git_ref: &str,
    path: &str,
) -> Result<String> {
    let xml = client
        .get(&format!("/{}/{}/blob/{}/{}", project, repo, git_ref, path))
        .await?;
    Ok(xml)
}

// ── Hooks ──

pub async fn list_hooks(client: &CodebaseClient, project: &str, repo: &str) -> Result<Vec<Hook>> {
    let xml = client.get(&format!("/{}/{}/hooks", project, repo)).await?;
    let hooks: Hooks = from_str(&xml)?;
    Ok(hooks.hooks)
}

pub async fn create_hook(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    url: &str,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<Hook> {
    let mut fields = String::new();
    fields.push_str(&format!("<url>{}</url>", url));
    if let Some(u) = username {
        fields.push_str(&format!("<username>{}</username>", u));
    }
    if let Some(p) = password {
        fields.push_str(&format!("<password>{}</password>", p));
    }
    let body = format!("<repository-hook>{}</repository-hook>", fields);
    let xml = client
        .post(&format!("/{}/{}/hooks", project, repo), body)
        .await?;
    let hook: Hook = from_str(&xml)?;
    Ok(hook)
}

// ── Branches ──

pub async fn list_branches(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
) -> Result<Vec<Branch>> {
    let xml = client
        .get(&format!("/{}/{}/branches", project, repo))
        .await?;
    let branches: Branches = from_str(&xml)?;
    Ok(branches.branches)
}

// ── Merge Requests ──

pub async fn list_merge_requests(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
) -> Result<Vec<MergeRequest>> {
    let xml = client
        .get(&format!("/{}/{}/merge_requests", project, repo))
        .await?;
    let mrs: MergeRequests = from_str(&xml)?;
    Ok(mrs.merge_requests)
}

pub async fn show_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
) -> Result<MergeRequest> {
    let xml = client
        .get(&format!("/{}/{}/merge_requests/{}", project, repo, mr_id))
        .await?;
    let mr: MergeRequest = from_str(&xml)?;
    Ok(mr)
}

pub async fn create_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    source_ref: &str,
    target_ref: &str,
    subject: &str,
) -> Result<MergeRequest> {
    let body = format!(
        "<merge-request><source-ref>{}</source-ref><target-ref>{}</target-ref><subject>{}</subject></merge-request>",
        source_ref, target_ref, subject
    );
    let xml = client
        .post(&format!("/{}/{}/merge_requests", project, repo), body)
        .await?;
    let mr: MergeRequest = from_str(&xml)?;
    Ok(mr)
}

pub async fn comment_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
    content: &str,
) -> Result<()> {
    let body = format!(
        "<merge-request-comment><content>{}</content></merge-request-comment>",
        content
    );
    client
        .post(
            &format!("/{}/{}/merge_requests/{}/comment", project, repo, mr_id),
            body,
        )
        .await?;
    Ok(())
}

pub async fn close_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
) -> Result<()> {
    client
        .post(
            &format!("/{}/{}/merge_requests/{}/close", project, repo, mr_id),
            String::new(),
        )
        .await?;
    Ok(())
}

pub async fn reopen_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
) -> Result<()> {
    client
        .post(
            &format!("/{}/{}/merge_requests/{}/reopen", project, repo, mr_id),
            String::new(),
        )
        .await?;
    Ok(())
}

pub async fn merge_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
) -> Result<()> {
    client
        .post(
            &format!("/{}/{}/merge_requests/{}/merge", project, repo, mr_id),
            String::new(),
        )
        .await?;
    Ok(())
}

pub async fn reassign_merge_request(
    client: &CodebaseClient,
    project: &str,
    repo: &str,
    mr_id: i64,
    user_id: i64,
) -> Result<()> {
    let body = format!(
        "<merge-request><user-id>{}</user-id></merge-request>",
        user_id
    );
    client
        .post(
            &format!("/{}/{}/merge_requests/{}/reassign", project, repo, mr_id),
            body,
        )
        .await?;
    Ok(())
}
