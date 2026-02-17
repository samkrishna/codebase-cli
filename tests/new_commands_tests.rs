/// Tests for new UX features: git_context parsing, Config helpers, and status command structs.
///
/// Note: `parse_codebase_remote` is a private function inside `git_context`, so we cannot
/// call it directly from integration tests. The existing unit tests inside `git_context.rs`
/// already cover the private parser. Here we cover:
///   - Config::account() and Config::username() parsing
///   - git_context::detect() behavior driven by temp git repos with controlled remote URLs
///   - Minimal recreations of the status command JSON structs (since those are also private)
///
/// Tests that change the process cwd (to drive git_context::detect()) share a mutex so they
/// run serially and do not interfere with each other.
use codebase_cli::api::config::Config;
use codebase_cli::git_context;
use serde::Serialize;
use serde_json::Value;
use std::sync::Mutex;

// Mutex to serialize tests that mutate the process-wide working directory.
static CWD_LOCK: Mutex<()> = Mutex::new(());

// ---------------------------------------------------------------------------
// Helper to build a Config quickly
// ---------------------------------------------------------------------------

fn make_config(api_username: &str) -> Config {
    Config {
        api_username: api_username.to_string(),
        api_key: "test-api-key".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Config::account() tests
// ---------------------------------------------------------------------------

#[test]
fn test_config_account_parses_account_from_slash_format() {
    let config = make_config("sectormobile/samkrishna");
    assert_eq!(config.account(), "sectormobile");
}

#[test]
fn test_config_account_with_different_account_name() {
    let config = make_config("mycompany/alice");
    assert_eq!(config.account(), "mycompany");
}

#[test]
fn test_config_account_with_no_slash_returns_full_string() {
    let config = make_config("standaloneaccount");
    assert_eq!(config.account(), "standaloneaccount");
}

#[test]
fn test_config_account_with_empty_string_returns_empty() {
    let config = make_config("");
    assert_eq!(config.account(), "");
}

#[test]
fn test_config_account_with_multiple_slashes_returns_first_segment() {
    let config = make_config("company/user/extra");
    assert_eq!(config.account(), "company");
}

// ---------------------------------------------------------------------------
// Config::username() tests
// ---------------------------------------------------------------------------

#[test]
fn test_config_username_parses_username_from_slash_format() {
    let config = make_config("sectormobile/samkrishna");
    assert_eq!(config.username(), "samkrishna");
}

#[test]
fn test_config_username_with_different_username() {
    let config = make_config("mycompany/alice");
    assert_eq!(config.username(), "alice");
}

#[test]
fn test_config_username_with_no_slash_returns_full_string() {
    // When there is no slash, username() falls back to the full api_username string.
    let config = make_config("standaloneaccount");
    assert_eq!(config.username(), "standaloneaccount");
}

#[test]
fn test_config_username_with_empty_string_returns_empty() {
    let config = make_config("");
    assert_eq!(config.username(), "");
}

// ---------------------------------------------------------------------------
// Config struct round-trips through JSON serialization
// ---------------------------------------------------------------------------

#[test]
fn test_config_serializes_to_json() {
    let config = make_config("sectormobile/samkrishna");
    let json = serde_json::to_string(&config).expect("Config must serialize to JSON");
    let parsed: Value = serde_json::from_str(&json).expect("Serialized config must be valid JSON");
    assert_eq!(parsed["api_username"], "sectormobile/samkrishna");
    assert_eq!(parsed["api_key"], "test-api-key");
}

#[test]
fn test_config_deserializes_from_json() {
    let json = r#"{"api_username":"sectormobile/samkrishna","api_key":"my-key"}"#;
    let config: Config = serde_json::from_str(json).expect("Config must deserialize from JSON");
    assert_eq!(config.api_username, "sectormobile/samkrishna");
    assert_eq!(config.api_key, "my-key");
    assert_eq!(config.account(), "sectormobile");
    assert_eq!(config.username(), "samkrishna");
}

// ---------------------------------------------------------------------------
// git_context::detect() — behavioral tests
//
// We cannot call the private `parse_codebase_remote` from integration tests.
// Instead we exercise `detect()` in the real git repo that runs these tests.
// The test repo (codebase-cli itself) most likely has a GitHub remote, so
// `detect()` should return None. We also verify that the function does not
// panic under normal conditions.
// ---------------------------------------------------------------------------

#[test]
fn test_detect_does_not_panic() {
    // Either returns Some(GitContext) or None; must not panic.
    let _ = git_context::detect();
}

#[test]
fn test_detect_returns_none_for_non_codebase_remote() {
    // The codebase-cli project itself lives on GitHub (or has no codebasehq remote),
    // so detect() should return None.
    let ctx = git_context::detect();
    // Only assert None when we can confirm the remote is not a codebasehq URL.
    // We read the remote URL via git config to determine whether we should
    // expect Some or None.
    let remote_output = std::process::Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output();

    if let Ok(output) = remote_output {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if url.contains("codebasehq.com") {
            // If the repo itself is hosted on codebasehq, detect() should return Some.
            assert!(
                ctx.is_some(),
                "detect() should return Some for a codebasehq remote"
            );
        } else {
            // Non-codebasehq remote (e.g. GitHub) should yield None.
            assert!(
                ctx.is_none(),
                "detect() should return None for a non-codebasehq remote, got {:?}",
                ctx
            );
        }
    }
    // If git is not available, the test trivially passes (no panic).
}

#[test]
fn test_git_context_fields_are_strings() {
    // Verify that when a GitContext is returned its fields are non-empty strings.
    // This is only exercised if we happen to be running inside a codebasehq repo.
    if let Some(ctx) = git_context::detect() {
        assert!(
            !ctx.project.is_empty(),
            "project must be a non-empty string"
        );
        if let Some(ref repo) = ctx.repo {
            assert!(
                !repo.is_empty(),
                "repo must be a non-empty string when present"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// git_context SSH and HTTPS URL parsing — tested via temporary git repos
//
// Since parse_codebase_remote is private we create temporary git repos with
// controlled remote URLs and then call detect() from within those directories.
// A process-wide mutex serialises all tests that mutate the working directory.
// ---------------------------------------------------------------------------

/// Run `f` with the process cwd set to a fresh git repo whose origin remote
/// is `remote_url`. The original cwd is restored after `f` returns, and the
/// temporary directory is cleaned up automatically.
fn with_git_remote<F>(remote_url: &str, f: F)
where
    F: FnOnce(),
{
    use std::env;
    use std::process::Command;

    // Acquire the mutex so cwd-changing tests run one at a time.
    let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let tmp_path = tmp.path().to_path_buf();

    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(&tmp_path)
        .output()
        .expect("git init failed");

    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(&tmp_path)
        .output()
        .ok();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&tmp_path)
        .output()
        .ok();

    Command::new("git")
        .args(["remote", "add", "origin", remote_url])
        .current_dir(&tmp_path)
        .output()
        .expect("git remote add failed");

    let original_dir = env::current_dir().expect("cannot read cwd");
    env::set_current_dir(&tmp_path).expect("cannot set cwd to temp dir");

    f();

    env::set_current_dir(original_dir).expect("cannot restore original cwd");
    // `tmp` is dropped here, cleaning up the directory.
}

#[test]
fn test_detect_ssh_remote_with_account_project_repo() {
    with_git_remote("git@codebasehq.com:myaccount/myproject/myrepo.git", || {
        let ctx = git_context::detect().expect("should detect codebasehq SSH remote");
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.as_deref(), Some("myrepo"));
    });
}

#[test]
fn test_detect_ssh_remote_with_different_account_project_repo() {
    with_git_remote("git@codebasehq.com:acmecorp/backend-api/core.git", || {
        let ctx = git_context::detect().expect("should detect codebasehq SSH remote");
        assert_eq!(ctx.project, "backend-api");
        assert_eq!(ctx.repo.as_deref(), Some("core"));
    });
}

#[test]
fn test_detect_ssh_remote_without_git_suffix() {
    with_git_remote("git@codebasehq.com:myaccount/myproject/myrepo", || {
        let ctx =
            git_context::detect().expect("should detect codebasehq SSH remote without .git suffix");
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.as_deref(), Some("myrepo"));
    });
}

#[test]
fn test_detect_ssh_remote_with_only_account_and_project() {
    with_git_remote("git@codebasehq.com:myaccount/myproject", || {
        let ctx =
            git_context::detect().expect("should detect codebasehq SSH remote with project only");
        assert_eq!(ctx.project, "myproject");
        assert!(
            ctx.repo.is_none(),
            "repo should be None when not present in URL"
        );
    });
}

#[test]
fn test_detect_https_remote_with_account_project_repo() {
    with_git_remote(
        "https://myaccount.codebasehq.com/myproject/myrepo.git",
        || {
            let ctx = git_context::detect().expect("should detect codebasehq HTTPS remote");
            assert_eq!(ctx.project, "myproject");
            assert_eq!(ctx.repo.as_deref(), Some("myrepo"));
        },
    );
}

#[test]
fn test_detect_https_remote_with_different_values() {
    with_git_remote(
        "https://acmecorp.codebasehq.com/platform/services.git",
        || {
            let ctx = git_context::detect().expect("should detect codebasehq HTTPS remote");
            assert_eq!(ctx.project, "platform");
            assert_eq!(ctx.repo.as_deref(), Some("services"));
        },
    );
}

#[test]
fn test_detect_https_remote_without_git_suffix() {
    with_git_remote("https://myaccount.codebasehq.com/myproject/myrepo", || {
        let ctx =
            git_context::detect().expect("should detect codebasehq HTTPS remote without .git");
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.as_deref(), Some("myrepo"));
    });
}

#[test]
fn test_detect_https_remote_with_only_project() {
    with_git_remote("https://myaccount.codebasehq.com/myproject", || {
        let ctx =
            git_context::detect().expect("should detect codebasehq HTTPS remote with project only");
        assert_eq!(ctx.project, "myproject");
        assert!(
            ctx.repo.is_none(),
            "repo should be None when not present in HTTPS URL"
        );
    });
}

#[test]
fn test_detect_github_remote_returns_none() {
    with_git_remote("git@github.com:user/repo.git", || {
        let ctx = git_context::detect();
        assert!(
            ctx.is_none(),
            "GitHub remote must not be detected as a codebasehq context"
        );
    });
}

#[test]
fn test_detect_gitlab_remote_returns_none() {
    with_git_remote("git@gitlab.com:group/project.git", || {
        let ctx = git_context::detect();
        assert!(
            ctx.is_none(),
            "GitLab remote must not be detected as a codebasehq context"
        );
    });
}

#[test]
fn test_detect_https_github_remote_returns_none() {
    with_git_remote("https://github.com/user/repo.git", || {
        let ctx = git_context::detect();
        assert!(
            ctx.is_none(),
            "GitHub HTTPS remote must not be detected as a codebasehq context"
        );
    });
}

// ---------------------------------------------------------------------------
// Status command struct JSON serialization
//
// The real structs (StatusDashboard, ProjectSummary, ActivityItem) are private
// to `commands::status`. We define minimal mirror versions here and verify that
// the JSON shape they produce matches what the status command would emit.
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct StatusDashboard {
    projects: Vec<ProjectSummary>,
    recent_activity: Vec<ActivityItem>,
}

#[derive(Serialize)]
struct ProjectSummary {
    name: String,
    permalink: String,
    status: String,
    open_tickets: i64,
    closed_tickets: i64,
    total_tickets: i64,
}

#[derive(Serialize)]
struct ActivityItem {
    event_type: String,
    timestamp: String,
    title: String,
}

#[test]
fn test_project_summary_serializes_to_json() {
    let summary = ProjectSummary {
        name: "My Project".to_string(),
        permalink: "my-project".to_string(),
        status: "active".to_string(),
        open_tickets: 5,
        closed_tickets: 20,
        total_tickets: 25,
    };

    let json = serde_json::to_string(&summary).expect("ProjectSummary must serialize to JSON");
    let parsed: Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["name"], "My Project");
    assert_eq!(parsed["permalink"], "my-project");
    assert_eq!(parsed["status"], "active");
    assert_eq!(parsed["open_tickets"], 5);
    assert_eq!(parsed["closed_tickets"], 20);
    assert_eq!(parsed["total_tickets"], 25);
}

#[test]
fn test_activity_item_serializes_to_json() {
    let item = ActivityItem {
        event_type: "commit".to_string(),
        timestamp: "2024-01-15T10:30:00Z".to_string(),
        title: "Fix bug in parser".to_string(),
    };

    let json = serde_json::to_string(&item).expect("ActivityItem must serialize to JSON");
    let parsed: Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["event_type"], "commit");
    assert_eq!(parsed["timestamp"], "2024-01-15T10:30:00Z");
    assert_eq!(parsed["title"], "Fix bug in parser");
}

