use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::repositories::*;
use mockito::Server;

// ── Hooks Tests ──

#[tokio::test]
async fn test_list_hooks() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository-hooks>
  <repository-hook>
    <id>1</id>
    <url>https://example.com/webhook1</url>
    <username>admin</username>
    <password>secret123</password>
  </repository-hook>
  <repository-hook>
    <id>2</id>
    <url>https://example.com/webhook2</url>
    <username>user</username>
    <password>pass456</password>
  </repository-hook>
</repository-hooks>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/hooks")
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

    let result = list_hooks(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let hooks = result.unwrap();
    assert_eq!(hooks.len(), 2);

    // Verify first hook
    assert_eq!(hooks[0].id, Some(1));
    assert_eq!(
        hooks[0].url.as_deref(),
        Some("https://example.com/webhook1")
    );
    assert_eq!(hooks[0].username.as_deref(), Some("admin"));
    assert_eq!(hooks[0].password.as_deref(), Some("secret123"));

    // Verify second hook
    assert_eq!(hooks[1].id, Some(2));
    assert_eq!(
        hooks[1].url.as_deref(),
        Some("https://example.com/webhook2")
    );
    assert_eq!(hooks[1].username.as_deref(), Some("user"));
    assert_eq!(hooks[1].password.as_deref(), Some("pass456"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_hooks_empty() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository-hooks>
</repository-hooks>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/hooks")
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

    let result = list_hooks(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let hooks = result.unwrap();
    assert_eq!(hooks.len(), 0);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_hook_with_auth() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository-hook>
  <id>42</id>
  <url>https://example.com/new-webhook</url>
  <username>webhook-user</username>
  <password>webhook-pass</password>
</repository-hook>"#;

    let expected_body = "<repository-hook><url>https://example.com/new-webhook</url><username>webhook-user</username><password>webhook-pass</password></repository-hook>";

    let mock = server
        .mock("POST", "/test-project/test-repo/hooks")
        .match_body(expected_body)
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

    let result = create_hook(
        &client,
        "test-project",
        "test-repo",
        "https://example.com/new-webhook",
        Some("webhook-user"),
        Some("webhook-pass"),
    )
    .await;

    assert!(result.is_ok());
    let hook = result.unwrap();

    assert_eq!(hook.id, Some(42));
    assert_eq!(hook.url.as_deref(), Some("https://example.com/new-webhook"));
    assert_eq!(hook.username.as_deref(), Some("webhook-user"));
    assert_eq!(hook.password.as_deref(), Some("webhook-pass"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_hook_without_auth() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<repository-hook>
  <id>99</id>
  <url>https://public.example.com/hook</url>
</repository-hook>"#;

    let expected_body =
        "<repository-hook><url>https://public.example.com/hook</url></repository-hook>";

    let mock = server
        .mock("POST", "/test-project/test-repo/hooks")
        .match_body(expected_body)
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

    let result = create_hook(
        &client,
        "test-project",
        "test-repo",
        "https://public.example.com/hook",
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
    let hook = result.unwrap();

    assert_eq!(hook.id, Some(99));
    assert_eq!(hook.url.as_deref(), Some("https://public.example.com/hook"));
    assert_eq!(hook.username, None);
    assert_eq!(hook.password, None);

    mock.assert_async().await;
}

// ── Branches Tests ──

#[tokio::test]
async fn test_list_branches() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<branches>
  <branch>
    <name>main</name>
  </branch>
</branches>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/branches")
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

    let result = list_branches(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let branches = result.unwrap();
    assert_eq!(branches.len(), 1);

    assert_eq!(branches[0].name.as_deref(), Some("main"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_branches_multiple() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<branches>
  <branch>
    <name>main</name>
  </branch>
  <branch>
    <name>develop</name>
  </branch>
  <branch>
    <name>feature/new-api</name>
  </branch>
  <branch>
    <name>hotfix/bug-123</name>
  </branch>
</branches>"#;

    let mock = server
        .mock("GET", "/my-project/my-repo/branches")
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

    let result = list_branches(&client, "my-project", "my-repo").await;

    assert!(result.is_ok());
    let branches = result.unwrap();
    assert_eq!(branches.len(), 4);

    assert_eq!(branches[0].name.as_deref(), Some("main"));
    assert_eq!(branches[1].name.as_deref(), Some("develop"));
    assert_eq!(branches[2].name.as_deref(), Some("feature/new-api"));
    assert_eq!(branches[3].name.as_deref(), Some("hotfix/bug-123"));

    mock.assert_async().await;
}

// ── Error Tests ──

#[tokio::test]
async fn test_list_hooks_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent-project/test-repo/hooks")
        .with_status(404)
        .with_body("Repository not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_hooks(&client, "nonexistent-project", "test-repo").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_hook_error_422() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/hooks")
        .with_status(422)
        .with_body("Invalid URL format")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_hook(
        &client,
        "test-project",
        "test-repo",
        "not-a-valid-url",
        None,
        None,
    )
    .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("422"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_branches_error_403() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/restricted-repo/branches")
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

    let result = list_branches(&client, "test-project", "restricted-repo").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("403"));

    mock.assert_async().await;
}
