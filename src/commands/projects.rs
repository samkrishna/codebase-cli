use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::api::client::CodebaseClient;
use crate::api::projects;
use crate::output;

#[derive(Subcommand)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Show a specific project
    Show {
        /// Project permalink
        permalink: String,
    },
    /// Create a new project
    Create {
        /// Project name
        name: String,
    },
    /// Update a project
    Update {
        /// Project permalink (used as ID)
        permalink: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New status (active, on_hold, archived)
        #[arg(long)]
        status: Option<String>,
    },
    /// Delete a project
    Delete {
        /// Project permalink
        permalink: String,
    },
    /// List project groups
    Groups,
    /// List users assigned to a project
    Users {
        /// Project permalink
        project: String,
    },
    /// Assign users to a project (overwrites existing assignments)
    AssignUsers {
        /// Project permalink
        project: String,
        /// User IDs to assign
        #[arg(required = true)]
        user_ids: Vec<i64>,
    },
}

pub async fn execute(client: &CodebaseClient, cmd: ProjectCommands, json: bool) -> Result<()> {
    match cmd {
        ProjectCommands::List => {
            let project_list = projects::list_projects(client).await?;
            output::print_list(json, &project_list, |projects| {
                for p in projects {
                    let status = output::colorize_status(p.status.as_deref().unwrap_or("unknown"));
                    println!(
                        "{} ({}) [{}]",
                        p.name.as_deref().unwrap_or("").bold(),
                        p.permalink.as_deref().unwrap_or("").dimmed(),
                        status,
                    );
                }
            })?;
        }
        ProjectCommands::Show { permalink } => {
            let p = projects::show_project(client, &permalink).await?;
            output::print_output(json, &p, || {
                let status = output::colorize_status(p.status.as_deref().unwrap_or("unknown"));
                println!(
                    "{}: {}",
                    "Name".dimmed(),
                    p.name.as_deref().unwrap_or("").bold()
                );
                println!(
                    "{}: {}",
                    "Permalink".dimmed(),
                    p.permalink.as_deref().unwrap_or("")
                );
                println!("{}: {}", "Status".dimmed(), status);
                println!(
                    "{}: {}",
                    "Overview".dimmed(),
                    p.overview.as_deref().unwrap_or("")
                );
                println!(
                    "{}: {} open / {} closed / {} total",
                    "Tickets".dimmed(),
                    p.open_tickets.unwrap_or(0).to_string().green(),
                    p.closed_tickets.unwrap_or(0).to_string().red(),
                    p.total_tickets.unwrap_or(0),
                );
            })?;
        }
        ProjectCommands::Create { name } => {
            let p = projects::create_project(client, &name).await?;
            output::print_output(json, &p, || {
                println!(
                    "Created project: {} ({})",
                    p.name.as_deref().unwrap_or("").bold(),
                    p.permalink.as_deref().unwrap_or("")
                );
            })?;
        }
        ProjectCommands::Update {
            permalink,
            name,
            status,
        } => {
            let p =
                projects::update_project(client, &permalink, name.as_deref(), status.as_deref())
                    .await?;
            output::print_output(json, &p, || {
                println!(
                    "Updated project: {}",
                    p.name.as_deref().unwrap_or("").bold()
                );
            })?;
        }
        ProjectCommands::Delete { permalink } => {
            projects::delete_project(client, &permalink).await?;
            println!("Deleted project: {}", permalink);
        }
        ProjectCommands::Groups => {
            let groups = projects::list_project_groups(client).await?;
            output::print_list(json, &groups, |groups| {
                for g in groups {
                    println!(
                        "{}: {}",
                        g.id.unwrap_or(0),
                        g.label.as_deref().unwrap_or("").bold()
                    );
                }
            })?;
        }
        ProjectCommands::Users { project } => {
            let users = projects::list_project_users(client, &project).await?;
            output::print_list(json, &users, |users| {
                for u in users {
                    println!(
                        "{}: {} {} ({})",
                        u.id.unwrap_or(0),
                        u.first_name.as_deref().unwrap_or(""),
                        u.last_name.as_deref().unwrap_or(""),
                        u.username.as_deref().unwrap_or("").dimmed()
                    );
                }
            })?;
        }
        ProjectCommands::AssignUsers { project, user_ids } => {
            projects::assign_project_users(client, &project, &user_ids).await?;
            println!("Assigned {} users to {}", user_ids.len(), project);
        }
    }
    Ok(())
}
