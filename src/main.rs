mod api;
mod commands;

use clap::Parser;

use api::client::CodebaseClient;
use api::config::Config;
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
    /// Store API credentials
    Login {
        /// API username (format: account/username)
        api_username: String,
        /// API key
        api_key: String,
    },
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

fn load_client() -> anyhow::Result<CodebaseClient> {
    let config = Config::load()?;
    Ok(CodebaseClient::new(
        config.account().to_string(),
        config.api_username.clone(),
        config.api_key,
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login {
            api_username,
            api_key,
        } => {
            let config = Config {
                api_username: api_username.clone(),
                api_key,
            };
            config.save()?;
            println!(
                "Credentials saved for {} at {}",
                api_username,
                Config::config_path()?.display()
            );
        }
        Commands::Project { command } => {
            let client = load_client()?;
            commands::projects::execute(&client, command).await?;
        }
        Commands::Repo { command } => {
            let client = load_client()?;
            commands::repositories::execute(&client, command).await?;
        }
        Commands::Ticket { command } => {
            let client = load_client()?;
            commands::tickets::execute(&client, command).await?;
        }
        Commands::Milestone { command } => {
            let client = load_client()?;
            commands::milestones::execute(&client, command).await?;
        }
        Commands::Activity { command } => {
            let client = load_client()?;
            commands::activity::execute(&client, command).await?;
        }
        Commands::Version => {
            println!("cb {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
