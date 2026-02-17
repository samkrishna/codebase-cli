use codebase_cli::api::models::*;
use codebase_cli::output;

// ── JSON serialization tests ──

#[test]
fn test_project_serializes_to_json() {
    let project = Project {
        name: Some("My Project".to_string()),
        account_name: Some("myaccount".to_string()),
        permalink: Some("my-project".to_string()),
        project_id: Some(42),
        group_id: Some(7),
        overview: Some("A great project".to_string()),
        start_page: Some("overview".to_string()),
        status: Some("active".to_string()),
        icon: Some(1),
        disk_usage: Some(1024),
        total_tickets: Some(100),
        open_tickets: Some(25),
        closed_tickets: Some(75),
    };

    let json = serde_json::to_string_pretty(&project).expect("Project should serialize to JSON");

    assert!(!json.is_empty());
    assert!(json.contains("\"name\""));
    assert!(json.contains("My Project"));
    assert!(json.contains("\"account-name\""));
    assert!(json.contains("myaccount"));
    assert!(json.contains("\"permalink\""));
    assert!(json.contains("my-project"));
    assert!(json.contains("\"project-id\""));
    assert!(json.contains("42"));
    assert!(json.contains("\"group-id\""));
    assert!(json.contains("\"status\""));
    assert!(json.contains("active"));
    assert!(json.contains("\"total-tickets\""));
    assert!(json.contains("100"));
    assert!(json.contains("\"open-tickets\""));
    assert!(json.contains("25"));
    assert!(json.contains("\"closed-tickets\""));
    assert!(json.contains("75"));
}

#[test]
fn test_project_with_none_fields_serializes_to_json() {
    let project = Project {
        name: Some("Minimal Project".to_string()),
        account_name: None,
        permalink: None,
        project_id: None,
        group_id: None,
        overview: None,
        start_page: None,
        status: None,
        icon: None,
        disk_usage: None,
        total_tickets: None,
        open_tickets: None,
        closed_tickets: None,
    };

    let json =
        serde_json::to_string_pretty(&project).expect("Project with None fields should serialize");

    assert!(!json.is_empty());
    assert!(json.contains("\"name\""));
    assert!(json.contains("Minimal Project"));
}

#[test]
fn test_ticket_list_serializes_to_json() {
    let tickets = vec![
        Ticket {
            ticket_id: Some(1),
            summary: Some("Fix the bug".to_string()),
            ticket_type: Some("bug".to_string()),
            description: Some("There is a bug that needs fixing".to_string()),
            priority_id: Some(2),
            status_id: Some(1),
            category_id: Some(3),
            milestone_id: Some(5),
            assignee_id: Some(101),
            reporter_id: Some(102),
            assignee: Some("johndoe".to_string()),
            reporter: Some("janesmith".to_string()),
            tags: Some("backend,urgent".to_string()),
        },
        Ticket {
            ticket_id: Some(2),
            summary: Some("Add new feature".to_string()),
            ticket_type: Some("feature".to_string()),
            description: Some("A new feature is needed".to_string()),
            priority_id: Some(1),
            status_id: Some(2),
            category_id: None,
            milestone_id: None,
            assignee_id: None,
            reporter_id: Some(103),
            assignee: None,
            reporter: Some("alice".to_string()),
            tags: None,
        },
    ];

    let json =
        serde_json::to_string_pretty(&tickets).expect("Ticket list should serialize to JSON");

    assert!(!json.is_empty());
    // Verify it's a JSON array
    let trimmed = json.trim();
    assert!(trimmed.starts_with('['));
    assert!(trimmed.ends_with(']'));
    // Verify ticket fields use renamed keys
    assert!(json.contains("\"ticket-id\""));
    assert!(json.contains("\"ticket-type\""));
    assert!(json.contains("\"priority-id\""));
    assert!(json.contains("\"status-id\""));
    // Verify content
    assert!(json.contains("Fix the bug"));
    assert!(json.contains("Add new feature"));
    assert!(json.contains("bug"));
    assert!(json.contains("feature"));
    assert!(json.contains("johndoe"));
}

