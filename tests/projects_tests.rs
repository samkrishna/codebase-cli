use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::projects::*;

#[tokio::test]
async fn test_list_projects() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/projects")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<projects>
  <project>
    <name>Project Alpha</name>
    <account-name>testaccount</account-name>
    <permalink>project-alpha</permalink>
    <group-id>1</group-id>
    <overview>Alpha project overview</overview>
    <start-page>overview</start-page>
    <status>active</status>
    <icon>0</icon>
    <total-tickets>100</total-tickets>
    <open-tickets>25</open-tickets>
    <closed-tickets>75</closed-tickets>
  </project>
  <project>
    <name>Project Beta</name>
    <account-name>testaccount</account-name>
    <permalink>project-beta</permalink>
    <group-id>2</group-id>
    <overview>Beta project overview</overview>
    <start-page>tickets</start-page>
    <status>archived</status>
    <icon>1</icon>
    <total-tickets>50</total-tickets>
    <open-tickets>10</open-tickets>
    <closed-tickets>40</closed-tickets>
  </project>
</projects>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let projects = list_projects(&client).await.unwrap();

    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].name.as_deref(), Some("Project Alpha"));
    assert_eq!(projects[0].permalink.as_deref(), Some("project-alpha"));
    assert_eq!(projects[0].account_name.as_deref(), Some("testaccount"));
    assert_eq!(projects[0].group_id, Some(1));
    assert_eq!(projects[0].status.as_deref(), Some("active"));
    assert_eq!(projects[0].total_tickets, Some(100));
    assert_eq!(projects[0].open_tickets, Some(25));
    assert_eq!(projects[0].closed_tickets, Some(75));

    assert_eq!(projects[1].name.as_deref(), Some("Project Beta"));
    assert_eq!(projects[1].permalink.as_deref(), Some("project-beta"));
    assert_eq!(projects[1].status.as_deref(), Some("archived"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_project() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/my-project")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project>
  <name>My Project</name>
  <account-name>myaccount</account-name>
  <permalink>my-project</permalink>
  <group-id>5</group-id>
  <overview>This is my project</overview>
  <start-page>overview</start-page>
  <status>active</status>
  <icon>2</icon>
  <total-tickets>200</total-tickets>
  <open-tickets>50</open-tickets>
  <closed-tickets>150</closed-tickets>
</project>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "myaccount".to_string(),
        "myuser".to_string(),
        "mykey".to_string(),
    );

    let project = show_project(&client, "my-project").await.unwrap();

    assert_eq!(project.name.as_deref(), Some("My Project"));
    assert_eq!(project.permalink.as_deref(), Some("my-project"));
    assert_eq!(project.account_name.as_deref(), Some("myaccount"));
    assert_eq!(project.group_id, Some(5));
    assert_eq!(project.overview.as_deref(), Some("This is my project"));
    assert_eq!(project.start_page.as_deref(), Some("overview"));
    assert_eq!(project.status.as_deref(), Some("active"));
    assert_eq!(project.icon, Some(2));
    assert_eq!(project.total_tickets, Some(200));
    assert_eq!(project.open_tickets, Some(50));
    assert_eq!(project.closed_tickets, Some(150));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_project() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/create_project")
        .match_body("<project><name>New Project</name></project>")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project>
  <name>New Project</name>
  <account-name>testaccount</account-name>
  <permalink>new-project</permalink>
  <group-id>1</group-id>
  <status>active</status>
  <icon>0</icon>
  <total-tickets>0</total-tickets>
  <open-tickets>0</open-tickets>
  <closed-tickets>0</closed-tickets>
</project>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let project = create_project(&client, "New Project").await.unwrap();

    assert_eq!(project.name.as_deref(), Some("New Project"));
    assert_eq!(project.permalink.as_deref(), Some("new-project"));
    assert_eq!(project.account_name.as_deref(), Some("testaccount"));
    assert_eq!(project.status.as_deref(), Some("active"));
    assert_eq!(project.total_tickets, Some(0));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_project_name() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/project/123")
        .match_body("<project><name>Updated Name</name></project>")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project>
  <name>Updated Name</name>
  <account-name>testaccount</account-name>
  <permalink>my-project</permalink>
  <group-id>1</group-id>
  <status>active</status>
  <icon>0</icon>
  <total-tickets>10</total-tickets>
  <open-tickets>5</open-tickets>
  <closed-tickets>5</closed-tickets>
</project>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let project = update_project(&client, "123", Some("Updated Name"), None)
        .await
        .unwrap();

    assert_eq!(project.name.as_deref(), Some("Updated Name"));
    assert_eq!(project.status.as_deref(), Some("active"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_project_status() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/project/456")
        .match_body("<project><status>archived</status></project>")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project>
  <name>Test Project</name>
  <account-name>testaccount</account-name>
  <permalink>test-project</permalink>
  <group-id>1</group-id>
  <status>archived</status>
  <icon>0</icon>
  <total-tickets>10</total-tickets>
  <open-tickets>0</open-tickets>
  <closed-tickets>10</closed-tickets>
</project>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let project = update_project(&client, "456", None, Some("archived"))
        .await
        .unwrap();

    assert_eq!(project.name.as_deref(), Some("Test Project"));
    assert_eq!(project.status.as_deref(), Some("archived"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_update_project_name_and_status() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("PUT", "/project/789")
        .match_body("<project><name>Both Updated</name><status>archived</status></project>")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project>
  <name>Both Updated</name>
  <account-name>testaccount</account-name>
  <permalink>both-updated</permalink>
  <group-id>2</group-id>
  <status>archived</status>
  <icon>1</icon>
  <total-tickets>25</total-tickets>
  <open-tickets>0</open-tickets>
  <closed-tickets>25</closed-tickets>
</project>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let project = update_project(&client, "789", Some("Both Updated"), Some("archived"))
        .await
        .unwrap();

    assert_eq!(project.name.as_deref(), Some("Both Updated"));
    assert_eq!(project.status.as_deref(), Some("archived"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_project() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("DELETE", "/project-to-delete")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let result = delete_project(&client, "project-to-delete").await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_project_groups() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/project_groups")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<project-groups>
  <project-group>
    <id>1</id>
    <label>Development</label>
  </project-group>
  <project-group>
    <id>2</id>
    <label>Production</label>
  </project-group>
  <project-group>
    <id>3</id>
    <label>Internal</label>
  </project-group>
</project-groups>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let groups = list_project_groups(&client).await.unwrap();

    assert_eq!(groups.len(), 3);
    assert_eq!(groups[0].id, Some(1));
    assert_eq!(groups[0].label.as_deref(), Some("Development"));
    assert_eq!(groups[1].id, Some(2));
    assert_eq!(groups[1].label.as_deref(), Some("Production"));
    assert_eq!(groups[2].id, Some(3));
    assert_eq!(groups[2].label.as_deref(), Some("Internal"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_project_users() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/my-project/assignments")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<users>
  <user>
    <id>101</id>
    <first-name>John</first-name>
    <last-name>Doe</last-name>
    <username>johndoe</username>
    <email-address>john.doe@example.com</email-address>
    <company>Acme Corp</company>
  </user>
  <user>
    <id>102</id>
    <first-name>Jane</first-name>
    <last-name>Smith</last-name>
    <username>janesmith</username>
    <email-address>jane.smith@example.com</email-address>
    <company>Tech Inc</company>
  </user>
</users>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let users = list_project_users(&client, "my-project").await.unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id, Some(101));
    assert_eq!(users[0].first_name.as_deref(), Some("John"));
    assert_eq!(users[0].last_name.as_deref(), Some("Doe"));
    assert_eq!(users[0].username.as_deref(), Some("johndoe"));
    assert_eq!(
        users[0].email_address.as_deref(),
        Some("john.doe@example.com")
    );
    assert_eq!(users[0].company.as_deref(), Some("Acme Corp"));

    assert_eq!(users[1].id, Some(102));
    assert_eq!(users[1].first_name.as_deref(), Some("Jane"));
    assert_eq!(users[1].last_name.as_deref(), Some("Smith"));
    assert_eq!(users[1].username.as_deref(), Some("janesmith"));
    assert_eq!(
        users[1].email_address.as_deref(),
        Some("jane.smith@example.com")
    );
    assert_eq!(users[1].company.as_deref(), Some("Tech Inc"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_assign_project_users() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/my-project/assignments")
        .match_body("<users><user><id>201</id></user><user><id>202</id></user><user><id>203</id></user></users>")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let user_ids = vec![201, 202, 203];
    let result = assign_project_users(&client, "my-project", &user_ids).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_assign_project_users_single_user() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/assignments")
        .match_body("<users><user><id>999</id></user></users>")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let user_ids = vec![999];
    let result = assign_project_users(&client, "test-project", &user_ids).await;

    assert!(result.is_ok());
    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_projects_error_handling() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/projects")
        .with_status(401)
        .with_body("Unauthorized: Invalid API credentials")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "baduser".to_string(),
        "badkey".to_string(),
    );

    let result = list_projects(&client).await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("401") || error_msg.contains("API error"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_show_project_not_found() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent-project")
        .with_status(404)
        .with_body("Project not found")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let result = show_project(&client, "nonexistent-project").await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("404") || error_msg.contains("API error"));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_delete_project_forbidden() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("DELETE", "/protected-project")
        .with_status(403)
        .with_body("Forbidden: You do not have permission to delete this project")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "testaccount".to_string(),
        "testuser".to_string(),
        "testkey".to_string(),
    );

    let result = delete_project(&client, "protected-project").await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("403") || error_msg.contains("API error"));

    mock.assert_async().await;
}
