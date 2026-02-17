use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::repositories::*;
use mockito::Server;

// ── Commits Tests ──

#[tokio::test]
async fn test_list_commits() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<commits>
  <commit>
    <ref>abc123def456</ref>
    <message>Initial commit</message>
    <author-name>John Doe</author-name>
    <author-email>john@example.com</author-email>
    <authored-at>2026-02-15T10:30:00Z</authored-at>
    <committer-name>John Doe</committer-name>
    <committer-email>john@example.com</committer-email>
    <committed-at>2026-02-15T10:30:00Z</committed-at>
    <parent-refs></parent-refs>
    <tree-ref>tree123abc456</tree-ref>
    <author-user>johndoe</author-user>
    <committer-user>johndoe</committer-user>
  </commit>
</commits>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/commits/main")
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

    let result = list_commits(&client, "test-project", "test-repo", "main").await;

    assert!(result.is_ok());
    let commits = result.unwrap();
    assert_eq!(commits.len(), 1);

    // Verify commit details
    assert_eq!(commits[0].commit_ref.as_deref(), Some("abc123def456"));
    assert_eq!(commits[0].message.as_deref(), Some("Initial commit"));
    assert_eq!(commits[0].author_name.as_deref(), Some("John Doe"));
    assert_eq!(commits[0].author_email.as_deref(), Some("john@example.com"));
    assert_eq!(
        commits[0].authored_at.as_deref(),
        Some("2026-02-15T10:30:00Z")
    );
    assert_eq!(commits[0].committer_name.as_deref(), Some("John Doe"));
    assert_eq!(
        commits[0].committer_email.as_deref(),
        Some("john@example.com")
    );
    assert_eq!(
        commits[0].committed_at.as_deref(),
        Some("2026-02-15T10:30:00Z")
    );
    assert_eq!(commits[0].parent_refs.as_deref(), Some(""));
    assert_eq!(commits[0].tree_ref.as_deref(), Some("tree123abc456"));
    assert_eq!(commits[0].author_user.as_deref(), Some("johndoe"));
    assert_eq!(commits[0].committer_user.as_deref(), Some("johndoe"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_commits_multiple() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<commits>
  <commit>
    <ref>abc123def456</ref>
    <message>Add new feature</message>
    <author-name>Jane Smith</author-name>
    <author-email>jane@example.com</author-email>
    <authored-at>2026-02-16T14:20:00Z</authored-at>
    <committer-name>Jane Smith</committer-name>
    <committer-email>jane@example.com</committer-email>
    <committed-at>2026-02-16T14:20:00Z</committed-at>
    <parent-refs>xyz789ghi012</parent-refs>
    <tree-ref>tree456def789</tree-ref>
    <author-user>janesmith</author-user>
    <committer-user>janesmith</committer-user>
  </commit>
  <commit>
    <ref>xyz789ghi012</ref>
    <message>Fix bug in authentication</message>
    <author-name>Bob Johnson</author-name>
    <author-email>bob@example.com</author-email>
    <authored-at>2026-02-15T09:15:00Z</authored-at>
    <committer-name>Bob Johnson</committer-name>
    <committer-email>bob@example.com</committer-email>
    <committed-at>2026-02-15T09:15:00Z</committed-at>
    <parent-refs>def456jkl789</parent-refs>
    <tree-ref>tree789ghi123</tree-ref>
    <author-user>bobjohnson</author-user>
    <committer-user>bobjohnson</committer-user>
  </commit>
  <commit>
    <ref>def456jkl789</ref>
    <message>Initial setup</message>
    <author-name>Admin User</author-name>
    <author-email>admin@example.com</author-email>
    <authored-at>2026-02-14T08:00:00Z</authored-at>
    <committer-name>Admin User</committer-name>
    <committer-email>admin@example.com</committer-email>
    <committed-at>2026-02-14T08:00:00Z</committed-at>
    <parent-refs></parent-refs>
    <tree-ref>tree123start456</tree-ref>
    <author-user>admin</author-user>
    <committer-user>admin</committer-user>
  </commit>
</commits>"#;

    let mock = server
        .mock("GET", "/my-project/backend/commits/develop")
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

    let result = list_commits(&client, "my-project", "backend", "develop").await;

    assert!(result.is_ok());
    let commits = result.unwrap();
    assert_eq!(commits.len(), 3);

    // Verify first commit
    assert_eq!(commits[0].commit_ref.as_deref(), Some("abc123def456"));
    assert_eq!(commits[0].message.as_deref(), Some("Add new feature"));
    assert_eq!(commits[0].author_name.as_deref(), Some("Jane Smith"));
    assert_eq!(commits[0].parent_refs.as_deref(), Some("xyz789ghi012"));

    // Verify second commit
    assert_eq!(commits[1].commit_ref.as_deref(), Some("xyz789ghi012"));
    assert_eq!(
        commits[1].message.as_deref(),
        Some("Fix bug in authentication")
    );
    assert_eq!(commits[1].author_name.as_deref(), Some("Bob Johnson"));

    // Verify third commit
    assert_eq!(commits[2].commit_ref.as_deref(), Some("def456jkl789"));
    assert_eq!(commits[2].message.as_deref(), Some("Initial setup"));
    assert_eq!(commits[2].parent_refs.as_deref(), Some(""));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_commits_path() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<commits>
  <commit>
    <ref>path123abc456</ref>
    <message>Update README documentation</message>
    <author-name>Doc Writer</author-name>
    <author-email>docs@example.com</author-email>
    <authored-at>2026-02-16T11:00:00Z</authored-at>
    <committer-name>Doc Writer</committer-name>
    <committer-email>docs@example.com</committer-email>
    <committed-at>2026-02-16T11:00:00Z</committed-at>
    <parent-refs>oldpath789def012</parent-refs>
    <tree-ref>treeREADME456</tree-ref>
    <author-user>docwriter</author-user>
    <committer-user>docwriter</committer-user>
  </commit>
  <commit>
    <ref>oldpath789def012</ref>
    <message>Add README file</message>
    <author-name>Initial Author</author-name>
    <author-email>initial@example.com</author-email>
    <authored-at>2026-02-10T09:00:00Z</authored-at>
    <committer-name>Initial Author</committer-name>
    <committer-email>initial@example.com</committer-email>
    <committed-at>2026-02-10T09:00:00Z</committed-at>
    <parent-refs></parent-refs>
    <tree-ref>treeREADMEinit789</tree-ref>
    <author-user>initialauthor</author-user>
    <committer-user>initialauthor</committer-user>
  </commit>
</commits>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/commits/main/README.md")
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

    let result = list_commits_path(&client, "test-project", "test-repo", "main", "README.md").await;

    assert!(result.is_ok());
    let commits = result.unwrap();
    assert_eq!(commits.len(), 2);

    // Verify commits are filtered to specific path
    assert_eq!(commits[0].commit_ref.as_deref(), Some("path123abc456"));
    assert_eq!(
        commits[0].message.as_deref(),
        Some("Update README documentation")
    );
    assert_eq!(commits[1].commit_ref.as_deref(), Some("oldpath789def012"));
    assert_eq!(commits[1].message.as_deref(), Some("Add README file"));

    mock.assert_async().await;
}

// ── Deployments Tests ──

#[tokio::test]
async fn test_create_deployment_with_environment() {
    let mut server = Server::new_async().await;

    let expected_body = "<deployment><branch>main</branch><revision>abc123def456</revision><servers>web01,web02</servers><environment>production</environment></deployment>";

    let mock = server
        .mock("POST", "/test-project/test-repo/deployments")
        .match_body(expected_body)
        .with_status(201)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_deployment(
        &client,
        "test-project",
        "test-repo",
        "main",
        "abc123def456",
        "web01,web02",
        Some("production"),
    )
    .await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_deployment_without_environment() {
    let mut server = Server::new_async().await;

    let expected_body = "<deployment><branch>develop</branch><revision>xyz789ghi012</revision><servers>staging-server</servers></deployment>";

    let mock = server
        .mock("POST", "/my-project/backend/deployments")
        .match_body(expected_body)
        .with_status(201)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_deployment(
        &client,
        "my-project",
        "backend",
        "develop",
        "xyz789ghi012",
        "staging-server",
        None,
    )
    .await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

// ── Files Tests ──

#[tokio::test]
async fn test_get_file() {
    let mut server = Server::new_async().await;

    let file_content = r#"# My Project

This is a README file for the project.

## Installation

Run `npm install` to get started.

## Usage

Start the server with `npm start`.
"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/blob/main/README.md")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(file_content)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = get_file(&client, "test-project", "test-repo", "main", "README.md").await;

    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("# My Project"));
    assert!(content.contains("npm install"));
    assert!(content.contains("npm start"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_file_source_code() {
    let mut server = Server::new_async().await;

    let file_content = r#"pub fn main() {
    println!("Hello, world!");
}
"#;

    let mock = server
        .mock("GET", "/my-project/backend/blob/abc123def456/src/main.rs")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(file_content)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = get_file(
        &client,
        "my-project",
        "backend",
        "abc123def456",
        "src/main.rs",
    )
    .await;

    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(content.contains("pub fn main()"));
    assert!(content.contains("println!"));

    mock.assert_async().await;
}

// ── Error Tests ──

#[tokio::test]
async fn test_list_commits_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/test-repo/commits/nonexistent-branch")
        .with_status(404)
        .with_body("Branch not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_commits(&client, "test-project", "test-repo", "nonexistent-branch").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_file_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/test-repo/blob/main/nonexistent.txt")
        .with_status(404)
        .with_body("File not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = get_file(
        &client,
        "test-project",
        "test-repo",
        "main",
        "nonexistent.txt",
    )
    .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_deployment_error_422() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/deployments")
        .with_status(422)
        .with_body("Invalid deployment parameters")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_deployment(
        &client,
        "test-project",
        "test-repo",
        "invalid-branch",
        "invalid-revision",
        "",
        None,
    )
    .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("422"));

    mock.assert_async().await;
}
