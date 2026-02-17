use codebase_cli::api::client::CodebaseClient;
use codebase_cli::api::models::NoteChanges;
use codebase_cli::api::tickets::*;
use mockito;

#[tokio::test]
async fn test_list_tickets() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<tickets>
                <ticket>
                    <ticket-id>123</ticket-id>
                    <summary>Test ticket</summary>
                    <ticket-type>bug</ticket-type>
                    <priority-id>2</priority-id>
                    <status-id>1</status-id>
                    <assignee>John Doe</assignee>
                </ticket>
                <ticket>
                    <ticket-id>124</ticket-id>
                    <summary>Another ticket</summary>
                    <ticket-type>feature</ticket-type>
                    <priority-id>3</priority-id>
                    <status-id>2</status-id>
                </ticket>
            </tickets>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_tickets(&client, "test-project").await;
    assert!(result.is_ok());
    let tickets = result.unwrap();
    assert_eq!(tickets.len(), 2);
    assert_eq!(tickets[0].ticket_id, Some(123));
    assert_eq!(tickets[0].summary, Some("Test ticket".to_string()));
    assert_eq!(tickets[0].ticket_type, Some("bug".to_string()));
    assert_eq!(tickets[1].ticket_id, Some(124));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_search_tickets() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets?query=status%3Aopen")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<tickets>
                <ticket>
                    <ticket-id>456</ticket-id>
                    <summary>Search result ticket</summary>
                    <ticket-type>task</ticket-type>
                    <description>Test description</description>
                </ticket>
            </tickets>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = search_tickets(&client, "test-project", "status:open").await;
    assert!(result.is_ok());
    let tickets = result.unwrap();
    assert_eq!(tickets.len(), 1);
    assert_eq!(tickets[0].ticket_id, Some(456));
    assert_eq!(tickets[0].summary, Some("Search result ticket".to_string()));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_ticket() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/tickets")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticket>
                <ticket-id>789</ticket-id>
                <summary>Newly created ticket</summary>
                <ticket-type>bug</ticket-type>
                <priority-id>1</priority-id>
                <status-id>1</status-id>
                <description>This is a new ticket</description>
                <assignee-id>42</assignee-id>
            </ticket>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = create_ticket(
        &client,
        "test-project",
        "Newly created ticket",
        "bug",
        Some(1),
        Some(1),
        Some("This is a new ticket"),
        Some(42),
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
    let ticket = result.unwrap();
    assert_eq!(ticket.ticket_id, Some(789));
    assert_eq!(ticket.summary, Some("Newly created ticket".to_string()));
    assert_eq!(ticket.ticket_type, Some("bug".to_string()));
    assert_eq!(ticket.priority_id, Some(1));
    assert_eq!(ticket.assignee_id, Some(42));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_ticket_notes() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/123/notes")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticket-notes>
                <ticket-note>
                    <id>1</id>
                    <content>First note</content>
                    <time-added>2024-01-15T10:00:00Z</time-added>
                    <private>0</private>
                </ticket-note>
                <ticket-note>
                    <id>2</id>
                    <content>Second note</content>
                    <time-added>2024-01-16T11:00:00Z</time-added>
                    <private>1</private>
                </ticket-note>
            </ticket-notes>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_ticket_notes(&client, "test-project", 123).await;
    assert!(result.is_ok());
    let notes = result.unwrap();
    assert_eq!(notes.len(), 2);
    assert_eq!(notes[0].id, Some(1));
    assert_eq!(notes[0].content, Some("First note".to_string()));
    assert_eq!(notes[0].private, Some(false));
    assert_eq!(notes[1].id, Some(2));
    assert_eq!(notes[1].private, Some(true));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_create_ticket_note() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/tickets/123/notes")
        .with_status(201)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticket-note>
                <id>3</id>
                <content>New note content</content>
                <time-added>2024-01-17T12:00:00Z</time-added>
                <private>1</private>
                <changes>
                    <status-id>2</status-id>
                    <priority-id>3</priority-id>
                </changes>
            </ticket-note>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let changes = NoteChanges {
        status_id: Some(2),
        priority_id: Some(3),
        category_id: None,
        assignee_id: None,
        milestone_id: None,
        subject: None,
    };

    let result = create_ticket_note(
        &client,
        "test-project",
        123,
        Some("New note content"),
        Some(&changes),
        true,
    )
    .await;

    assert!(result.is_ok());
    let note = result.unwrap();
    assert_eq!(note.id, Some(3));
    assert_eq!(note.content, Some("New note content".to_string()));
    assert_eq!(note.private, Some(true));
    assert!(note.changes.is_some());
    let note_changes = note.changes.unwrap();
    assert_eq!(note_changes.status_id, Some(2));
    assert_eq!(note_changes.priority_id, Some(3));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_watchers() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/123/watchers")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<watchers>
                <watcher><watcher>42</watcher></watcher>
                <watcher><watcher>43</watcher></watcher>
                <watcher><watcher>44</watcher></watcher>
            </watchers>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_watchers(&client, "test-project", 123).await;
    assert!(result.is_ok());
    let watchers = result.unwrap();
    assert_eq!(watchers.len(), 3);
    assert_eq!(watchers[0].watcher, Some(42));
    assert_eq!(watchers[1].watcher, Some(43));
    assert_eq!(watchers[2].watcher, Some(44));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_set_watchers() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("POST", "/test-project/tickets/123/watchers")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body("")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let user_ids = vec![42, 43, 44];
    let result = set_watchers(&client, "test-project", 123, &user_ids).await;
    assert!(result.is_ok());

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_statuses() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/statuses")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticketing-statuses>
                <ticketing-status>
                    <id>1</id>
                    <name>Open</name>
                    <background-colour>#FF0000</background-colour>
                    <order>1</order>
                    <treat-as-closed>0</treat-as-closed>
                </ticketing-status>
                <ticketing-status>
                    <id>2</id>
                    <name>Closed</name>
                    <background-colour>#00FF00</background-colour>
                    <order>2</order>
                    <treat-as-closed>1</treat-as-closed>
                </ticketing-status>
            </ticketing-statuses>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_statuses(&client, "test-project").await;
    assert!(result.is_ok());
    let statuses = result.unwrap();
    assert_eq!(statuses.len(), 2);
    assert_eq!(statuses[0].id, Some(1));
    assert_eq!(statuses[0].name, Some("Open".to_string()));
    assert_eq!(statuses[0].treat_as_closed, Some(false));
    assert_eq!(statuses[1].id, Some(2));
    assert_eq!(statuses[1].name, Some("Closed".to_string()));
    assert_eq!(statuses[1].treat_as_closed, Some(true));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_priorities() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/priorities")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticketing-priorities>
                <ticketing-priority>
                    <id>1</id>
                    <name>High</name>
                    <colour>#FF0000</colour>
                    <default>0</default>
                    <position>1</position>
                </ticketing-priority>
                <ticketing-priority>
                    <id>2</id>
                    <name>Medium</name>
                    <colour>#FFFF00</colour>
                    <default>1</default>
                    <position>2</position>
                </ticketing-priority>
                <ticketing-priority>
                    <id>3</id>
                    <name>Low</name>
                    <colour>#00FF00</colour>
                    <default>0</default>
                    <position>3</position>
                </ticketing-priority>
            </ticketing-priorities>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_priorities(&client, "test-project").await;
    assert!(result.is_ok());
    let priorities = result.unwrap();
    assert_eq!(priorities.len(), 3);
    assert_eq!(priorities[0].id, Some(1));
    assert_eq!(priorities[0].name, Some("High".to_string()));
    assert_eq!(priorities[0].colour, Some("#FF0000".to_string()));
    assert_eq!(priorities[0].default, Some(false));
    assert_eq!(priorities[1].id, Some(2));
    assert_eq!(priorities[1].default, Some(true));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_categories() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/categories")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticketing-categories>
                <ticketing-category>
                    <id>1</id>
                    <name>Backend</name>
                </ticketing-category>
                <ticketing-category>
                    <id>2</id>
                    <name>Frontend</name>
                </ticketing-category>
                <ticketing-category>
                    <id>3</id>
                    <name>Infrastructure</name>
                </ticketing-category>
            </ticketing-categories>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_categories(&client, "test-project").await;
    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 3);
    assert_eq!(categories[0].id, Some(1));
    assert_eq!(categories[0].name, Some("Backend".to_string()));
    assert_eq!(categories[1].id, Some(2));
    assert_eq!(categories[1].name, Some("Frontend".to_string()));
    assert_eq!(categories[2].id, Some(3));
    assert_eq!(categories[2].name, Some("Infrastructure".to_string()));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_list_types() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets/types")
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body(
            r#"<ticketing-types>
                <ticketing-type>
                    <id>1</id>
                    <name>Bug</name>
                    <icon>bug-icon</icon>
                </ticketing-type>
                <ticketing-type>
                    <id>2</id>
                    <name>Feature</name>
                    <icon>feature-icon</icon>
                </ticketing-type>
                <ticketing-type>
                    <id>3</id>
                    <name>Task</name>
                    <icon>task-icon</icon>
                </ticketing-type>
            </ticketing-types>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_types(&client, "test-project").await;
    assert!(result.is_ok());
    let types = result.unwrap();
    assert_eq!(types.len(), 3);
    assert_eq!(types[0].id, Some(1));
    assert_eq!(types[0].name, Some("Bug".to_string()));
    assert_eq!(types[0].icon, Some("bug-icon".to_string()));
    assert_eq!(types[1].id, Some(2));
    assert_eq!(types[1].name, Some("Feature".to_string()));
    assert_eq!(types[2].id, Some(3));
    assert_eq!(types[2].name, Some("Task".to_string()));

    mock.assert_async().await;
}

#[tokio::test]
async fn test_api_error() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/test-project/tickets")
        .with_status(404)
        .with_header("content-type", "application/xml")
        .with_body(r#"<error>Project not found</error>"#)
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = list_tickets(&client, "test-project").await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("404"));

    mock.assert_async().await;
}
