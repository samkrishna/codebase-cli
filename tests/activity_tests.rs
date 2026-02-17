use codebase_cli::api::activity::{account_activity, project_activity};
use codebase_cli::api::client::CodebaseClient;
use mockito;

#[tokio::test]
async fn test_account_activity_no_params() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Commit pushed</title>
                    <type>commit</type>
                    <timestamp>2026-02-15T10:30:00Z</timestamp>
                    <html-title>Commit to main</html-title>
                    <html-text>Added new feature</html-text>
                    <project-permalink>myproject</project-permalink>
                    <project-name>My Project</project-name>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = account_activity(&client, false, None, None)
        .await
        .expect("Failed to fetch account activity");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Commit pushed".to_string()));
    assert_eq!(events[0].event_type, Some("commit".to_string()));
    assert_eq!(
        events[0].timestamp,
        Some("2026-02-15T10:30:00Z".to_string())
    );
    assert_eq!(events[0].html_title, Some("Commit to main".to_string()));
    assert_eq!(events[0].html_text, Some("Added new feature".to_string()));
    assert_eq!(events[0].project_permalink, Some("myproject".to_string()));
    assert_eq!(events[0].project_name, Some("My Project".to_string()));
}

#[tokio::test]
async fn test_account_activity_with_raw_true() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity?raw=true")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Raw event</title>
                    <type>ticket</type>
                    <timestamp>2026-02-14T15:00:00Z</timestamp>
                    <html-title>Ticket created</html-title>
                    <html-text>New bug report</html-text>
                    <subject>Bug in login</subject>
                    <number>42</number>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = account_activity(&client, true, None, None)
        .await
        .expect("Failed to fetch account activity with raw=true");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Raw event".to_string()));
    assert_eq!(events[0].event_type, Some("ticket".to_string()));
    assert_eq!(events[0].subject, Some("Bug in login".to_string()));
    assert_eq!(events[0].number, Some(42));
}

#[tokio::test]
async fn test_account_activity_with_since_parameter() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity?since=2026-02-01")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Recent activity</title>
                    <type>deployment</type>
                    <timestamp>2026-02-10T09:00:00Z</timestamp>
                    <html-title>Deployed to production</html-title>
                    <html-text>Version 1.2.0</html-text>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = account_activity(&client, false, Some("2026-02-01"), None)
        .await
        .expect("Failed to fetch account activity with since parameter");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Recent activity".to_string()));
    assert_eq!(events[0].event_type, Some("deployment".to_string()));
}

#[tokio::test]
async fn test_account_activity_with_page_parameter() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity?page=2")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Page 2 event</title>
                    <type>comment</type>
                    <timestamp>2026-02-08T14:30:00Z</timestamp>
                    <html-title>Comment added</html-title>
                    <html-text>Great work!</html-text>
                    <content>This looks good to me.</content>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = account_activity(&client, false, None, Some(2))
        .await
        .expect("Failed to fetch account activity with page parameter");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Page 2 event".to_string()));
    assert_eq!(events[0].event_type, Some("comment".to_string()));
    assert_eq!(
        events[0].content,
        Some("This looks good to me.".to_string())
    );
}

#[tokio::test]
async fn test_project_activity_no_params() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/myproject/activity")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Project event</title>
                    <type>push</type>
                    <timestamp>2026-02-16T08:00:00Z</timestamp>
                    <html-title>Push to repository</html-title>
                    <html-text>5 commits pushed</html-text>
                    <project-permalink>myproject</project-permalink>
                    <project-name>My Project</project-name>
                    <name>feature-branch</name>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = project_activity(&client, "myproject", false, None, None)
        .await
        .expect("Failed to fetch project activity");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Project event".to_string()));
    assert_eq!(events[0].event_type, Some("push".to_string()));
    assert_eq!(events[0].name, Some("feature-branch".to_string()));
}

#[tokio::test]
async fn test_project_activity_with_all_params() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/testproject/activity?raw=true&since=2026-01-15&page=3",
        )
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Combined params event</title>
                    <type>milestone</type>
                    <timestamp>2026-02-05T12:00:00Z</timestamp>
                    <html-title>Milestone completed</html-title>
                    <html-text>Sprint 5 finished</html-text>
                    <project-permalink>testproject</project-permalink>
                    <project-name>Test Project</project-name>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = project_activity(&client, "testproject", true, Some("2026-01-15"), Some(3))
        .await
        .expect("Failed to fetch project activity with all params");

    mock.assert_async().await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, Some("Combined params event".to_string()));
    assert_eq!(events[0].event_type, Some("milestone".to_string()));
}

#[tokio::test]
async fn test_multiple_events_different_types() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity")
        .with_status(200)
        .with_body(
            r#"<events>
                <event>
                    <title>Commit event</title>
                    <type>commit</type>
                    <timestamp>2026-02-16T10:00:00Z</timestamp>
                    <html-title>Commit pushed</html-title>
                    <html-text>Fixed bug #123</html-text>
                    <project-permalink>proj1</project-permalink>
                    <project-name>Project One</project-name>
                </event>
                <event>
                    <title>Ticket created</title>
                    <type>ticket</type>
                    <timestamp>2026-02-16T09:30:00Z</timestamp>
                    <html-title>New ticket</html-title>
                    <html-text>Report login issue</html-text>
                    <subject>Login fails on Safari</subject>
                    <number>101</number>
                    <project-permalink>proj2</project-permalink>
                    <project-name>Project Two</project-name>
                </event>
                <event>
                    <title>Comment added</title>
                    <type>comment</type>
                    <timestamp>2026-02-16T09:00:00Z</timestamp>
                    <html-title>Comment on issue</html-title>
                    <html-text>I can reproduce this</html-text>
                    <content>Confirmed on my machine too</content>
                    <project-permalink>proj2</project-permalink>
                    <project-name>Project Two</project-name>
                </event>
            </events>"#,
        )
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let events = account_activity(&client, false, None, None)
        .await
        .expect("Failed to fetch multiple events");

    mock.assert_async().await;
    assert_eq!(events.len(), 3);

    // First event: commit
    assert_eq!(events[0].title, Some("Commit event".to_string()));
    assert_eq!(events[0].event_type, Some("commit".to_string()));
    assert_eq!(events[0].project_name, Some("Project One".to_string()));

    // Second event: ticket
    assert_eq!(events[1].title, Some("Ticket created".to_string()));
    assert_eq!(events[1].event_type, Some("ticket".to_string()));
    assert_eq!(events[1].subject, Some("Login fails on Safari".to_string()));
    assert_eq!(events[1].number, Some(101));

    // Third event: comment
    assert_eq!(events[2].title, Some("Comment added".to_string()));
    assert_eq!(events[2].event_type, Some("comment".to_string()));
    assert_eq!(
        events[2].content,
        Some("Confirmed on my machine too".to_string())
    );
}

#[tokio::test]
async fn test_account_activity_error_response() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/activity")
        .with_status(401)
        .with_body("<error>Unauthorized</error>")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "invalid-key".to_string(),
    );

    let result = account_activity(&client, false, None, None).await;

    mock.assert_async().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("401") || err_msg.contains("Unauthorized"));
}

#[tokio::test]
async fn test_project_activity_error_response() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/nonexistent/activity")
        .with_status(404)
        .with_body("<error>Project not found</error>")
        .create_async()
        .await;

    let client = CodebaseClient::with_base_url(
        server.url(),
        "test-account".to_string(),
        "test-user".to_string(),
        "test-key".to_string(),
    );

    let result = project_activity(&client, "nonexistent", false, None, None).await;

    mock.assert_async().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("404") || err_msg.contains("not found"));
}
