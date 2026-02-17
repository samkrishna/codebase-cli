# codebase-cli

A command-line utility implementation of the [CodebaseHQ](https://www.codebasehq.com/) API.

## Installation

```
cargo install --path .
```

This installs the `cb` binary.

## Configuration

Set these environment variables with your CodebaseHQ credentials:

```
export CODEBASE_ACCOUNT=your-account
export CODEBASE_USERNAME=your-username
export CODEBASE_API_KEY=your-api-key
```

## Usage

```
cb <command> <subcommand> [options]
```

### Projects

```bash
cb project list
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
cb repo branches my-project my-repo
cb repo commits my-project my-repo main
cb repo commits my-project my-repo main --path src/lib.rs
cb repo file my-project my-repo main README.md
cb repo hooks my-project my-repo
cb repo create-hook my-project my-repo https://example.com/webhook --username user --password pass
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

## License

MIT