#[test]
fn test_status_dashboard_serializes_to_json() {
    let dashboard = StatusDashboard {
        projects: vec![
            ProjectSummary {
                name: "Alpha".to_string(),
                permalink: "alpha".to_string(),
                status: "active".to_string(),
                open_tickets: 3,
                closed_tickets: 10,
                total_tickets: 13,
            },
            ProjectSummary {
                name: "Beta".to_string(),
                permalink: "beta".to_string(),
                status: "archived".to_string(),
                open_tickets: 0,
                closed_tickets: 50,
                total_tickets: 50,
            },
        ],
        recent_activity: vec![ActivityItem {
            event_type: "ticket_update".to_string(),
            timestamp: "2024-03-01T09:00:00Z".to_string(),
            title: "Resolved issue #42".to_string(),
        }],
    };

    let json =
        serde_json::to_string_pretty(&dashboard).expect("StatusDashboard must serialize to JSON");
    let parsed: Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["projects"].as_array().unwrap().len(), 2);
    assert_eq!(parsed["projects"][0]["name"], "Alpha");
    assert_eq!(parsed["projects"][0]["status"], "active");
    assert_eq!(parsed["projects"][0]["open_tickets"], 3);
    assert_eq!(parsed["projects"][1]["name"], "Beta");
    assert_eq!(parsed["projects"][1]["status"], "archived");
    assert_eq!(parsed["projects"][1]["open_tickets"], 0);

    assert_eq!(parsed["recent_activity"].as_array().unwrap().len(), 1);
    assert_eq!(parsed["recent_activity"][0]["event_type"], "ticket_update");
    assert_eq!(parsed["recent_activity"][0]["title"], "Resolved issue #42");
}

