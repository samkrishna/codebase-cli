use anyhow::{Context, Result};
use colored::Colorize;
use serde::Serialize;

/// Print data as pretty JSON when --json flag is set, otherwise run the human-readable closure.
pub fn print_output<T, F>(json: bool, data: &T, human: F) -> Result<()>
where
    T: Serialize,
    F: FnOnce(),
{
    if json {
        let output = serde_json::to_string_pretty(data).context("Failed to serialize to JSON")?;
        println!("{}", output);
    } else {
        human();
    }
    Ok(())
}

/// Print a list as JSON or iterate with a human-readable formatter.
pub fn print_list<T, F>(json: bool, data: &[T], human: F) -> Result<()>
where
    T: Serialize,
    F: FnOnce(&[T]),
{
    if json {
        let output = serde_json::to_string_pretty(data).context("Failed to serialize to JSON")?;
        println!("{}", output);
    } else {
        human(data);
    }
    Ok(())
}

// ── Color helpers ──

pub fn colorize_status(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "active" | "open" | "new" => status.green().to_string(),
        "on_hold" | "on hold" | "in_progress" | "in progress" => status.yellow().to_string(),
        "archived" | "closed" | "completed" | "resolved" => status.red().to_string(),
        "cancelled" | "rejected" => status.red().dimmed().to_string(),
        _ => status.to_string(),
    }
}

pub fn colorize_priority(priority: &str) -> String {
    match priority.to_lowercase().as_str() {
        "critical" => priority.red().bold().to_string(),
        "high" => priority.red().to_string(),
        "normal" | "medium" => priority.yellow().to_string(),
        "low" => priority.green().to_string(),
        _ => priority.to_string(),
    }
}

pub fn colorize_ticket_type(ticket_type: &str) -> String {
    match ticket_type.to_lowercase().as_str() {
        "bug" => ticket_type.red().to_string(),
        "enhancement" | "feature" => ticket_type.cyan().to_string(),
        "task" => ticket_type.blue().to_string(),
        _ => ticket_type.to_string(),
    }
}

pub fn colorize_mr_status(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "new" | "open" => status.green().to_string(),
        "merged" => status.magenta().to_string(),
        "closed" | "rejected" => status.red().to_string(),
        _ => status.to_string(),
    }
}

pub fn colorize_bool(val: bool, true_label: &str, false_label: &str) -> String {
    if val {
        true_label.green().to_string()
    } else {
        false_label.red().to_string()
    }
}

pub fn dim(text: &str) -> String {
    text.dimmed().to_string()
}

pub fn bold(text: &str) -> String {
    text.bold().to_string()
}