#[test]
fn test_merge_request_serializes_to_json() {
    let mr = MergeRequest {
        id: Some(10),
        source_ref: Some("feature/new-thing".to_string()),
        target_ref: Some("main".to_string()),
        subject: Some("Add new thing".to_string()),
        status: Some("open".to_string()),
        user_id: Some(42),
        created_at: Some("2024-01-15T10:00:00Z".to_string()),
        updated_at: Some("2024-01-16T12:00:00Z".to_string()),
        can_merge: Some(true),
    };

    let json = serde_json::to_string_pretty(&mr).expect("MergeRequest should serialize to JSON");

    assert!(!json.is_empty());
    assert!(json.contains("\"id\""));
    assert!(json.contains("10"));
    assert!(json.contains("\"source-ref\""));
    assert!(json.contains("feature/new-thing"));
    assert!(json.contains("\"target-ref\""));
    assert!(json.contains("main"));
    assert!(json.contains("\"subject\""));
    assert!(json.contains("Add new thing"));
    assert!(json.contains("\"status\""));
    assert!(json.contains("open"));
    assert!(json.contains("\"user-id\""));
    assert!(json.contains("\"created-at\""));
    assert!(json.contains("\"updated-at\""));
    assert!(json.contains("\"can-merge\""));
    assert!(json.contains("true"));
}

#[test]
fn test_milestone_serializes_to_json() {
    let milestone = Milestone {
        id: Some(5),
        name: Some("v1.0 Release".to_string()),
        description: Some("First stable release".to_string()),
        start_at: Some("2024-01-01".to_string()),
        deadline: Some("2024-03-31".to_string()),
        parent_id: None,
        estimated_time: Some(120.5),
        responsible_user_id: Some(99),
        status: Some("active".to_string()),
    };

    let json =
        serde_json::to_string_pretty(&milestone).expect("Milestone should serialize to JSON");

    assert!(!json.is_empty());
    assert!(json.contains("\"id\""));
    assert!(json.contains("5"));
    assert!(json.contains("\"name\""));
    assert!(json.contains("v1.0 Release"));
    assert!(json.contains("\"description\""));
    assert!(json.contains("First stable release"));
    assert!(json.contains("\"start-at\""));
    assert!(json.contains("2024-01-01"));
    assert!(json.contains("\"deadline\""));
    assert!(json.contains("2024-03-31"));
    assert!(json.contains("\"estimated-time\""));
    assert!(json.contains("120.5"));
    assert!(json.contains("\"responsible-user-id\""));
    assert!(json.contains("\"status\""));
    assert!(json.contains("active"));
}

#[test]
fn test_event_serializes_to_json() {
    let event = Event {
        title: Some("John committed to main".to_string()),
        event_type: Some("commit".to_string()),
        timestamp: Some("2024-01-15T09:30:00Z".to_string()),
        html_title: Some("<strong>John</strong> committed to main".to_string()),
        html_text: Some("<p>Fixed the login bug</p>".to_string()),
        content: Some("Fixed the login bug".to_string()),
        project_permalink: Some("my-project".to_string()),
        project_name: Some("My Project".to_string()),
        subject: Some("Fix login bug".to_string()),
        number: Some(123),
        name: Some("main".to_string()),
    };

    let json = serde_json::to_string_pretty(&event).expect("Event should serialize to JSON");

    assert!(!json.is_empty());
    assert!(json.contains("\"title\""));
    assert!(json.contains("John committed to main"));
    assert!(json.contains("\"type\""));
    assert!(json.contains("commit"));
    assert!(json.contains("\"timestamp\""));
    assert!(json.contains("2024-01-15T09:30:00Z"));
    assert!(json.contains("\"html-title\""));
    assert!(json.contains("\"html-text\""));
    assert!(json.contains("\"content\""));
    assert!(json.contains("\"project-permalink\""));
    assert!(json.contains("my-project"));
    assert!(json.contains("\"project-name\""));
    assert!(json.contains("My Project"));
    assert!(json.contains("\"subject\""));
    assert!(json.contains("\"number\""));
    assert!(json.contains("123"));
    assert!(json.contains("\"name\""));
}

// ── Color helper tests ──

#[test]
fn test_colorize_status_active() {
    let result = output::colorize_status("active");
    assert!(!result.is_empty());
    assert!(result.contains("active"));
}

#[test]
fn test_colorize_status_open() {
    let result = output::colorize_status("open");
    assert!(!result.is_empty());
    assert!(result.contains("open"));
}

