use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::repositories::*;
use mockito::Server;

#[tokio::test]
async fn test_list_merge_requests() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-requests>
  <merge-request>
    <id>1</id>
    <source-ref>feature/new-feature</source-ref>
    <target-ref>main</target-ref>
    <subject>Add new feature</subject>
    <status>open</status>
    <user-id>123</user-id>
    <created-at>2026-02-15T10:00:00Z</created-at>
    <updated-at>2026-02-15T12:00:00Z</updated-at>
  </merge-request>
</merge-requests>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/merge_requests")
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

    let result = list_merge_requests(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let mrs = result.unwrap();
    assert_eq!(mrs.len(), 1);

    // Verify merge request
    assert_eq!(mrs[0].id, Some(1));
    assert_eq!(mrs[0].source_ref.as_deref(), Some("feature/new-feature"));
    assert_eq!(mrs[0].target_ref.as_deref(), Some("main"));
    assert_eq!(mrs[0].subject.as_deref(), Some("Add new feature"));
    assert_eq!(mrs[0].status.as_deref(), Some("open"));
    assert_eq!(mrs[0].user_id, Some(123));
    assert_eq!(mrs[0].created_at.as_deref(), Some("2026-02-15T10:00:00Z"));
    assert_eq!(mrs[0].updated_at.as_deref(), Some("2026-02-15T12:00:00Z"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_merge_requests_multiple() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-requests>
  <merge-request>
    <id>1</id>
    <source-ref>feature/feature-a</source-ref>
    <target-ref>main</target-ref>
    <subject>Add feature A</subject>
    <status>open</status>
    <user-id>123</user-id>
    <created-at>2026-02-15T10:00:00Z</created-at>
    <updated-at>2026-02-15T12:00:00Z</updated-at>
  </merge-request>
  <merge-request>
    <id>2</id>
    <source-ref>bugfix/fix-issue</source-ref>
    <target-ref>main</target-ref>
    <subject>Fix critical bug</subject>
    <status>merged</status>
    <user-id>456</user-id>
    <created-at>2026-02-14T08:00:00Z</created-at>
    <updated-at>2026-02-14T16:00:00Z</updated-at>
  </merge-request>
  <merge-request>
    <id>3</id>
    <source-ref>feature/feature-b</source-ref>
    <target-ref>develop</target-ref>
    <subject>Add feature B</subject>
    <status>closed</status>
    <user-id>789</user-id>
    <created-at>2026-02-13T09:00:00Z</created-at>
    <updated-at>2026-02-13T18:00:00Z</updated-at>
  </merge-request>
</merge-requests>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/merge_requests")
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

    let result = list_merge_requests(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let mrs = result.unwrap();
    assert_eq!(mrs.len(), 3);

    // Verify first merge request
    assert_eq!(mrs[0].id, Some(1));
    assert_eq!(mrs[0].source_ref.as_deref(), Some("feature/feature-a"));
    assert_eq!(mrs[0].status.as_deref(), Some("open"));
    assert_eq!(mrs[0].user_id, Some(123));

    // Verify second merge request
    assert_eq!(mrs[1].id, Some(2));
    assert_eq!(mrs[1].source_ref.as_deref(), Some("bugfix/fix-issue"));
    assert_eq!(mrs[1].status.as_deref(), Some("merged"));
    assert_eq!(mrs[1].user_id, Some(456));

    // Verify third merge request
    assert_eq!(mrs[2].id, Some(3));
    assert_eq!(mrs[2].source_ref.as_deref(), Some("feature/feature-b"));
    assert_eq!(mrs[2].target_ref.as_deref(), Some("develop"));
    assert_eq!(mrs[2].status.as_deref(), Some("closed"));
    assert_eq!(mrs[2].user_id, Some(789));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_merge_requests_empty() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-requests>
</merge-requests>"#;

    let mock = server
        .mock("GET", "/test-project/test-repo/merge_requests")
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

    let result = list_merge_requests(&client, "test-project", "test-repo").await;

    assert!(result.is_ok());
    let mrs = result.unwrap();
    assert_eq!(mrs.len(), 0);

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_merge_request() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-request>
  <id>42</id>
  <source-ref>feature/awesome</source-ref>
  <target-ref>main</target-ref>
  <subject>Add awesome feature</subject>
  <status>open</status>
  <user-id>999</user-id>
  <created-at>2026-02-16T08:00:00Z</created-at>
  <updated-at>2026-02-16T09:30:00Z</updated-at>
  <can-merge>true</can-merge>
</merge-request>"#;

    let mock = server
        .mock("GET", "/my-project/my-repo/merge_requests/42")
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

    let result = show_merge_request(&client, "my-project", "my-repo", 42).await;

    assert!(result.is_ok());
    let mr = result.unwrap();

    assert_eq!(mr.id, Some(42));
    assert_eq!(mr.source_ref.as_deref(), Some("feature/awesome"));
    assert_eq!(mr.target_ref.as_deref(), Some("main"));
    assert_eq!(mr.subject.as_deref(), Some("Add awesome feature"));
    assert_eq!(mr.status.as_deref(), Some("open"));
    assert_eq!(mr.user_id, Some(999));
    assert_eq!(mr.created_at.as_deref(), Some("2026-02-16T08:00:00Z"));
    assert_eq!(mr.updated_at.as_deref(), Some("2026-02-16T09:30:00Z"));
    assert_eq!(mr.can_merge, Some(true));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_merge_request_cannot_merge() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-request>
  <id>55</id>
  <source-ref>feature/conflict</source-ref>
  <target-ref>main</target-ref>
  <subject>Conflicting changes</subject>
  <status>open</status>
  <user-id>123</user-id>
  <created-at>2026-02-15T10:00:00Z</created-at>
  <updated-at>2026-02-15T11:00:00Z</updated-at>
  <can-merge>false</can-merge>
</merge-request>"#;

    let mock = server
        .mock("GET", "/my-project/my-repo/merge_requests/55")
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

    let result = show_merge_request(&client, "my-project", "my-repo", 55).await;

    assert!(result.is_ok());
    let mr = result.unwrap();

    assert_eq!(mr.id, Some(55));
    assert_eq!(mr.can_merge, Some(false));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_merge_request() {
    let mut server = Server::new_async().await;

    let xml_response = r#"<?xml version="1.0" encoding="UTF-8"?>
<merge-request>
  <id>100</id>
  <source-ref>feature/new</source-ref>
  <target-ref>main</target-ref>
  <subject>New feature request</subject>
  <status>open</status>
  <user-id>123</user-id>
  <created-at>2026-02-16T10:00:00Z</created-at>
  <updated-at>2026-02-16T10:00:00Z</updated-at>
</merge-request>"#;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests")
        .match_body("<merge-request><source-ref>feature/new</source-ref><target-ref>main</target-ref><subject>New feature request</subject></merge-request>")
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

    let result = create_merge_request(
        &client,
        "test-project",
        "test-repo",
        "feature/new",
        "main",
        "New feature request",
    )
    .await;

    assert!(result.is_ok());
    let mr = result.unwrap();

    assert_eq!(mr.id, Some(100));
    assert_eq!(mr.source_ref.as_deref(), Some("feature/new"));
    assert_eq!(mr.target_ref.as_deref(), Some("main"));
    assert_eq!(mr.subject.as_deref(), Some("New feature request"));
    assert_eq!(mr.status.as_deref(), Some("open"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_comment_merge_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/42/comment")
        .match_body(
            "<merge-request-comment><content>This looks good!</content></merge-request-comment>",
        )
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

    let result =
        comment_merge_request(&client, "test-project", "test-repo", 42, "This looks good!").await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_close_merge_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/15/close")
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

    let result = close_merge_request(&client, "test-project", "test-repo", 15).await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_reopen_merge_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/20/reopen")
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

    let result = reopen_merge_request(&client, "test-project", "test-repo", 20).await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_merge_merge_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/30/merge")
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

    let result = merge_merge_request(&client, "test-project", "test-repo", 30).await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_reassign_merge_request() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/25/reassign")
        .match_body("<merge-request><user-id>456</user-id></merge-request>")
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

    let result = reassign_merge_request(&client, "test-project", "test-repo", 25, 456).await;

    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_merge_requests_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent/repo/merge_requests")
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

    let result = list_merge_requests(&client, "nonexistent", "repo").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_merge_request_error_404() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/test-repo/merge_requests/999")
        .with_status(404)
        .with_body("Merge request not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = show_merge_request(&client, "test-project", "test-repo", 999).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_merge_request_error_422() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests")
        .with_status(422)
        .with_body("Invalid merge request data")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_merge_request(
        &client,
        "test-project",
        "test-repo",
        "invalid-branch",
        "main",
        "Test",
    )
    .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("422"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_merge_merge_request_error_409() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/test-repo/merge_requests/50/merge")
        .with_status(409)
        .with_body("Cannot merge: conflicts exist")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = merge_merge_request(&client, "test-project", "test-repo", 50).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("409"));

    mock.assert_async().await;
}
