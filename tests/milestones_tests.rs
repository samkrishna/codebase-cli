use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::milestones::{create_milestone, list_milestones, update_milestone};

#[tokio::test]
async fn test_list_milestones() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/milestones")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestones>
  <ticketing-milestone>
    <id>1</id>
    <name>Version 1.0</name>
    <description>First release</description>
    <start-at>2026-01-01</start-at>
    <deadline>2026-03-31</deadline>
    <parent-id>0</parent-id>
    <estimated-time>120.5</estimated-time>
    <responsible-user-id>42</responsible-user-id>
    <status>active</status>
  </ticketing-milestone>
  <ticketing-milestone>
    <id>2</id>
    <name>Version 2.0</name>
    <description>Second release</description>
    <start-at>2026-04-01</start-at>
    <deadline>2026-06-30</deadline>
    <parent-id>1</parent-id>
    <estimated-time>80.0</estimated-time>
    <responsible-user-id>43</responsible-user-id>
    <status>completed</status>
  </ticketing-milestone>
</ticketing-milestones>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestones = list_milestones(&client, "test-project").await.unwrap();

    mock.assert_async().await;
    assert_eq!(milestones.len(), 2);

    assert_eq!(milestones[0].id, Some(1));
    assert_eq!(milestones[0].name, Some("Version 1.0".to_string()));
    assert_eq!(milestones[0].description, Some("First release".to_string()));
    assert_eq!(milestones[0].start_at, Some("2026-01-01".to_string()));
    assert_eq!(milestones[0].deadline, Some("2026-03-31".to_string()));
    assert_eq!(milestones[0].parent_id, Some(0));
    assert_eq!(milestones[0].estimated_time, Some(120.5));
    assert_eq!(milestones[0].responsible_user_id, Some(42));
    assert_eq!(milestones[0].status, Some("active".to_string()));

    assert_eq!(milestones[1].id, Some(2));
    assert_eq!(milestones[1].name, Some("Version 2.0".to_string()));
    assert_eq!(milestones[1].status, Some("completed".to_string()));
}

#[tokio::test]
async fn test_list_milestones_empty() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/milestones")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestones>
</ticketing-milestones>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestones = list_milestones(&client, "test-project").await.unwrap();

    mock.assert_async().await;
    assert_eq!(milestones.len(), 0);
}

#[tokio::test]
async fn test_create_milestone_all_fields() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/milestones")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestone>
  <id>3</id>
  <name>Sprint 1</name>
  <description>First sprint of the quarter</description>
  <start-at>2026-02-01</start-at>
  <deadline>2026-02-14</deadline>
  <parent-id>5</parent-id>
  <estimated-time>40.0</estimated-time>
  <responsible-user-id>100</responsible-user-id>
  <status>active</status>
</ticketing-milestone>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestone = create_milestone(
        &client,
        "test-project",
        "Sprint 1",
        Some("First sprint of the quarter"),
        Some("2026-02-01"),
        Some("2026-02-14"),
        Some(100),
        Some(5),
        Some("active"),
    )
    .await
    .unwrap();

    mock.assert_async().await;
    assert_eq!(milestone.id, Some(3));
    assert_eq!(milestone.name, Some("Sprint 1".to_string()));
    assert_eq!(
        milestone.description,
        Some("First sprint of the quarter".to_string())
    );
    assert_eq!(milestone.start_at, Some("2026-02-01".to_string()));
    assert_eq!(milestone.deadline, Some("2026-02-14".to_string()));
    assert_eq!(milestone.parent_id, Some(5));
    assert_eq!(milestone.estimated_time, Some(40.0));
    assert_eq!(milestone.responsible_user_id, Some(100));
    assert_eq!(milestone.status, Some("active".to_string()));
}

#[tokio::test]
async fn test_create_milestone_minimal_fields() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/milestones")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestone>
  <id>4</id>
  <name>Quick Milestone</name>