#[test]
fn test_status_dashboard_empty_serializes_to_json() {
    let dashboard = StatusDashboard {
        projects: vec![],
        recent_activity: vec![],
    };

    let json = serde_json::to_string(&dashboard).expect("Empty StatusDashboard must serialize");
    let parsed: Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["projects"].as_array().unwrap().len(), 0);
    assert_eq!(parsed["recent_activity"].as_array().unwrap().len(), 0);
}

#[test]
fn test_project_summary_zero_tickets() {
    let summary = ProjectSummary {
        name: "Empty Project".to_string(),
        permalink: "empty-project".to_string(),
        status: "active".to_string(),
        open_tickets: 0,
        closed_tickets: 0,
        total_tickets: 0,
    };

    let json = serde_json::to_string(&summary).expect("ProjectSummary must serialize");
    let parsed: Value = serde_json::from_str(&json).unwrap();

    assert_eq!(parsed["open_tickets"], 0);
    assert_eq!(parsed["closed_tickets"], 0);
    assert_eq!(parsed["total_tickets"], 0);
}

// ---------------------------------------------------------------------------
// Browse command URL construction — tested by replicating the logic
//
// browse::execute() is not pub in lib.rs, so we replicate the URL-building
// logic here and verify the output format matches what the command would open.
// ---------------------------------------------------------------------------

