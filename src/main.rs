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
#[command(version)]
#[command(about = "A command-line utility for the CodebaseHQ API")]
#[command(long_about = None)]
#[command(after_help = "\
GETTING STARTED:
    1. Obtain your API username and key from CodebaseHQ
    2. Run: cb login <account/username> <api-key>
    3. Credentials are stored in ~/.config/cb/config.toml

COMMANDS IN DETAIL:
    login          Store API credentials for future use
    project        List, show, create, update, delete projects; manage groups and user assignments
    repo           List, show, create, delete repositories; browse branches, commits, and files;
                   manage hooks and deployments; create and manage merge requests
    ticket         List, search, create tickets; add notes to update ticket fields; manage
                   watchers; view statuses, priorities, categories, and types
    milestone      List, create, update milestones with deadlines and responsible users
    activity       View account-wide or project-specific activity feeds with pagination
    version        Display the current version of cb

EXAMPLES:
    cb login mycompany/jdoe abc123def456
    cb project list
    cb repo branches my-project my-repo
    cb repo commits my-project my-repo main --path src/
    cb ticket create my-project \"Fix bug\" --ticket-type bug --priority-id 1
    cb ticket add-note my-project 42 --content \"Fixed\" --status-id 3
    cb repo create-mr my-project my-repo feature main \"Add feature\"
    cb activity account --page 2

RETRY BEHAVIOR:
    The client automatically retries with exponential backoff (1s, 2s, 4s, 8s, 16s)
    on HTTP 429 (rate limit), 503 (service unavailable), and 529 (overloaded) responses.

ENVIRONMENT:
    Credentials file:  ~/.config/cb/config.toml
    API base URL:      https://api3.codebasehq.com

AUTHOR:
    Sam Krishna <samkrishna@gmail.com>

SEE ALSO:
    CodebaseHQ API docs: https://support.codebasehq.com/kb
")]
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
