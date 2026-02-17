use anyhow::Result;
use clap::Subcommand;

use crate::api::client::CodebaseClient;
use crate::api::milestones;

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

pub async fn execute(client: &CodebaseClient, cmd: MilestoneCommands) -> Result<()> {
    match cmd {
        MilestoneCommands::List { project } => {
            let milestones = milestones::list_milestones(client, &project).await?;
            for m in milestones {
                println!(
                    "{}: {} [{}] ({} -> {})",
                    m.id.unwrap_or(0),
                    m.name.unwrap_or_default(),
                    m.status.unwrap_or_default(),
                    m.start_at.unwrap_or_default(),
                    m.deadline.unwrap_or_default()
                );
            }
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
            println!(
                "Created milestone {}: {}",
                m.id.unwrap_or(0),
                m.name.unwrap_or_default()
            );
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
            println!(
                "Updated milestone {}: {}",
                m.id.unwrap_or(0),
                m.name.unwrap_or_default()
            );
        }
    }
    Ok(())
}
