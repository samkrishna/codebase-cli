use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;

use crate::api::client::CodebaseClient;
use crate::api::models::NoteChanges;
use crate::api::tickets;
use crate::output;

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

pub async fn execute(client: &CodebaseClient, cmd: TicketCommands, json: bool) -> Result<()> {
    match cmd {
        TicketCommands::List { project } => {
            let tix = tickets::list_tickets(client, &project).await?;
            output::print_list(json, &tix, |tix| {
                for t in tix {
                    let tt = output::colorize_ticket_type(t.ticket_type.as_deref().unwrap_or(""));
                    println!(
                        "#{} [{}] {}",
                        t.ticket_id.unwrap_or(0).to_string().bold(),
                        tt,
                        t.summary.as_deref().unwrap_or("")
                    );
                }
            })?;
        }
        TicketCommands::Search { project, query } => {
            let tix = tickets::search_tickets(client, &project, &query).await?;
            output::print_list(json, &tix, |tix| {
                for t in tix {
                    let tt = output::colorize_ticket_type(t.ticket_type.as_deref().unwrap_or(""));
                    println!(
                        "#{} [{}] {}",
                        t.ticket_id.unwrap_or(0).to_string().bold(),
                        tt,
                        t.summary.as_deref().unwrap_or("")
                    );
                }
            })?;
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
            output::print_output(json, &t, || {
                println!(
                    "Created ticket #{}: {}",
                    t.ticket_id.unwrap_or(0).to_string().bold(),
                    t.summary.as_deref().unwrap_or("")
                );
            })?;
        }
        TicketCommands::Notes { project, ticket_id } => {
            let notes = tickets::list_ticket_notes(client, &project, ticket_id).await?;
            output::print_list(json, &notes, |notes| {
                for n in notes {
                    println!(
                        "Note #{}: {}",
                        n.id.unwrap_or(0).to_string().bold(),
                        n.content.as_deref().unwrap_or("").dimmed()
                    );
                }
            })?;
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
            output::print_output(json, &n, || {
                println!("Added note #{}", n.id.unwrap_or(0).to_string().bold());
            })?;
        }
        TicketCommands::Watchers { project, ticket_id } => {
            let watchers = tickets::list_watchers(client, &project, ticket_id).await?;
            output::print_list(json, &watchers, |watchers| {
                for w in watchers {
                    println!("User ID: {}", w.watcher.unwrap_or(0));
                }
            })?;
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
            output::print_list(json, &statuses, |statuses| {
                for s in statuses {
                    let closed =
                        output::colorize_bool(s.treat_as_closed.unwrap_or(false), "closed", "open");
                    println!(
                        "{}: {} ({})",
                        s.id.unwrap_or(0),
                        s.name.as_deref().unwrap_or("").bold(),
                        closed,
                    );
                }
            })?;
        }
        TicketCommands::Priorities { project } => {
            let priorities = tickets::list_priorities(client, &project).await?;
            output::print_list(json, &priorities, |priorities| {
                for p in priorities {
                    let name = output::colorize_priority(p.name.as_deref().unwrap_or(""));
                    let default_marker = if p.default.unwrap_or(false) {
                        " *".green().to_string()
                    } else {
                        String::new()
                    };
                    println!("{}: {}{}", p.id.unwrap_or(0), name, default_marker);
                }
            })?;
        }
        TicketCommands::Categories { project } => {
            let categories = tickets::list_categories(client, &project).await?;
            output::print_list(json, &categories, |categories| {
                for c in categories {
                    println!(
                        "{}: {}",
                        c.id.unwrap_or(0),
                        c.name.as_deref().unwrap_or("").bold()
                    );
                }
            })?;
        }
        TicketCommands::Types { project } => {
            let types = tickets::list_types(client, &project).await?;
            output::print_list(json, &types, |types| {
                for t in types {
                    let name = output::colorize_ticket_type(t.name.as_deref().unwrap_or(""));
                    println!("{}: {}", t.id.unwrap_or(0), name);
                }
            })?;
        }
    }
    Ok(())
}
