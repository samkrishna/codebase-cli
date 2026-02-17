use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::api::client::CodebaseClient;
use crate::api::milestones;
use crate::output;

#[derive(Subcommand)]
pub enum MilestoneCommands {
    /// List milestones for a project
    List {
        /// Project permalink
        project: String,
    },
    /// Create a new milestone
    Create {
        /// Project permalink
        project: String,
        /// Milestone name
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Start date (yyyy-mm-dd)
        #[arg(long)]
        start_at: Option<String>,
        /// Deadline (yyyy-mm-dd)
        #[arg(long)]
        deadline: Option<String>,
        /// Responsible user ID
        #[arg(long)]
        responsible_user_id: Option<i64>,
        /// Parent milestone ID
        #[arg(long)]
        parent_id: Option<i64>,
        /// Status (active, completed, cancelled)
        #[arg(long)]
        status: Option<String>,
    },
    /// Update a milestone
    Update {
        /// Project permalink
        project: String,
        /// Milestone ID
        milestone_id: i64,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New start date
        #[arg(long)]
        start_at: Option<String>,
        /// New deadline
        #[arg(long)]
        deadline: Option<String>,
        /// New responsible user ID
        #[arg(long)]
        responsible_user_id: Option<i64>,
        /// New parent milestone ID
        #[arg(long)]
        parent_id: Option<i64>,
        /// New status
        #[arg(long)]
        status: Option<String>,
    },
}

pub async fn execute(client: &CodebaseClient, cmd: MilestoneCommands, json: bool) -> Result<()> {
    match cmd {
        MilestoneCommands::List { project } => {
            let milestone_list = milestones::list_milestones(client, &project).await?;
            output::print_list(json, &milestone_list, |milestones| {
                for m in milestones {
                    let status = output::colorize_status(m.status.as_deref().unwrap_or("unknown"));
                    println!(
                        "{}: {} [{}] ({} -> {})",
                        m.id.unwrap_or(0),
                        m.name.as_deref().unwrap_or("").bold(),
                        status,
                        m.start_at.as_deref().unwrap_or("").dimmed(),
                        m.deadline.as_deref().unwrap_or("").dimmed()
                    );
                }
            })?;
        }
        MilestoneCommands::Create {
            project,
            name,
            description,
            start_at,
            deadline,
            responsible_user_id,
            parent_id,
            status,
        } => {
            let m = milestones::create_milestone(
                client,
                &project,
                &name,
                description.as_deref(),
                start_at.as_deref(),
                deadline.as_deref(),
                responsible_user_id,
                parent_id,
                status.as_deref(),
            )
            .await?;
            output::print_output(json, &m, || {
                println!(
                    "Created milestone {}: {}",
                    m.id.unwrap_or(0),
                    m.name.as_deref().unwrap_or("").bold()
                );
            })?;
        }
        MilestoneCommands::Update {
            project,
            milestone_id,
            name,
            description,
            start_at,
            deadline,
            responsible_user_id,
            parent_id,
            status,
        } => {
            let m = milestones::update_milestone(
                client,
                &project,
                milestone_id,
                name.as_deref(),
                description.as_deref(),
                start_at.as_deref(),
                deadline.as_deref(),
                responsible_user_id,
                parent_id,
                status.as_deref(),
            )
            .await?;
            output::print_output(json, &m, || {
                println!(
                    "Updated milestone {}: {}",
                    m.id.unwrap_or(0),
                    m.name.as_deref().unwrap_or("").bold()
                );
            })?;
        }
    }
    Ok(())
}
