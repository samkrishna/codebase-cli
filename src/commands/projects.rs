use anyhow::Result;
use clap::Subcommand;

use crate::api::client::CodebaseClient;
use crate::api::projects;

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

pub async fn execute(client: &CodebaseClient, cmd: ProjectCommands) -> Result<()> {
    match cmd {
        ProjectCommands::List => {
            let projects = projects::list_projects(client).await?;
            for p in projects {
                println!(
                    "{} ({}) [{}]",
                    p.name.unwrap_or_default(),
                    p.permalink.unwrap_or_default(),
                    p.status.unwrap_or_default()
                );
            }
        }
        ProjectCommands::Show { permalink } => {
            let p = projects::show_project(client, &permalink).await?;
            println!("Name:     {}", p.name.unwrap_or_default());
            println!("Permalink: {}", p.permalink.unwrap_or_default());
            println!("Status:   {}", p.status.unwrap_or_default());
            println!("Overview: {}", p.overview.unwrap_or_default());
            println!(
                "Tickets:  {} open / {} closed / {} total",
                p.open_tickets.unwrap_or(0),
                p.closed_tickets.unwrap_or(0),
                p.total_tickets.unwrap_or(0)
            );
        }
        ProjectCommands::Create { name } => {
            let p = projects::create_project(client, &name).await?;
            println!(
                "Created project: {} ({})",
                p.name.unwrap_or_default(),
                p.permalink.unwrap_or_default()
            );
        }
        ProjectCommands::Update {
            permalink,
            name,
            status,
        } => {
            let p =
                projects::update_project(client, &permalink, name.as_deref(), status.as_deref())
                    .await?;
            println!("Updated project: {}", p.name.unwrap_or_default());
        }
        ProjectCommands::Delete { permalink } => {
            projects::delete_project(client, &permalink).await?;
            println!("Deleted project: {}", permalink);
        }
        ProjectCommands::Groups => {
            let groups = projects::list_project_groups(client).await?;
            for g in groups {
                println!("{}: {}", g.id.unwrap_or(0), g.label.unwrap_or_default());
            }
        }
        ProjectCommands::Users { project } => {
            let users = projects::list_project_users(client, &project).await?;
            for u in users {
                println!(
                    "{}: {} {} ({})",
                    u.id.unwrap_or(0),
                    u.first_name.unwrap_or_default(),
                    u.last_name.unwrap_or_default(),
                    u.username.unwrap_or_default()
                );
            }
        }
        ProjectCommands::AssignUsers { project, user_ids } => {
            projects::assign_project_users(client, &project, &user_ids).await?;
            println!("Assigned {} users to {}", user_ids.len(), project);
        }
    }
    Ok(())
}
