mod api;
mod commands;

use clap::Parser;

use api::client::CodebaseClient;
use commands::activity::ActivityCommands;
use commands::milestones::MilestoneCommands;
use commands::projects::ProjectCommands;
use commands::repositories::RepoCommands;
use commands::tickets::TicketCommands;

#[derive(Parser)]
#[command(name = "cb")]
#[command(about = "A command-line utility for the CodebaseHQ API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Manage projects
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    /// Manage repositories
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
    /// Manage tickets
    Ticket {
        #[command(subcommand)]
        command: TicketCommands,
    },
    /// Manage milestones
    Milestone {
        #[command(subcommand)]
        command: MilestoneCommands,
    },
    /// View activity feeds
    Activity {
        #[command(subcommand)]
        command: ActivityCommands,
    },
    /// Display version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Build client from environment variables (placeholder — auth not implemented)
    let account = std::env::var("CODEBASE_ACCOUNT").unwrap_or_default();
    let username = std::env::var("CODEBASE_USERNAME").unwrap_or_default();
    let api_key = std::env::var("CODEBASE_API_KEY").unwrap_or_default();
    let client = CodebaseClient::new(account, username, api_key);

    match cli.command {
        Commands::Project { command } => commands::projects::execute(&client, command).await?,
        Commands::Repo { command } => commands::repositories::execute(&client, command).await?,
        Commands::Ticket { command } => commands::tickets::execute(&client, command).await?,
        Commands::Milestone { command } => commands::milestones::execute(&client, command).await?,
        Commands::Activity { command } => commands::activity::execute(&client, command).await?,
        Commands::Version => {
            println!("cb {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
