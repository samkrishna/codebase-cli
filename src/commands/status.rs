use anyhow::Result;
use colored::Colorize;
use serde::Serialize;

use crate::api::client::CodebaseClient;
use crate::api::{activity, projects};
use crate::output;

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

pub async fn execute(client: &CodebaseClient, json: bool) -> Result<()> {
    let project_list = projects::list_projects(client).await?;
    let events = activity::account_activity(client, false, None, None).await?;

    let summaries: Vec<ProjectSummary> = project_list
        .iter()
        .map(|p| ProjectSummary {
            name: p.name.clone().unwrap_or_default(),
            permalink: p.permalink.clone().unwrap_or_default(),
            status: p.status.clone().unwrap_or_default(),
            open_tickets: p.open_tickets.unwrap_or(0),
            closed_tickets: p.closed_tickets.unwrap_or(0),
            total_tickets: p.total_tickets.unwrap_or(0),
        })
        .collect();

    let activity_items: Vec<ActivityItem> = events
        .iter()
        .take(10)
        .map(|e| ActivityItem {
            event_type: e.event_type.clone().unwrap_or_default(),
            timestamp: e.timestamp.clone().unwrap_or_default(),
            title: e.title.clone().unwrap_or_default(),
        })
        .collect();

    let dashboard = StatusDashboard {
        projects: summaries,
        recent_activity: activity_items,
    };

    if json {
        let out = serde_json::to_string_pretty(&dashboard)?;
        println!("{}", out);
    } else {
        println!("{}", "PROJECTS".bold());
        println!("{}", "─".repeat(60));
        for p in &dashboard.projects {
            let status = output::colorize_status(&p.status);
            println!(
                "  {} ({}) [{}]  {} open / {} closed",
                p.name.bold(),
                p.permalink.dimmed(),
                status,
                p.open_tickets.to_string().green(),
                p.closed_tickets.to_string().red(),
            );
        }
        if dashboard.projects.is_empty() {
            println!("  {}", "No projects found.".dimmed());
        }

        println!();
        println!("{}", "RECENT ACTIVITY".bold());
        println!("{}", "─".repeat(60));
        for a in &dashboard.recent_activity {
            println!(
                "  {} {} {}",
                a.event_type.cyan(),
                a.timestamp.dimmed(),
                a.title,
            );
        }
        if dashboard.recent_activity.is_empty() {
            println!("  {}", "No recent activity.".dimmed());
        }
    }

    Ok(())
}
