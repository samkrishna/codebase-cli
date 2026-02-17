use serde::{Deserialize, Deserializer, Serialize};

/// Deserializes an optional i64 that may be an empty string in XML.
/// CodebaseHQ returns `<group-id></group-id>` for null integer fields.
fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
    }
}

/// Deserializes an optional f64 that may be an empty string in XML.
fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<f64>().map(Some).map_err(serde::de::Error::custom),
    }
}

/// Deserializes an optional bool that may be an empty string in XML.
fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => match s.as_str() {
            "true" | "1" => Ok(Some(true)),
            "false" | "0" => Ok(Some(false)),
            _ => Err(serde::de::Error::custom(format!(
                "invalid bool value: {}",
                s
            ))),
        },
    }
}

// ── Projects ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    #[serde(rename = "account-name")]
    pub account_name: Option<String>,
    pub permalink: Option<String>,
    #[serde(
        rename = "project-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub project_id: Option<i64>,
    #[serde(
        rename = "group-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub group_id: Option<i64>,
    pub overview: Option<String>,
    #[serde(rename = "start-page")]
    pub start_page: Option<String>,
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub icon: Option<i64>,
    #[serde(
        rename = "disk-usage",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub disk_usage: Option<i64>,
    #[serde(
        rename = "total-tickets",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub total_tickets: Option<i64>,
    #[serde(
        rename = "open-tickets",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub open_tickets: Option<i64>,
    #[serde(
        rename = "closed-tickets",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub closed_tickets: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projects {
    #[serde(rename = "project", default)]
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGroup {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGroups {
    #[serde(rename = "project-group", default)]
    pub groups: Vec<ProjectGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUser {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    #[serde(rename = "first-name")]
    pub first_name: Option<String>,
    #[serde(rename = "last-name")]
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "email-address")]
    pub email_address: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUsers {
    #[serde(rename = "user", default)]
    pub users: Vec<ProjectUser>,
}

// ── Repositories ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: Option<String>,
    pub permalink: Option<String>,
    #[serde(
        rename = "disk-usage",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub disk_usage: Option<i64>,
    #[serde(rename = "last-commit-ref")]
    pub last_commit_ref: Option<String>,
    #[serde(rename = "clone-url")]
    pub clone_url: Option<String>,
    pub source: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub sync: Option<bool>,
    #[serde(rename = "last-sync-at")]
    pub last_sync_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repositories {
    #[serde(rename = "repository", default)]
    pub repositories: Vec<Repository>,
}

// ── Commits ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    #[serde(rename = "ref")]
    pub commit_ref: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "author-name")]
    pub author_name: Option<String>,
    #[serde(rename = "author-email")]
    pub author_email: Option<String>,
    #[serde(rename = "authored-at")]
    pub authored_at: Option<String>,
    #[serde(rename = "committer-name")]
    pub committer_name: Option<String>,
    #[serde(rename = "committer-email")]
    pub committer_email: Option<String>,
    #[serde(rename = "committed-at")]
    pub committed_at: Option<String>,
    #[serde(rename = "parent-refs")]
    pub parent_refs: Option<String>,
    #[serde(rename = "tree-ref")]
    pub tree_ref: Option<String>,
    #[serde(rename = "author-user")]
    pub author_user: Option<String>,
    #[serde(rename = "committer-user")]
    pub committer_user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commits {
    #[serde(rename = "commit", default)]
    pub commits: Vec<Commit>,
}

// ── Deployments ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deployment {
    pub branch: Option<String>,
    pub revision: Option<String>,
    pub environment: Option<String>,
    pub servers: Option<String>,
}

// ── Hooks ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(rename = "repository-hook", default)]
    pub hooks: Vec<Hook>,
}

// ── Branches ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branches {
    #[serde(rename = "branch", default)]
    pub branches: Vec<Branch>,
}

