use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::repositories::*;
use mockito::Server;

#[tokio::test]
async fn test_list_repositories() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repositories>
  <repository>
    <name>main-repo</name>
    <permalink>main-repo</permalink>
    <disk-usage>1024000</disk-usage>
    <last-commit-ref>abc123def456</last-commit-ref>
    <clone-url>git@codebase.com:account/project/main-repo.git</clone-url>
    <source>codebase</source>
    <sync>true</sync>
    <last-sync-at>2026-02-15T10:30:00Z</last-sync-at>
  </repository>
  <repository>
    <name>dev-repo</name>
    <permalink>dev-repo</permalink>
    <disk-usage>512000</disk-usage>
    <last-commit-ref>xyz789abc123</last-commit-ref>
    <clone-url>git@codebase.com:account/project/dev-repo.git</clone-url>
    <source>github</source>
    <sync>false</sync>
    <last-sync-at>2026-02-10T08:15:00Z</last-sync-at>
  </repository>
</repositories>"#;

    let mock = server
        .mock("GET", "/test-project/repositories")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(xml_response)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_repositories(&client, "test-project").await;

    assert!(result.is_ok());
    let repos = result.unwrap();
    assert_eq!(repos.len(), 2);

    // Verify first repository
    assert_eq!(repos[0].name.as_deref(), Some("main-repo"));
    assert_eq!(repos[0].permalink.as_deref(), Some("main-repo"));
    assert_eq!(repos[0].disk_usage, Some(1024000));
    assert_eq!(repos[0].last_commit_ref.as_deref(), Some("abc123def456"));
    assert_eq!(
        repos[0].clone_url.as_deref(),
        Some("git@codebase.com:account/project/main-repo.git")
    );
    assert_eq!(repos[0].source.as_deref(), Some("codebase"));
    assert_eq!(repos[0].sync, Some(true));
    assert_eq!(
        repos[0].last_sync_at.as_deref(),
        Some("2026-02-15T10:30:00Z")
    );

    // Verify second repository
    assert_eq!(repos[1].name.as_deref(), Some("dev-repo"));
    assert_eq!(repos[1].permalink.as_deref(), Some("dev-repo"));
    assert_eq!(repos[1].disk_usage, Some(512000));
    assert_eq!(repos[1].last_commit_ref.as_deref(), Some("xyz789abc123"));
    assert_eq!(
        repos[1].clone_url.as_deref(),
        Some("git@codebase.com:account/project/dev-repo.git")
    );
    assert_eq!(repos[1].source.as_deref(), Some("github"));
    assert_eq!(repos[1].sync, Some(false));
    assert_eq!(
        repos[1].last_sync_at.as_deref(),
        Some("2026-02-10T08:15:00Z")
    );

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_repositories_empty() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repositories>
</repositories>"#;

    let mock = server
        .mock("GET", "/test-project/repositories")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(xml_response)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_repositories(&client, "test-project").await;

    assert!(result.is_ok());
    let repos = result.unwrap();
    assert_eq!(repos.len(), 0);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_repository() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository>
  <name>backend</name>
  <permalink>backend</permalink>
  <disk-usage>2048000</disk-usage>
  <last-commit-ref>def456abc789</last-commit-ref>
  <clone-url>git@codebase.com:account/project/backend.git</clone-url>
  <source>codebase</source>
  <sync>true</sync>
  <last-sync-at>2026-02-16T12:00:00Z</last-sync-at>
</repository>"#;

    let mock = server
        .mock("GET", "/my-project/backend")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(xml_response)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = show_repository(&client, "my-project", "backend").await;

    assert!(result.is_ok());
    let repo = result.unwrap();

    assert_eq!(repo.name.as_deref(), Some("backend"));
    assert_eq!(repo.permalink.as_deref(), Some("backend"));
    assert_eq!(repo.disk_usage, Some(2048000));
    assert_eq!(repo.last_commit_ref.as_deref(), Some("def456abc789"));
    assert_eq!(
        repo.clone_url.as_deref(),
        Some("git@codebase.com:account/project/backend.git")
    );
    assert_eq!(repo.source.as_deref(), Some("codebase"));
    assert_eq!(repo.sync, Some(true));
    assert_eq!(repo.last_sync_at.as_deref(), Some("2026-02-16T12:00:00Z"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_repository() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository>
  <name>new-repo</name>
  <permalink>new-repo</permalink>
  <disk-usage>0</disk-usage>
  <last-commit-ref></last-commit-ref>
  <clone-url>git@codebase.com:account/project/new-repo.git</clone-url>
  <source>codebase</source>
  <sync>false</sync>
  <last-sync-at></last-sync-at>
</repository>"#;

    let mock = server
        .mock("POST", "/test-project/repositories")
        .match_body("<repository><name>new-repo</name><scm>git</scm></repository>")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(xml_response)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_repository(&client, "test-project", "new-repo", "git").await;

    assert!(result.is_ok());
    let repo = result.unwrap();

    assert_eq!(repo.name.as_deref(), Some("new-repo"));
    assert_eq!(repo.permalink.as_deref(), Some("new-repo"));
    assert_eq!(repo.disk_usage, Some(0));
    assert_eq!(
        repo.clone_url.as_deref(),
        Some("git@codebase.com:account/project/new-repo.git")
    );
    assert_eq!(repo.source.as_deref(), Some("codebase"));
    assert_eq!(repo.sync, Some(false));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_repository_svn() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository>
  <name>svn-repo</name>
  <permalink>svn-repo</permalink>
  <disk-usage>0</disk-usage>
  <last-commit-ref></last-commit-ref>
  <clone-url>svn://codebase.com/account/project/svn-repo</clone-url>
  <source>codebase</source>
  <sync>false</sync>
  <last-sync-at></last-sync-at>
</repository>"#;

    let mock = server
        .mock("POST", "/test-project/repositories")
        .match_body("<repository><name>svn-repo</name><scm>svn</scm></repository>")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(xml_response)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_repository(&client, "test-project", "svn-repo", "svn").await;

    assert!(result.is_ok());
    let repo = result.unwrap();

    assert_eq!(repo.name.as_deref(), Some("svn-repo"));
    assert_eq!(repo.permalink.as_deref(), Some("svn-repo"));
    assert_eq!(
        repo.clone_url.as_deref(),
        Some("svn://codebase.com/account/project/svn-repo")
    );

    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_repository() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("DELETE", "/test-project/old-repo")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = delete_repository(&client, "test-project", "old-repo").await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_repositories_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent-project/repositories")
        .with_status(404)
        .with_body("Project not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_repositories(&client, "nonexistent-project").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_repository_error_403() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/restricted-repo")
        .with_status(403)
        .with_body("Access denied")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = show_repository(&client, "test-project", "restricted-repo").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("403"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_repository_error_422() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/repositories")
        .with_status(422)
        .with_body("Invalid repository name")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_repository(&client, "test-project", "invalid name!", "git").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("422"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_repository_error_500() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("DELETE", "/test-project/problem-repo")
        .with_status(500)
        .with_body("Internal server error")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = delete_repository(&client, "test-project", "problem-repo").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("500"));

    mock.assert_async().await;
}
