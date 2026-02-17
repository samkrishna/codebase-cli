use anyhow::Result;

use crate::api::config::Config;
use crate::git_context;

pub fn execute(config: &Config, project: Option<String>, target: Option<String>) -> Result<()> {
    let account = config.account();

    let project = match project {
        Some(p) => p,
        None => git_context::detect()
            .map(|ctx| ctx.project)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No project specified and could not detect from git remote.\n\
                     Usage: cb browse <project> [target]"
                )
            })?,
    };

    let url = match target {
        Some(ref t) if t.parse::<i64>().is_ok() => {
            // Ticket number
            format!(
                "https://{}.codebasehq.com/projects/{}/tickets/{}",
                account, project, t
            )
        }
        Some(ref t) => {
            // Repository permalink
            format!(
                "https://{}.codebasehq.com/projects/{}/repositories/{}",
                account, project, t
            )
        }
        None => {
            // Project page
            format!("https://{}.codebasehq.com/projects/{}", account, project)
        }
    };

    println!("Opening {}", url);
    open::that(&url)?;
    Ok(())
}
