# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`cb` — a command-line utility wrapping the CodebaseHQ API (https://api3.codebasehq.com). Written in Rust (edition 2024), licensed MIT.

## Commands

- `cargo build` — compile
- `cargo run --bin cb -- <args>` — run the CLI
- `cargo test` — run all tests
- `cargo test --test projects_tests` — run a single test file
- `cargo test test_list_tickets` — run a single test by name
- `cargo clippy` — lint
- `cargo fmt` — format code
- `cargo fmt --check` — check formatting

## Architecture

```
src/
  main.rs              — CLI entry point, clap parser, credential-based config, command dispatch
  lib.rs               — Exposes api, git_context, output modules for integration tests
  output.rs            — JSON/colored output formatting helpers (print_output, print_list, colorize_*)
  git_context.rs       — Auto-detect project/repo from git remote URL (SSH and HTTPS)
  api/
    client.rs          — CodebaseClient: HTTP client with Basic auth, XML headers, retry with exponential backoff
    config.rs          — Credential storage at ~/.config/cb/config.toml (api_username, api_key)
    models.rs          — All serde data models with custom deserializers for empty-string nullable XML fields
    projects.rs        — Projects API functions (8): list, show, create, update, delete, groups, users, assign
    repositories.rs    — Repositories API functions (18): list, show, create, delete, commits, deployments, files, hooks, branches, merge requests
    tickets.rs         — Tickets API functions (11): list, search, create, notes, add-note, watchers, set-watchers, statuses, priorities, categories, types
    milestones.rs      — Milestones API functions (3): list, create, update
    activity.rs        — Activity Feed API functions (2): account feed, project feed
  commands/
    projects.rs        — `cb project` subcommands → calls api::projects
    repositories.rs    — `cb repo` subcommands → calls api::repositories
    tickets.rs         — `cb ticket` subcommands → calls api::tickets
    milestones.rs      — `cb milestone` subcommands → calls api::milestones
    activity.rs        — `cb activity` subcommands → calls api::activity
    status.rs          — `cb status` dashboard (projects + recent activity)
    browse.rs          — `cb browse` opens project/repo/ticket in web browser
    pr.rs              — `cb pr` shorthand for merge request commands with git context auto-detection
```

## Key Patterns

- **Two-layer design**: `api/` contains pure API logic (returns deserialized structs), `commands/` handles CLI arg parsing and output formatting. Each command module calls into its corresponding API module.
- **XML API**: CodebaseHQ uses XML with kebab-case field names. Deserialization uses `quick-xml` + `serde`. Models use `#[serde(rename = "kebab-name")]` for field mapping.
- **Custom deserializers**: CodebaseHQ returns empty strings for null integer/float/bool fields (e.g. `<group-id></group-id>`). Custom `deserialize_optional_i64`, `deserialize_optional_f64`, `deserialize_optional_bool` functions handle this.
- **Wrapper types for lists**: XML list responses need a wrapper struct (e.g., `Projects { projects: Vec<Project> }`) because quick-xml deserializes the root element.
- **All fields are `Option<T>`**: API responses may omit fields; models use Options throughout.
- **Dual output**: All commands support `--json` (global flag) for JSON output via `output::print_output`/`output::print_list`, with colored human-readable output as default.
- **Contextual defaults**: `cb pr` commands auto-detect project/repo from git remote via `git_context::detect()`. The `resolve_project_repo()` helper falls back to git context when args are omitted.
- **Auth via config file**: Credentials stored at `~/.config/cb/config.toml`. API username format: `account/username`.
- **Retry logic**: Exponential backoff (1s, 2s, 4s, 8s, 16s) on HTTP 429, 503, 529.
- **Testing with mockito**: Integration tests use `mockito::Server::new_async()` and `CodebaseClient::with_base_url()` to point at mock servers.

## Key Dependencies

- **clap** (derive) — CLI parsing with subcommands
- **clap_complete** — Shell completion generation (bash/zsh/fish)
- **colored** — Terminal color output
- **open** — Open URLs in default browser
- **reqwest** (json) — HTTP client
- **quick-xml** (serialize) — XML deserialization
- **serde** / **serde_json** — serialization
- **tokio** (full) — async runtime
- **anyhow** — error handling
- **dirs** — Home directory detection
- **toml** — Config file parsing
- **mockito** (dev) — HTTP mocking for tests
