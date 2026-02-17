use serde::{Deserialize, Serialize};

// ── Projects ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: Option<String>,
    #[serde(rename = "account-name")]
    pub account_name: Option<String>,
    pub permalink: Option<String>,
    #[serde(rename = "group-id")]
    pub group_id: Option<i64>,
    pub overview: Option<String>,
    #[serde(rename = "start-page")]
    pub start_page: Option<String>,
    pub status: Option<String>,
    pub icon: Option<i64>,
    #[serde(rename = "total-tickets")]
    pub total_tickets: Option<i64>,
    #[serde(rename = "open-tickets")]
    pub open_tickets: Option<i64>,
    #[serde(rename = "closed-tickets")]
    pub closed_tickets: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projects {
    #[serde(rename = "project", default)]
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGroup {
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
    #[serde(rename = "disk-usage")]
    pub disk_usage: Option<i64>,
    #[serde(rename = "last-commit-ref")]
    pub last_commit_ref: Option<String>,
    #[serde(rename = "clone-url")]
    pub clone_url: Option<String>,
    pub source: Option<String>,
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
    pub id: Option<i64>,
    #[serde(rename = "source-ref")]
    pub source_ref: Option<String>,
    #[serde(rename = "target-ref")]
    pub target_ref: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "user-id")]
    pub user_id: Option<i64>,
    #[serde(rename = "created-at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated-at")]
    pub updated_at: Option<String>,
    #[serde(rename = "can-merge")]
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
    #[serde(rename = "user-id")]
    pub user_id: Option<i64>,
    pub action: Option<String>,
    #[serde(rename = "created-at")]
    pub created_at: Option<String>,
}

// ── Tickets ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "ticket-id")]
    pub ticket_id: Option<i64>,
    pub summary: Option<String>,
    #[serde(rename = "ticket-type")]
    pub ticket_type: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "priority-id")]
    pub priority_id: Option<i64>,
    #[serde(rename = "status-id")]
    pub status_id: Option<i64>,
    #[serde(rename = "category-id")]
    pub category_id: Option<i64>,
    #[serde(rename = "milestone-id")]
    pub milestone_id: Option<i64>,
    #[serde(rename = "assignee-id")]
    pub assignee_id: Option<i64>,
    #[serde(rename = "reporter-id")]
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
    pub id: Option<i64>,
    pub content: Option<String>,
    #[serde(rename = "time-added")]
    pub time_added: Option<String>,
    pub changes: Option<NoteChanges>,
    pub private: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteChanges {
    #[serde(rename = "status-id")]
    pub status_id: Option<i64>,
    #[serde(rename = "priority-id")]
    pub priority_id: Option<i64>,
    #[serde(rename = "category-id")]
    pub category_id: Option<i64>,
    #[serde(rename = "assignee-id")]
    pub assignee_id: Option<i64>,
    #[serde(rename = "milestone-id")]
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
    pub id: Option<i64>,
    pub name: Option<String>,
    #[serde(rename = "background-colour")]
    pub background_colour: Option<String>,
    pub order: Option<i64>,
    #[serde(rename = "treat-as-closed")]
    pub treat_as_closed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketStatuses {
    #[serde(rename = "ticketing-status", default)]
    pub statuses: Vec<TicketStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPriority {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub colour: Option<String>,
    pub default: Option<bool>,
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketPriorities {
    #[serde(rename = "ticketing-priority", default)]
    pub priorities: Vec<TicketPriority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategory {
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
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "start-at")]
    pub start_at: Option<String>,
    pub deadline: Option<String>,
    #[serde(rename = "parent-id")]
    pub parent_id: Option<i64>,
    #[serde(rename = "estimated-time")]
    pub estimated_time: Option<f64>,
    #[serde(rename = "responsible-user-id")]
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
    pub number: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Events {
    #[serde(rename = "event", default)]
    pub events: Vec<Event>,
}