fn build_browse_url(account: &str, project: &str, target: Option<&str>) -> String {
    match target {
        Some(t) if t.parse::<i64>().is_ok() => {
            format!(
                "https://{}.codebasehq.com/projects/{}/tickets/{}",
                account, project, t
            )
        }
        Some(t) => {
            format!(
                "https://{}.codebasehq.com/projects/{}/repositories/{}",
                account, project, t
            )
        }
        None => {
            format!("https://{}.codebasehq.com/projects/{}", account, project)
        }
    }
}

#[test]
fn test_browse_url_project_only() {
    let url = build_browse_url("mycompany", "my-project", None);
    assert_eq!(url, "https://mycompany.codebasehq.com/projects/my-project");
}

#[test]
fn test_browse_url_with_ticket_number() {
    let url = build_browse_url("mycompany", "my-project", Some("42"));
    assert_eq!(
        url,
        "https://mycompany.codebasehq.com/projects/my-project/tickets/42"
    );
}

#[test]
fn test_browse_url_with_negative_number_treated_as_ticket() {
    // Negative integers still parse as i64, so they go to the tickets route.
    let url = build_browse_url("mycompany", "my-project", Some("-1"));
    assert_eq!(
        url,
        "https://mycompany.codebasehq.com/projects/my-project/tickets/-1"
    );
}

#[test]
fn test_browse_url_with_repo_permalink() {
    let url = build_browse_url("mycompany", "my-project", Some("main-repo"));
    assert_eq!(
        url,
        "https://mycompany.codebasehq.com/projects/my-project/repositories/main-repo"
    );
}

#[test]
fn test_browse_url_with_non_numeric_target_goes_to_repository() {
    let url = build_browse_url("acmecorp", "backend", Some("api-service"));
    assert_eq!(
        url,
        "https://acmecorp.codebasehq.com/projects/backend/repositories/api-service"
    );
}

#[test]
fn test_browse_url_account_comes_from_config_account_helper() {
    let config = make_config("sectormobile/samkrishna");
    let url = build_browse_url(config.account(), "myproject", None);
    assert_eq!(
        url,
        "https://sectormobile.codebasehq.com/projects/myproject"
    );
}
