# codebase-cli

A command-line utility implementation of the [CodebaseHQ](https://www.codebasehq.com/) API.

## Installation

```
cargo install --path .
```

This installs the `cb` binary.

## Configuration

```bash
cb login account/username your-api-key
```

Credentials are stored in `~/.config/cb/config.toml`. The API username format is `account/username` (e.g. `mycompany/jdoe`).

The client automatically retries with exponential backoff on 429 (rate limit), 503 (service unavailable), and 529 (overloaded) responses.

## Usage

```
cb <command> <subcommand> [options]
```

### Global Flags

```bash
cb --json <command>       # Output results as JSON instead of colored text
```

### Contextual Defaults

When run inside a git repository with a CodebaseHQ remote, `cb` auto-detects the project and repository from the `origin` remote URL. This means you can omit the project/repo arguments for `cb pr` commands when working inside a CodebaseHQ-cloned repo.

### Projects

```bash
cb project list
cb project list --json
cb project show my-project
cb project create "New Project"
cb project update my-project --name "Renamed" --status archived
cb project delete my-project
cb project groups
cb project users my-project
cb project assign-users my-project 101 102 103
```

### Repositories

```bash
cb repo list my-project
cb repo show my-project my-repo
cb repo create my-project "New Repo" --scm git
cb repo delete my-project my-repo
```

### Branches and Commits

```bash
cb repo branches my-project my-repo
cb repo commits my-project my-repo main
cb repo commits my-project my-repo main --path src/lib.rs
cb repo file my-project my-repo main README.md
```

### Hooks

```bash
cb repo hooks my-project my-repo
cb repo create-hook my-project my-repo https://example.com/webhook --username user --password pass
```

### Deployments

```bash
cb repo deploy my-project my-repo main abc123def "app1.example.com,app2.example.com" --environment production
```

### Merge Requests

```bash
cb repo merge-requests my-project my-repo
cb repo show-mr my-project my-repo 1
cb repo create-mr my-project my-repo feature-branch main "Add new feature"
cb repo comment-mr my-project my-repo 1 "Looks good to me"
cb repo merge my-project my-repo 1
cb repo close-mr my-project my-repo 1
cb repo reopen-mr my-project my-repo 1
cb repo reassign-mr my-project my-repo 1 42
```

### PR (Merge Request Shorthand)

The `pr` command provides a convenient shorthand for merge request operations, with auto-detection of project and repository from the git remote:

```bash
cb pr list                                        # auto-detect project/repo
cb pr list my-project my-repo                     # explicit project/repo
cb pr show 1 --project my-project --repo my-repo
cb pr create feature-branch main "Add feature"    # auto-detect project/repo
cb pr comment 1 "Looks good"
cb pr merge 1
cb pr close 1
cb pr reopen 1
cb pr reassign 1 42
```

### Tickets

```bash
cb ticket list my-project
cb ticket search my-project "status:open"
cb ticket create my-project "Fix login bug" --ticket-type bug --priority-id 1 --assignee-id 42
cb ticket notes my-project 123
cb ticket add-note my-project 123 --content "Working on this" --status-id 2
cb ticket watchers my-project 123
cb ticket set-watchers my-project 123 42 43
cb ticket statuses my-project
cb ticket priorities my-project
cb ticket categories my-project
cb ticket types my-project
```

### Milestones

```bash
cb milestone list my-project
cb milestone create my-project "v2.0" --deadline 2026-06-01 --status active
cb milestone update my-project 1 --name "v2.1" --status completed
```

### Activity Feeds

```bash
cb activity account
cb activity account --page 2
cb activity project my-project
cb activity project my-project --since "2026-01-01 00:00:00 +0000" --raw
```

### Status Dashboard

```bash
cb status             # show all projects and recent activity
cb status --json      # machine-readable output
```

### Browse (Open in Browser)

```bash
cb browse my-project           # open project page
cb browse my-project my-repo   # open repository page
cb browse my-project 42        # open ticket #42
cb browse                      # auto-detect project from git remote
```

### Shell Completions

```bash
cb completions bash > ~/.bash_completion.d/cb
cb completions zsh > ~/.zfunc/_cb
cb completions fish > ~/.config/fish/completions/cb.fish
```

## Output

All commands support `--json` for machine-readable JSON output. Human-readable output uses colored text:

- **Status indicators**: green (active/open), yellow (in progress/on hold), red (closed/archived)
- **Priority highlighting**: red bold (critical), red (high), yellow (normal), green (low)
- **Ticket types**: red (bug), cyan (enhancement), blue (task)
- **Merge request status**: green (open), magenta (merged), red (closed)
- **Commit SHAs**: yellow (abbreviated to 7 chars)
- **Branch names**: cyan

## License

MIT
