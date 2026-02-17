use anyhow::Result;
use quick_xml::de::from_str;

use super::client::CodebaseClient;
use super::models::*;

pub async fn list_tickets(client: &CodebaseClient, project: &str) -> Result<Vec<Ticket>> {
    let xml = client.get(&format!("/{}/tickets", project)).await?;
    let tickets: Tickets = from_str(&xml)?;
    Ok(tickets.tickets)
}

pub async fn search_tickets(
    client: &CodebaseClient,
    project: &str,
    query: &str,
) -> Result<Vec<Ticket>> {
    let encoded = urlencoding(query);
    let xml = client
        .get(&format!("/{}/tickets?query={}", project, encoded))
        .await?;
    let tickets: Tickets = from_str(&xml)?;
    Ok(tickets.tickets)
}

pub async fn create_ticket(
    client: &CodebaseClient,
    project: &str,
    summary: &str,
    ticket_type: &str,
    priority_id: Option<i64>,
    status_id: Option<i64>,
    description: Option<&str>,
    assignee_id: Option<i64>,
    category_id: Option<i64>,
    milestone_id: Option<i64>,
    tags: Option<&str>,
) -> Result<Ticket> {
    let mut fields = String::new();
    fields.push_str(&format!("<summary>{}</summary>", summary));
    fields.push_str(&format!("<ticket-type>{}</ticket-type>", ticket_type));
    if let Some(id) = priority_id {
        fields.push_str(&format!("<priority-id>{}</priority-id>", id));
    }
    if let Some(id) = status_id {
        fields.push_str(&format!("<status-id>{}</status-id>", id));
    }
    if let Some(desc) = description {
        fields.push_str(&format!("<description><![CDATA[{}]]></description>", desc));
    }
    if let Some(id) = assignee_id {
        fields.push_str(&format!("<assignee-id>{}</assignee-id>", id));
    }
    if let Some(id) = category_id {
        fields.push_str(&format!("<category-id>{}</category-id>", id));
    }
    if let Some(id) = milestone_id {
        fields.push_str(&format!("<milestone-id>{}</milestone-id>", id));
    }
    if let Some(t) = tags {
        fields.push_str(&format!("<tags>{}</tags>", t));
    }
    let body = format!("<ticket>{}</ticket>", fields);
    let xml = client.post(&format!("/{}/tickets", project), body).await?;
    let ticket: Ticket = from_str(&xml)?;
    Ok(ticket)
}

pub async fn list_ticket_notes(
    client: &CodebaseClient,
    project: &str,
    ticket_id: i64,
) -> Result<Vec<TicketNote>> {
    let xml = client
        .get(&format!("/{}/tickets/{}/notes", project, ticket_id))
        .await?;
    let notes: TicketNotes = from_str(&xml)?;
    Ok(notes.notes)
}

pub async fn create_ticket_note(
    client: &CodebaseClient,
    project: &str,
    ticket_id: i64,
    content: Option<&str>,
    changes: Option<&NoteChanges>,
    private: bool,
) -> Result<TicketNote> {
    let mut fields = String::new();
    if let Some(c) = content {
        fields.push_str(&format!("<content><![CDATA[{}]]></content>", c));
    }
    if private {
        fields.push_str("<private>1</private>");
    }
    if let Some(ch) = changes {
        let mut ch_fields = String::new();
        if let Some(id) = ch.status_id {
            ch_fields.push_str(&format!("<status-id>{}</status-id>", id));
        }
        if let Some(id) = ch.priority_id {
            ch_fields.push_str(&format!("<priority-id>{}</priority-id>", id));
        }
        if let Some(id) = ch.category_id {
            ch_fields.push_str(&format!("<category-id>{}</category-id>", id));
        }
        if let Some(id) = ch.assignee_id {
            ch_fields.push_str(&format!("<assignee-id>{}</assignee-id>", id));
        }
        if let Some(id) = ch.milestone_id {
            ch_fields.push_str(&format!("<milestone-id>{}</milestone-id>", id));
        }
        if let Some(ref s) = ch.subject {
            ch_fields.push_str(&format!("<subject>{}</subject>", s));
        }
        fields.push_str(&format!("<changes>{}</changes>", ch_fields));
    }
    let body = format!("<ticket-note>{}</ticket-note>", fields);
    let xml = client
        .post(&format!("/{}/tickets/{}/notes", project, ticket_id), body)
        .await?;
    let note: TicketNote = from_str(&xml)?;
    Ok(note)
}

pub async fn list_watchers(
    client: &CodebaseClient,
    project: &str,
    ticket_id: i64,
) -> Result<Vec<Watcher>> {
    let xml = client
        .get(&format!("/{}/tickets/{}/watchers", project, ticket_id))
        .await?;
    let watchers: Watchers = from_str(&xml)?;
    Ok(watchers.watchers)
}

pub async fn set_watchers(
    client: &CodebaseClient,
    project: &str,
    ticket_id: i64,
    user_ids: &[i64],
) -> Result<()> {
    let watchers_xml: String = user_ids
        .iter()
        .map(|id| format!("<watcher>{}</watcher>", id))
        .collect();
    let body = format!("<watchers>{}</watchers>", watchers_xml);
    client
        .post(
            &format!("/{}/tickets/{}/watchers", project, ticket_id),
            body,
        )
        .await?;
    Ok(())
}

pub async fn list_statuses(client: &CodebaseClient, project: &str) -> Result<Vec<TicketStatus>> {
    let xml = client
        .get(&format!("/{}/tickets/statuses", project))
        .await?;
    let statuses: TicketStatuses = from_str(&xml)?;
    Ok(statuses.statuses)
}

pub async fn list_priorities(
    client: &CodebaseClient,
    project: &str,
) -> Result<Vec<TicketPriority>> {
    let xml = client
        .get(&format!("/{}/tickets/priorities", project))
        .await?;
    let priorities: TicketPriorities = from_str(&xml)?;
    Ok(priorities.priorities)
}

pub async fn list_categories(
    client: &CodebaseClient,
    project: &str,
) -> Result<Vec<TicketCategory>> {
    let xml = client
        .get(&format!("/{}/tickets/categories", project))
        .await?;
    let categories: TicketCategories = from_str(&xml)?;
    Ok(categories.categories)
}

pub async fn list_types(client: &CodebaseClient, project: &str) -> Result<Vec<TicketType>> {
    let xml = client.get(&format!("/{}/tickets/types", project)).await?;
    let types: TicketTypes = from_str(&xml)?;
    Ok(types.types)
}

fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20")
        .replace(':', "%3A")
        .replace('=', "%3D")
}
