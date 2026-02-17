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
    /// List commits for a ref (branch, tag, or commit)
    Commits {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Git ref (branch name, tag, or commit SHA)
        git_ref: String,
        /// Optional file/folder path to filter commits
        #[arg(long)]
        path: Option<String>,
    },
    /// Create a deployment
    Deploy {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Branch being deployed
        branch: String,
        /// Revision/commit SHA being deployed
        revision: String,
        /// Servers (comma-separated)
        servers: String,
        /// Environment name (e.g. production, staging)
        #[arg(long)]
        environment: Option<String>,
    },
    /// Get file contents at a ref and path
    File {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Git ref (branch, tag, or commit)
        git_ref: String,
        /// Path to file
        path: String,
    },
    /// List hooks for a repository
    Hooks {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
    },
    /// Create a hook for a repository
    CreateHook {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// URL to receive the hook
        url: String,
        /// Username for basic auth
        #[arg(long)]
        username: Option<String>,
        /// Password for basic auth
        #[arg(long)]
        password: Option<String>,
    },
    /// List branches for a repository
    Branches {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
    },
    /// List merge requests for a repository
    MergeRequests {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
    },
    /// Show a specific merge request
    ShowMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
    },
    /// Create a new merge request
    CreateMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Source branch (merging from)
        source_ref: String,
        /// Target branch (merging into)
        target_ref: String,
        /// Subject/description
        subject: String,
    },
    /// Comment on a merge request
    CommentMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
        /// Comment content
        content: String,
    },
    /// Close a merge request
    CloseMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
    },
    /// Reopen a merge request
    ReopenMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
    },
    /// Perform automatic merge on a merge request
    Merge {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
    },
    /// Reassign a merge request to a different user
    ReassignMr {
        /// Project permalink
        project: String,
        /// Repository permalink
        repo: String,
        /// Merge request ID
        mr_id: i64,
        /// User ID to assign to
        user_id: i64,
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
        RepoCommands::Commits {
            project,
            repo,
            git_ref,
            path,
        } => {
            let commits = if let Some(p) = path {
                repositories::list_commits_path(client, &project, &repo, &git_ref, &p).await?
            } else {
                repositories::list_commits(client, &project, &repo, &git_ref).await?
            };
            for c in commits {
                println!(
                    "{} {} <{}> {}",
                    c.commit_ref.unwrap_or_default(),
                    c.author_name.unwrap_or_default(),
                    c.author_email.unwrap_or_default(),
                    c.message.unwrap_or_default().lines().next().unwrap_or("")
                );
            }
        }
        RepoCommands::Deploy {
            project,
            repo,
            branch,
            revision,
            servers,
            environment,
        } => {
            repositories::create_deployment(
                client,
                &project,
                &repo,
                &branch,
                &revision,
                &servers,
                environment.as_deref(),
            )
            .await?;
            println!("Deployed {} ({}) to {}", branch, revision, servers);
        }
        RepoCommands::File {
            project,
            repo,
            git_ref,
            path,
        } => {
            let content = repositories::get_file(client, &project, &repo, &git_ref, &path).await?;
            println!("{}", content);
        }
        RepoCommands::Hooks { project, repo } => {
            let hooks = repositories::list_hooks(client, &project, &repo).await?;
            for h in hooks {
                println!("{}: {}", h.id.unwrap_or(0), h.url.unwrap_or_default());
            }
        }
        RepoCommands::CreateHook {
            project,
            repo,
            url,
            username,
            password,
        } => {
            let h = repositories::create_hook(
                client,
                &project,
                &repo,
                &url,
                username.as_deref(),
                password.as_deref(),
            )
            .await?;
            println!(
                "Created hook {}: {}",
                h.id.unwrap_or(0),
                h.url.unwrap_or_default()
            );
        }
        RepoCommands::Branches { project, repo } => {
            let branches = repositories::list_branches(client, &project, &repo).await?;
            for b in branches {
                println!("{}", b.name.unwrap_or_default());
            }
        }
        RepoCommands::MergeRequests { project, repo } => {
            let mrs = repositories::list_merge_requests(client, &project, &repo).await?;
            for mr in mrs {
                println!(
                    "#{} [{}] {} ({} -> {})",
                    mr.id.unwrap_or(0),
                    mr.status.unwrap_or_default(),
                    mr.subject.unwrap_or_default(),
                    mr.source_ref.unwrap_or_default(),
                    mr.target_ref.unwrap_or_default()
                );
            }
        }
        RepoCommands::ShowMr {
            project,
            repo,
            mr_id,
        } => {
            let mr = repositories::show_merge_request(client, &project, &repo, mr_id).await?;
            println!("ID:         {}", mr.id.unwrap_or(0));
            println!("Subject:    {}", mr.subject.unwrap_or_default());
            println!("Status:     {}", mr.status.unwrap_or_default());
            println!("Source:     {}", mr.source_ref.unwrap_or_default());
            println!("Target:     {}", mr.target_ref.unwrap_or_default());
            println!("Can Merge:  {}", mr.can_merge.unwrap_or(false));
            println!("Created:    {}", mr.created_at.unwrap_or_default());
            println!("Updated:    {}", mr.updated_at.unwrap_or_default());
        }
        RepoCommands::CreateMr {
            project,
            repo,
            source_ref,
            target_ref,
            subject,
        } => {
            let mr = repositories::create_merge_request(
                client,
                &project,
                &repo,
                &source_ref,
                &target_ref,
                &subject,
            )
            .await?;
            println!(
                "Created merge request #{}: {}",
                mr.id.unwrap_or(0),
                mr.subject.unwrap_or_default()
            );
        }
        RepoCommands::CommentMr {
            project,
            repo,
            mr_id,
            content,
        } => {
            repositories::comment_merge_request(client, &project, &repo, mr_id, &content).await?;
            println!("Commented on merge request #{}", mr_id);
        }
        RepoCommands::CloseMr {
            project,
            repo,
            mr_id,
        } => {
            repositories::close_merge_request(client, &project, &repo, mr_id).await?;
            println!("Closed merge request #{}", mr_id);
        }
        RepoCommands::ReopenMr {
            project,
            repo,
            mr_id,
        } => {
            repositories::reopen_merge_request(client, &project, &repo, mr_id).await?;
            println!("Reopened merge request #{}", mr_id);
        }
        RepoCommands::Merge {
            project,
            repo,
            mr_id,
        } => {
            repositories::merge_merge_request(client, &project, &repo, mr_id).await?;
            println!("Merged merge request #{}", mr_id);
        }
        RepoCommands::ReassignMr {
            project,
            repo,
            mr_id,
            user_id,
        } => {
            repositories::reassign_merge_request(client, &project, &repo, mr_id, user_id).await?;
            println!("Reassigned merge request #{} to user {}", mr_id, user_id);
        }
    }
    Ok(())
}