#[test]
fn test_colorize_status_new() {
    let result = output::colorize_status("new");
    assert!(!result.is_empty());
    assert!(result.contains("new"));
}

#[test]
fn test_colorize_status_on_hold() {
    let result = output::colorize_status("on_hold");
    assert!(!result.is_empty());
    assert!(result.contains("on_hold"));
}

#[test]
fn test_colorize_status_in_progress() {
    let result = output::colorize_status("in_progress");
    assert!(!result.is_empty());
    assert!(result.contains("in_progress"));
}

#[test]
fn test_colorize_status_archived() {
    let result = output::colorize_status("archived");
    assert!(!result.is_empty());
    assert!(result.contains("archived"));
}

#[test]
fn test_colorize_status_closed() {
    let result = output::colorize_status("closed");
    assert!(!result.is_empty());
    assert!(result.contains("closed"));
}

#[test]
fn test_colorize_status_completed() {
    let result = output::colorize_status("completed");
    assert!(!result.is_empty());
    assert!(result.contains("completed"));
}

#[test]
fn test_colorize_status_resolved() {
    let result = output::colorize_status("resolved");
    assert!(!result.is_empty());
    assert!(result.contains("resolved"));
}

#[test]
fn test_colorize_status_cancelled() {
    let result = output::colorize_status("cancelled");
    assert!(!result.is_empty());
    assert!(result.contains("cancelled"));
}

#[test]
fn test_colorize_status_rejected() {
    let result = output::colorize_status("rejected");
    assert!(!result.is_empty());
    assert!(result.contains("rejected"));
}

#[test]
fn test_colorize_status_unknown() {
    let result = output::colorize_status("unknown_status");
    assert!(!result.is_empty());
    assert_eq!(result, "unknown_status");
}

#[test]
fn test_colorize_status_case_insensitive() {
    let lower = output::colorize_status("active");
    let upper = output::colorize_status("ACTIVE");
    // Both should return non-empty results and contain the status text
    assert!(!lower.is_empty());
    assert!(!upper.is_empty());
    assert!(upper.contains("ACTIVE"));
}

#[test]
fn test_colorize_priority_critical() {
    let result = output::colorize_priority("critical");
    assert!(!result.is_empty());
    assert!(result.contains("critical"));
}

#[test]
fn test_colorize_priority_high() {
    let result = output::colorize_priority("high");
    assert!(!result.is_empty());
    assert!(result.contains("high"));
}

#[test]
fn test_colorize_priority_normal() {
    let result = output::colorize_priority("normal");
    assert!(!result.is_empty());
    assert!(result.contains("normal"));
}

#[test]
fn test_colorize_priority_medium() {
    let result = output::colorize_priority("medium");
    assert!(!result.is_empty());
    assert!(result.contains("medium"));
}

#[test]
fn test_colorize_priority_low() {
    let result = output::colorize_priority("low");
    assert!(!result.is_empty());
    assert!(result.contains("low"));
}

#[test]
fn test_colorize_priority_unknown() {
    let result = output::colorize_priority("unknown_priority");
    assert!(!result.is_empty());
    assert_eq!(result, "unknown_priority");
}

#[test]
fn test_colorize_priority_case_insensitive() {
    let result = output::colorize_priority("CRITICAL");
    assert!(!result.is_empty());
    assert!(result.contains("CRITICAL"));
}

#[test]
fn test_colorize_ticket_type_bug() {
    let result = output::colorize_ticket_type("bug");
    assert!(!result.is_empty());
    assert!(result.contains("bug"));
}

#[test]
fn test_colorize_ticket_type_enhancement() {
    let result = output::colorize_ticket_type("enhancement");
    assert!(!result.is_empty());
    assert!(result.contains("enhancement"));
}

#[test]
fn test_colorize_ticket_type_feature() {
    let result = output::colorize_ticket_type("feature");
    assert!(!result.is_empty());
    assert!(result.contains("feature"));
}

#[test]
fn test_colorize_ticket_type_task() {
    let result = output::colorize_ticket_type("task");
    assert!(!result.is_empty());
    assert!(result.contains("task"));
}

#[test]
fn test_colorize_ticket_type_unknown() {
    let result = output::colorize_ticket_type("unknown_type");
    assert!(!result.is_empty());
    assert_eq!(result, "unknown_type");
}

