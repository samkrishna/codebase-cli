use std::process::Command;

/// Detected project and repository from the current git working directory.
#[derive(Debug, Clone)]
pub struct GitContext {
    pub project: String,
    pub repo: Option<String>,
}

/// Try to detect the CodebaseHQ project and repository from the current
/// git remote URL. Supports both SSH and HTTPS clone URLs.
///
/// SSH:   git@codebasehq.com:account/project/repo.git
/// HTTPS: https://account.codebasehq.com/project/repo.git
pub fn detect() -> Option<GitContext> {
    let remote_url = get_remote_url("origin")?;
    parse_codebase_remote(&remote_url)
}

fn get_remote_url(remote: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--get", &format!("remote.{}.url", remote)])
        .output()
        .ok()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if url.is_empty() { None } else { Some(url) }
    } else {
        None
    }
}

fn parse_codebase_remote(url: &str) -> Option<GitContext> {
    // SSH: git@codebasehq.com:account/project/repo.git
    if url.contains("codebasehq.com:") {
        let path = url.split(':').last()?;
        let path = path.strip_suffix(".git").unwrap_or(path);
        let parts: Vec<&str> = path.split('/').collect();
        return match parts.len() {
            // account/project/repo
            3 => Some(GitContext {
                project: parts[1].to_string(),
                repo: Some(parts[2].to_string()),
            }),
            // account/project
            2 => Some(GitContext {
                project: parts[1].to_string(),
                repo: None,
            }),
            _ => None,
        };
    }

    // HTTPS: https://account.codebasehq.com/project/repo.git
    if url.contains("codebasehq.com/") {
        let path = url.split("codebasehq.com/").nth(1)?;
        let path = path.strip_suffix(".git").unwrap_or(path);
        let parts: Vec<&str> = path.split('/').collect();
        return match parts.len() {
            2 => Some(GitContext {
                project: parts[0].to_string(),
                repo: Some(parts[1].to_string()),
            }),
            1 if !parts[0].is_empty() => Some(GitContext {
                project: parts[0].to_string(),
                repo: None,
            }),
            _ => None,
        };
    }

    None
}

/// Get the current git branch name.
pub fn current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if branch.is_empty() || branch == "HEAD" {
            None
        } else {
            Some(branch)
        }
    } else {
        None
    }
}

/// Get the repository root directory.
pub fn repo_root() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;

    if output.status.success() {
        let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if root.is_empty() { None } else { Some(root) }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ssh_remote() {
        let url = "git@codebasehq.com:mycompany/myproject/myrepo.git";
        let ctx = parse_codebase_remote(url).unwrap();
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.unwrap(), "myrepo");
    }

    #[test]
    fn test_parse_https_remote() {
        let url = "https://mycompany.codebasehq.com/myproject/myrepo.git";
        let ctx = parse_codebase_remote(url).unwrap();
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.unwrap(), "myrepo");
    }

    #[test]
    fn test_parse_ssh_no_git_suffix() {
        let url = "git@codebasehq.com:mycompany/myproject/myrepo";
        let ctx = parse_codebase_remote(url).unwrap();
        assert_eq!(ctx.project, "myproject");
        assert_eq!(ctx.repo.unwrap(), "myrepo");
    }

    #[test]
    fn test_parse_non_codebase_remote() {
        let url = "git@github.com:user/repo.git";
        assert!(parse_codebase_remote(url).is_none());
    }
}
