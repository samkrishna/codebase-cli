use anyhow::Result;
use clap::Subcommand;

use crate::api::client::CodebaseClient;
use crate::api::repositories;

#[derive(Subcommand)]
pub enum RepoCommands {
    /// List repositories for a project
    List {
        /// Project permalink
        project: String,
    },
    /// Show a specific repository
    Show {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
    },
    /// Create a new repository
    Create {
        /// Project permalink
        project: String,
        /// Repository name
        name: String,
        /// SCM type (e.g. git)
        #[arg(long, default_value = "git")]
        scm: String,
    },
    /// Delete a repository
    Delete {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
    },
}

pub async fn execute(client: &CodebaseClient, cmd: RepoCommands) -> Result<()> {
    match cmd {
        RepoCommands::List { project } => {
            let repos = repositories::list_repositories(client, &project).await?;
            for r in repos {
                println!(
                    "{} ({}) clone: {}",
                    r.name.unwrap_or_default(),
                    r.permalink.unwrap_or_default(),
                    r.clone_url.unwrap_or_default()
                );
            }
        }
        RepoCommands::Show { project, repo } => {
            let r = repositories::show_repository(client, &project, &repo).await?;
            println!("Name:       {}", r.name.unwrap_or_default());
            println!("Permalink:  {}", r.permalink.unwrap_or_default());
            println!("Clone URL:  {}", r.clone_url.unwrap_or_default());
            println!("Disk Usage: {} bytes", r.disk_usage.unwrap_or(0));
            println!("Last Commit: {}", r.last_commit_ref.unwrap_or_default());
        }
        RepoCommands::Create { project, name, scm } => {
            let r = repositories::create_repository(client, &project, &name, &scm).await?;
            println!(
                "Created repository: {} ({})",
                r.name.unwrap_or_default(),
                r.permalink.unwrap_or_default()
            );
        }
        RepoCommands::Delete { project, repo } => {
            repositories::delete_repository(client, &project, &repo).await?;
            println!("Deleted repository: {}/{}", project, repo);
        }
    }
    Ok(())
}
