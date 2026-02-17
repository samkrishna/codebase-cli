use anyhow::Result;
use clap::Subcommand;

use crate::api::activity;
use crate::api::client::CodebaseClient;

#[derive(Subcommand)]
pub enum ActivityCommands {
    /// Show account-wide activity feed
    Account {
        /// Return raw data
        #[arg(long)]
        raw: bool,
        /// Filter events since timestamp (YYYY-MM-DD HH:MM:SS +TZ)
        #[arg(long)]
        since: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
    /// Show project-specific activity feed
    Project {
        /// Project permalink
        project: String,
        /// Return raw data
        #[arg(long)]
        raw: bool,
        /// Filter events since timestamp
        #[arg(long)]
        since: Option<String>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
    },
}

pub async fn execute(client: &CodebaseClient, cmd: ActivityCommands) -> Result<()> {
    match cmd {
        ActivityCommands::Account { raw, since, page } => {
            let events = activity::account_activity(client, raw, since.as_deref(), page).await?;
            print_events(&events);
        }
        ActivityCommands::Project {
            project,
            raw,
            since,
            page,
        } => {
            let events =
                activity::project_activity(client, &project, raw, since.as_deref(), page).await?;
            print_events(&events);
        }
    }
    Ok(())
}

fn print_events(events: &[crate::api::models::Event]) {
    for e in events {
        println!(
            "[{}] {} — {}",
            e.event_type.as_deref().unwrap_or("unknown"),
            e.timestamp.as_deref().unwrap_or(""),
            e.title.as_deref().unwrap_or("")
        );
    }
}