</ticketing-milestone>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestone = create_milestone(
        &client,
        "test-project",
        "Quick Milestone",
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    mock.assert_async().await;
    assert_eq!(milestone.id, Some(4));
    assert_eq!(milestone.name, Some("Quick Milestone".to_string()));
    assert_eq!(milestone.description, None);
    assert_eq!(milestone.start_at, None);
    assert_eq!(milestone.deadline, None);
    assert_eq!(milestone.parent_id, None);
    assert_eq!(milestone.estimated_time, None);
    assert_eq!(milestone.responsible_user_id, None);
    assert_eq!(milestone.status, None);
}

#[tokio::test]
async fn test_update_milestone_name_and_status() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/test-project/milestones/10")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestone>
  <id>10</id>
  <name>Updated Milestone Name</name>
  <description>Original description</description>
  <start-at>2026-01-15</start-at>
  <deadline>2026-02-28</deadline>
  <parent-id>2</parent-id>
  <estimated-time>60.0</estimated-time>
  <responsible-user-id>50</responsible-user-id>
  <status>completed</status>
</ticketing-milestone>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestone = update_milestone(
        &client,
        "test-project",
        10,
        Some("Updated Milestone Name"),
        None,
        None,
        None,
        None,
        None,
        Some("completed"),
    )
    .await
    .unwrap();

    mock.assert_async().await;
    assert_eq!(milestone.id, Some(10));
    assert_eq!(milestone.name, Some("Updated Milestone Name".to_string()));
    assert_eq!(milestone.status, Some("completed".to_string()));
    assert_eq!(
        milestone.description,
        Some("Original description".to_string())
    );
}

#[tokio::test]
async fn test_update_milestone_all_fields() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/test-project/milestones/20")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestone>
  <id>20</id>
  <name>Completely Updated</name>
  <description>New description</description>
  <start-at>2026-03-01</start-at>
  <deadline>2026-04-30</deadline>
  <parent-id>3</parent-id>
  <estimated-time>90.5</estimated-time>
  <responsible-user-id>75</responsible-user-id>
  <status>active</status>
</ticketing-milestone>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestone = update_milestone(
        &client,
        "test-project",
        20,
        Some("Completely Updated"),
        Some("New description"),
        Some("2026-03-01"),
        Some("2026-04-30"),
        Some(75),
        Some(3),
        Some("active"),
    )
    .await
    .unwrap();

    mock.assert_async().await;
    assert_eq!(milestone.id, Some(20));
    assert_eq!(milestone.name, Some("Completely Updated".to_string()));
    assert_eq!(milestone.description, Some("New description".to_string()));
    assert_eq!(milestone.start_at, Some("2026-03-01".to_string()));
    assert_eq!(milestone.deadline, Some("2026-04-30".to_string()));
    assert_eq!(milestone.parent_id, Some(3));
    assert_eq!(milestone.estimated_time, Some(90.5));
    assert_eq!(milestone.responsible_user_id, Some(75));
    assert_eq!(milestone.status, Some("active".to_string()));
}

#[tokio::test]
async fn test_update_milestone_no_fields() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/test-project/milestones/30")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ticketing-milestone>
  <id>30</id>
  <name>Unchanged Milestone</name>
  <description>Still the same</description>
  <status>active</status>
</ticketing-milestone>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let milestone = update_milestone(
        &client,
        "test-project",
        30,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    mock.assert_async().await;
    assert_eq!(milestone.id, Some(30));
    assert_eq!(milestone.name, Some("Unchanged Milestone".to_string()));
    assert_eq!(milestone.description, Some("Still the same".to_string()));
}

#[tokio::test]
async fn test_list_milestones_error_404() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent-project/milestones")
        .with_status(404)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<error>Project not found</error>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_milestones(&client, "nonexistent-project").await;

    mock.assert_async().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("404"));
}

#[tokio::test]
async fn test_create_milestone_error_400() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/milestones")
        .with_status(400)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<error>Invalid milestone data</error>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_milestone(
        &client,
        "test-project",
        "",
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    mock.assert_async().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("400"));
}

#[tokio::test]
async fn test_update_milestone_error_403() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/test-project/milestones/99")
        .with_status(403)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<error>Forbidden</error>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = update_milestone(
        &client,
        "test-project",
        99,
        Some("Should fail"),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    mock.assert_async().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("403"));
}
