use anyhow::Result;
use clap::Subcommand;

use crate::api::client::CodebaseClient;
use crate::api::models::NoteChanges;
use crate::api::tickets;

#[derive(Subcommand)]
pub enum TicketCommands {
    /// List all tickets for a project
    List {
        /// Project permalink
        project: String,
    },
    /// Search tickets
    Search {
        /// Project permalink
        project: String,
        /// Search query (e.g. "status:closed")
        query: String,
    },
    /// Create a new ticket
    Create {
        /// Project permalink
        project: String,
        /// Ticket summary
        summary: String,
        /// Ticket type: bug, enhancement, or task
        #[arg(long, default_value = "task")]
        ticket_type: String,
        /// Priority ID
        #[arg(long)]
        priority_id: Option<i64>,
        /// Status ID
        #[arg(long)]
        status_id: Option<i64>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Assignee user ID
        #[arg(long)]
        assignee_id: Option<i64>,
        /// Category ID
        #[arg(long)]
        category_id: Option<i64>,
        /// Milestone ID
        #[arg(long)]
        milestone_id: Option<i64>,
        /// Space-separated tags
        #[arg(long)]
        tags: Option<String>,
    },
    /// List notes for a ticket
    Notes {
        /// Project permalink
        project: String,
        /// Ticket ID
        ticket_id: i64,
    },
    /// Add a note (and optionally update fields) on a ticket
    AddNote {
        /// Project permalink
        project: String,
        /// Ticket ID
        ticket_id: i64,
        /// Note content
        #[arg(long)]
        content: Option<String>,
        /// Make private (company-only)
        #[arg(long)]
        private: bool,
        /// Change status ID
        #[arg(long)]
        status_id: Option<i64>,
        /// Change priority ID
        #[arg(long)]
        priority_id: Option<i64>,
        /// Change assignee ID
        #[arg(long)]
        assignee_id: Option<i64>,
        /// Change category ID
        #[arg(long)]
        category_id: Option<i64>,
        /// Change milestone ID
        #[arg(long)]
        milestone_id: Option<i64>,
        /// Change subject
        #[arg(long)]
        subject: Option<String>,
    },
    /// List watchers for a ticket
    Watchers {
        /// Project permalink
        project: String,
        /// Ticket ID
        ticket_id: i64,
    },
    /// Set watchers for a ticket (overwrites existing)
    SetWatchers {
        /// Project permalink
        project: String,
        /// Ticket ID
        ticket_id: i64,
        /// User IDs to set as watchers
        #[arg(required = true)]
        user_ids: Vec<i64>,
    },
    /// List ticket statuses
    Statuses {
        /// Project permalink
        project: String,
    },
    /// List ticket priorities
    Priorities {
        /// Project permalink
        project: String,
    },
    /// List ticket categories
    Categories {
        /// Project permalink
        project: String,
    },
    /// List ticket types
    Types {
        /// Project permalink
        project: String,
    },
}

pub async fn execute(client: &CodebaseClient, cmd: TicketCommands) -> Result<()> {
    match cmd {
        TicketCommands::List { project } => {
            let tix = tickets::list_tickets(client, &project).await?;
            for t in tix {
                println!(
                    "#{} [{}] {}",
                    t.ticket_id.unwrap_or(0),
                    t.ticket_type.unwrap_or_default(),
                    t.summary.unwrap_or_default()
                );
            }
        }
        TicketCommands::Search { project, query } => {
            let tix = tickets::search_tickets(client, &project, &query).await?;
            for t in tix {
                println!(
                    "#{} [{}] {}",
                    t.ticket_id.unwrap_or(0),
                    t.ticket_type.unwrap_or_default(),
                    t.summary.unwrap_or_default()
                );
            }
        }
        TicketCommands::Create {
            project,
            summary,
            ticket_type,
            priority_id,
            status_id,
            description,
            assignee_id,
            category_id,
            milestone_id,
            tags,
        } => {
            let t = tickets::create_ticket(
                client,
                &project,
                &summary,
                &ticket_type,
                priority_id,
                status_id,
                description.as_deref(),
                assignee_id,
                category_id,
                milestone_id,
                tags.as_deref(),
            )
            .await?;
            println!(
                "Created ticket #{}: {}",
                t.ticket_id.unwrap_or(0),
                t.summary.unwrap_or_default()
            );
        }
        TicketCommands::Notes { project, ticket_id } => {
            let notes = tickets::list_ticket_notes(client, &project, ticket_id).await?;
            for n in notes {
                println!(
                    "Note #{}: {}",
                    n.id.unwrap_or(0),
                    n.content.unwrap_or_default()
                );
            }
        }
        TicketCommands::AddNote {
            project,
            ticket_id,
            content,
            private,
            status_id,
            priority_id,
            assignee_id,
            category_id,
            milestone_id,
            subject,
        } => {
            let changes = if status_id.is_some()
                || priority_id.is_some()
                || assignee_id.is_some()
                || category_id.is_some()
                || milestone_id.is_some()
                || subject.is_some()
            {
                Some(NoteChanges {
                    status_id,
                    priority_id,
                    category_id,
                    assignee_id,
                    milestone_id,
                    subject,
                })
            } else {
                None
            };
            let n = tickets::create_ticket_note(
                client,
                &project,
                ticket_id,
                content.as_deref(),
                changes.as_ref(),
                private,
            )
            .await?;
            println!("Added note #{}", n.id.unwrap_or(0));
        }
        TicketCommands::Watchers { project, ticket_id } => {
            let watchers = tickets::list_watchers(client, &project, ticket_id).await?;
            for w in watchers {
                println!("User ID: {}", w.watcher.unwrap_or(0));
            }
        }
        TicketCommands::SetWatchers {
            project,
            ticket_id,
            user_ids,
        } => {
            tickets::set_watchers(client, &project, ticket_id, &user_ids).await?;
            println!("Set {} watchers on ticket #{}", user_ids.len(), ticket_id);
        }
        TicketCommands::Statuses { project } => {
            let statuses = tickets::list_statuses(client, &project).await?;
            for s in statuses {
                println!(
                    "{}: {} (closed: {})",
                    s.id.unwrap_or(0),
                    s.name.unwrap_or_default(),
                    s.treat_as_closed.unwrap_or(false)
                );
            }
        }
        TicketCommands::Priorities { project } => {
            let priorities = tickets::list_priorities(client, &project).await?;
            for p in priorities {
                println!(
                    "{}: {} (default: {})",
                    p.id.unwrap_or(0),
                    p.name.unwrap_or_default(),
                    p.default.unwrap_or(false)
                );
            }
        }
        TicketCommands::Categories { project } => {
            let categories = tickets::list_categories(client, &project).await?;
            for c in categories {
                println!("{}: {}", c.id.unwrap_or(0), c.name.unwrap_or_default());
            }
        }
        TicketCommands::Types { project } => {
            let types = tickets::list_types(client, &project).await?;
            for t in types {
                println!("{}: {}", t.id.unwrap_or(0), t.name.unwrap_or_default());
            }
        }
    }
    Ok(())
}