#[test]
fn test_colorize_ticket_type_case_insensitive() {
    let result = output::colorize_ticket_type("BUG");
    assert!(!result.is_empty());
    assert!(result.contains("BUG"));
}

#[test]
fn test_colorize_mr_status_new() {
    let result = output::colorize_mr_status("new");
    assert!(!result.is_empty());
    assert!(result.contains("new"));
}

#[test]
fn test_colorize_mr_status_open() {
    let result = output::colorize_mr_status("open");
    assert!(!result.is_empty());
    assert!(result.contains("open"));
}

#[test]
fn test_colorize_mr_status_merged() {
    let result = output::colorize_mr_status("merged");
    assert!(!result.is_empty());
    assert!(result.contains("merged"));
}

#[test]
fn test_colorize_mr_status_closed() {
    let result = output::colorize_mr_status("closed");
    assert!(!result.is_empty());
    assert!(result.contains("closed"));
}

#[test]
fn test_colorize_mr_status_rejected() {
    let result = output::colorize_mr_status("rejected");
    assert!(!result.is_empty());
    assert!(result.contains("rejected"));
}

#[test]
fn test_colorize_mr_status_unknown() {
    let result = output::colorize_mr_status("unknown_status");
    assert!(!result.is_empty());
    assert_eq!(result, "unknown_status");
}

#[test]
fn test_colorize_mr_status_case_insensitive() {
    let result = output::colorize_mr_status("MERGED");
    assert!(!result.is_empty());
    assert!(result.contains("MERGED"));
}

#[test]
fn test_colorize_bool_true() {
    let result = output::colorize_bool(true, "Yes", "No");
    assert!(!result.is_empty());
    assert!(result.contains("Yes"));
}

#[test]
fn test_colorize_bool_false() {
    let result = output::colorize_bool(false, "Yes", "No");
    assert!(!result.is_empty());
    assert!(result.contains("No"));
}

#[test]
fn test_colorize_bool_custom_labels() {
    let true_result = output::colorize_bool(true, "Enabled", "Disabled");
    let false_result = output::colorize_bool(false, "Enabled", "Disabled");
    assert!(true_result.contains("Enabled"));
    assert!(false_result.contains("Disabled"));
}

#[test]
fn test_colorize_bool_open_closed_labels() {
    let open = output::colorize_bool(true, "open", "closed");
    let closed = output::colorize_bool(false, "open", "closed");
    assert!(open.contains("open"));
    assert!(closed.contains("closed"));
}

// ── print_output and print_list smoke tests ──

#[test]
fn test_print_output_json_mode_does_not_panic() {
    let project = Project {
        name: Some("Test".to_string()),
        account_name: None,
        permalink: None,
        project_id: None,
        group_id: None,
        overview: None,
        start_page: None,
        status: None,
        icon: None,
        disk_usage: None,
        total_tickets: None,
        open_tickets: None,
        closed_tickets: None,
    };

    let result = output::print_output(true, &project, || {});
    assert!(result.is_ok());
}

#[test]
fn test_print_output_human_mode_does_not_panic() {
    let project = Project {
        name: Some("Test".to_string()),
        account_name: None,
        permalink: None,
        project_id: None,
        group_id: None,
        overview: None,
        start_page: None,
        status: None,
        icon: None,
        disk_usage: None,
        total_tickets: None,
        open_tickets: None,
        closed_tickets: None,
    };

    let mut called = false;
    let result = output::print_output(false, &project, || {
        called = true;
    });
    assert!(result.is_ok());
    assert!(called);
}

#[test]
fn test_print_list_json_mode_does_not_panic() {
    let tickets: Vec<Ticket> = vec![Ticket {
        ticket_id: Some(1),
        summary: Some("Test ticket".to_string()),
        ticket_type: None,
        description: None,
        priority_id: None,
        status_id: None,
        category_id: None,
        milestone_id: None,
        assignee_id: None,
        reporter_id: None,
        assignee: None,
        reporter: None,
        tags: None,
    }];

    let result = output::print_list(true, &tickets, |_| {});
    assert!(result.is_ok());
}

#[test]
fn test_print_list_human_mode_does_not_panic() {
    let tickets: Vec<Ticket> = vec![];
    let mut called = false;
    let result = output::print_list(false, &tickets, |_| {
        called = true;
    });
    assert!(result.is_ok());
    assert!(called);
}
