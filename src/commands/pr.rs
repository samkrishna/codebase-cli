use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::api::client::CodebaseClient;
use crate::api::repositories;
use crate::output;

#[derive(Subcommand)]
pub enum PrCommands {
    /// List merge requests
    List {
        /// Project permalink (auto-detected if omitted)
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        repo: Option<String>,
    },
    /// Show a merge request
    Show {
        /// Merge request ID
        mr_id: i64,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Create a merge request
    Create {
        /// Source branch (merging from)
        source_ref: String,
        /// Target branch (merging into)
        target_ref: String,
        /// Subject/description
        subject: String,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Comment on a merge request
    Comment {
        /// Merge request ID
        mr_id: i64,
        /// Comment content
        content: String,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Merge a merge request
    Merge {
        /// Merge request ID
        mr_id: i64,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Close a merge request
    Close {
        /// Merge request ID
        mr_id: i64,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Reopen a merge request
    Reopen {
        /// Merge request ID
        mr_id: i64,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
    /// Reassign a merge request
    Reassign {
        /// Merge request ID
        mr_id: i64,
        /// User ID to assign to
        user_id: i64,
        /// Project permalink (auto-detected if omitted)
        #[arg(long)]
        project: Option<String>,
        /// Repository permalink (auto-detected if omitted)
        #[arg(long)]
        repo: Option<String>,
    },
}

fn resolve(project: Option<String>, repo: Option<String>) -> Result<(String, String)> {
    crate::resolve_project_repo(project, repo)
}

pub async fn execute(client: &CodebaseClient, cmd: PrCommands, json: bool) -> Result<()> {
    match cmd {
        PrCommands::List { project, repo } => {
            let (project, repo) = resolve(project, repo)?;
            let mrs = repositories::list_merge_requests(client, &project, &repo).await?;
            output::print_list(json, &mrs, |mrs| {
                for mr in mrs {
                    let status =
                        output::colorize_mr_status(mr.status.as_deref().unwrap_or("unknown"));
                    println!(
                        "#{} [{}] {} ({} -> {})",
                        mr.id.unwrap_or(0).to_string().bold(),
                        status,
                        mr.subject.as_deref().unwrap_or(""),
                        mr.source_ref.as_deref().unwrap_or("").cyan(),
                        mr.target_ref.as_deref().unwrap_or("").green(),
                    );
                }
            })?;
        }
        PrCommands::Show {
            mr_id,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            let mr = repositories::show_merge_request(client, &project, &repo, mr_id).await?;
            output::print_output(json, &mr, || {
                let status = output::colorize_mr_status(mr.status.as_deref().unwrap_or("unknown"));
                let can_merge = output::colorize_bool(mr.can_merge.unwrap_or(false), "yes", "no");
                println!("ID:         {}", mr.id.unwrap_or(0).to_string().bold());
                println!("Subject:    {}", mr.subject.as_deref().unwrap_or(""));
                println!("Status:     {}", status);
                println!(
                    "Source:     {}",
                    mr.source_ref.as_deref().unwrap_or("").cyan()
                );
                println!(
                    "Target:     {}",
                    mr.target_ref.as_deref().unwrap_or("").green()
                );
                println!("Can Merge:  {}", can_merge);
                println!(
                    "Created:    {}",
                    mr.created_at.as_deref().unwrap_or("").dimmed()
                );
                println!(
                    "Updated:    {}",
                    mr.updated_at.as_deref().unwrap_or("").dimmed()
                );
            })?;
        }
        PrCommands::Create {
            source_ref,
            target_ref,
            subject,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            let mr = repositories::create_merge_request(
                client,
                &project,
                &repo,
                &source_ref,
                &target_ref,
                &subject,
            )
            .await?;
            output::print_output(json, &mr, || {
                println!(
                    "Created merge request #{}: {}",
                    mr.id.unwrap_or(0).to_string().bold(),
                    mr.subject.as_deref().unwrap_or("")
                );
            })?;
        }
        PrCommands::Comment {
            mr_id,
            content,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            repositories::comment_merge_request(client, &project, &repo, mr_id, &content).await?;
            println!("Commented on merge request #{}", mr_id);
        }
        PrCommands::Merge {
            mr_id,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            repositories::merge_merge_request(client, &project, &repo, mr_id).await?;
            println!("Merged merge request #{}", mr_id.to_string().bold());
        }
        PrCommands::Close {
            mr_id,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            repositories::close_merge_request(client, &project, &repo, mr_id).await?;
            println!("Closed merge request #{}", mr_id);
        }
        PrCommands::Reopen {
            mr_id,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            repositories::reopen_merge_request(client, &project, &repo, mr_id).await?;
            println!("Reopened merge request #{}", mr_id);
        }
        PrCommands::Reassign {
            mr_id,
            user_id,
            project,
            repo,
        } => {
            let (project, repo) = resolve(project, repo)?;
            repositories::reassign_merge_request(client, &project, &repo, mr_id, user_id).await?;
            println!("Reassigned merge request #{} to user {}", mr_id, user_id);
        }
    }
    Ok(())
}