// ── Merge Requests ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    #[serde(rename = "source-ref")]
    pub source_ref: Option<String>,
    #[serde(rename = "target-ref")]
    pub target_ref: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    #[serde(
        rename = "user-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub user_id: Option<i64>,
    #[serde(rename = "created-at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated-at")]
    pub updated_at: Option<String>,
    #[serde(
        rename = "can-merge",
        default,
        deserialize_with = "deserialize_optional_bool"
    )]
    pub can_merge: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequests {
    #[serde(rename = "merge-request", default)]
    pub merge_requests: Vec<MergeRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequestComment {
    pub content: Option<String>,
    #[serde(
        rename = "user-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub user_id: Option<i64>,
    pub action: Option<String>,
    #[serde(rename = "created-at")]
    pub created_at: Option<String>,
}

// ── Tickets ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(
        rename = "ticket-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub ticket_id: Option<i64>,
    pub summary: Option<String>,
    #[serde(rename = "ticket-type")]
    pub ticket_type: Option<String>,
    pub description: Option<String>,
    #[serde(
        rename = "priority-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub priority_id: Option<i64>,
    #[serde(
        rename = "status-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub status_id: Option<i64>,
    #[serde(
        rename = "category-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub category_id: Option<i64>,
    #[serde(
        rename = "milestone-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub milestone_id: Option<i64>,
    #[serde(
        rename = "assignee-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub assignee_id: Option<i64>,
    #[serde(
        rename = "reporter-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub reporter_id: Option<i64>,
    pub assignee: Option<String>,
    pub reporter: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tickets {
    #[serde(rename = "ticket", default)]
    pub tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketNote {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub content: Option<String>,
    #[serde(rename = "time-added")]
    pub time_added: Option<String>,
    pub changes: Option<NoteChanges>,
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub private: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteChanges {
    #[serde(
        rename = "status-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub status_id: Option<i64>,
    #[serde(
        rename = "priority-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub priority_id: Option<i64>,
    #[serde(
        rename = "category-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub category_id: Option<i64>,
    #[serde(
        rename = "assignee-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub assignee_id: Option<i64>,
    #[serde(
        rename = "milestone-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub milestone_id: Option<i64>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketNotes {
    #[serde(rename = "ticket-note", default)]
    pub notes: Vec<TicketNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatus {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(rename = "background-colour")]
    pub background_colour: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub order: Option<i64>,
    #[serde(
        rename = "treat-as-closed",
        default,
        deserialize_with = "deserialize_optional_bool"
    )]
    pub treat_as_closed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatuses {
    #[serde(rename = "ticketing-status", default)]
    pub statuses: Vec<TicketStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPriority {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub colour: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub default: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPriorities {
    #[serde(rename = "ticketing-priority", default)]
    pub priorities: Vec<TicketPriority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategory {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategories {
    #[serde(rename = "ticketing-category", default)]
    pub categories: Vec<TicketCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketType {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketTypes {
    #[serde(rename = "ticketing-type", default)]
    pub types: Vec<TicketType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watcher {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub watcher: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watchers {
    #[serde(rename = "watcher", default)]
    pub watchers: Vec<Watcher>,
}

// ── Milestones ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "start-at")]
    pub start_at: Option<String>,
    pub deadline: Option<String>,
    #[serde(
        rename = "parent-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub parent_id: Option<i64>,
    #[serde(
        rename = "estimated-time",
        default,
        deserialize_with = "deserialize_optional_f64"
    )]
    pub estimated_time: Option<f64>,
    #[serde(
        rename = "responsible-user-id",
        default,
        deserialize_with = "deserialize_optional_i64"
    )]
    pub responsible_user_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestones {
    #[serde(rename = "ticketing-milestone", default)]
    pub milestones: Vec<Milestone>,
}

// ── Activity Feed ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub event_type: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "html-title")]
    pub html_title: Option<String>,
    #[serde(rename = "html-text")]
    pub html_text: Option<String>,
    // Common extra properties across event types
    pub content: Option<String>,
    #[serde(rename = "project-permalink")]
    pub project_permalink: Option<String>,
    #[serde(rename = "project-name")]
    pub project_name: Option<String>,
    pub subject: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub number: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
    #[serde(rename = "event", default)]
    pub events: Vec<Event>,
}
