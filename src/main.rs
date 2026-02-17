mod api;
mod commands;
mod git_context;
mod output;

use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};

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
    status         Dashboard showing your projects and open tickets
    browse         Open a project, repo, or ticket in the web browser
    pr             Shorthand for merge request commands (alias for repo merge-request commands)
    completions    Generate shell completions for bash, zsh, or fish
    version        Display the current version of cb

CONTEXTUAL DEFAULTS:
    When run inside a git repository with a CodebaseHQ remote, the project and
    repository arguments can be omitted. cb will auto-detect them from the
    git remote URL (origin).

OUTPUT:
    Use --json on any command to get machine-readable JSON output instead of
    the default human-readable colored output.

EXAMPLES:
    cb login mycompany/jdoe abc123def456
    cb project list
    cb project list --json
    cb repo branches my-project my-repo
    cb repo commits my-project my-repo main --path src/
    cb ticket create my-project \"Fix bug\" --ticket-type bug --priority-id 1
    cb ticket add-note my-project 42 --content \"Fixed\" --status-id 3
    cb pr list my-project my-repo
    cb pr create my-project my-repo feature main \"Add feature\"
    cb status
    cb browse my-project
    cb completions zsh
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
    /// Output results as JSON
    #[arg(long, global = true)]
    json: bool,

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
    /// Dashboard showing your projects and open tickets
    Status,
    /// Open a project, repo, or ticket in the web browser
    Browse {
        /// Project permalink (auto-detected from git remote if omitted)
        project: Option<String>,
        /// Repository permalink or ticket number
        target: Option<String>,
    },
    /// Merge request commands (shorthand for repo merge-request operations)
    Pr {
        #[command(subcommand)]
        command: commands::pr::PrCommands,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: Shell,
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

/// Resolve a project argument: use the provided value or fall back to git context.
fn resolve_project(project: Option<String>) -> anyhow::Result<String> {
    match project {
        Some(p) => Ok(p),
        None => git_context::detect()
            .map(|ctx| ctx.project)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No project specified and could not detect from git remote.\n\
                     Either provide the project argument or run from a git repo with a CodebaseHQ remote."
                )
            }),
    }
}

/// Resolve project and repo arguments from git context.
fn resolve_project_repo(
    project: Option<String>,
    repo: Option<String>,
) -> anyhow::Result<(String, String)> {
    let ctx = git_context::detect();
    let project = match project {
        Some(p) => p,
        None => ctx
            .as_ref()
            .map(|c| c.project.clone())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No project specified and could not detect from git remote.\n\
                     Either provide the project argument or run from a git repo with a CodebaseHQ remote."
                )
            })?,
    };
    let repo = match repo {
        Some(r) => r,
        None => ctx.and_then(|c| c.repo).ok_or_else(|| {
            anyhow::anyhow!(
                "No repository specified and could not detect from git remote.\n\
                     Please provide the repository argument."
            )
        })?,
    };
    Ok((project, repo))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let json = cli.json;

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
            commands::projects::execute(&client, command, json).await?;
        }
        Commands::Repo { command } => {
            let client = load_client()?;
            commands::repositories::execute(&client, command, json).await?;
        }
        Commands::Ticket { command } => {
            let client = load_client()?;
            commands::tickets::execute(&client, command, json).await?;
        }
        Commands::Milestone { command } => {
            let client = load_client()?;
            commands::milestones::execute(&client, command, json).await?;
        }
        Commands::Activity { command } => {
            let client = load_client()?;
            commands::activity::execute(&client, command, json).await?;
        }
        Commands::Status => {
            let client = load_client()?;
            commands::status::execute(&client, json).await?;
        }
        Commands::Browse { project, target } => {
            let config = Config::load()?;
            commands::browse::execute(&config, project, target)?;
        }
        Commands::Pr { command } => {
            let client = load_client()?;
            commands::pr::execute(&client, command, json).await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "cb", &mut io::stdout());
        }
        Commands::Version => {
            println!("cb {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
