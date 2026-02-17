# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`cb` — a command-line utility wrapping the CodebaseHQ API (https://api3.codebasehq.com). Written in Rust (edition 2024), licensed MIT.

## Commands

- `cargo build` — compile
- `cargo run --bin cb -- <args>` — run the CLI
- `cargo test` — run all 55 tests
- `cargo test --test projects_tests` — run a single test file
- `cargo test test_list_tickets` — run a single test by name
- `cargo clippy` — lint
- `cargo fmt` — format code
- `cargo fmt --check` — check formatting

## Architecture

```
src/
  main.rs              — CLI entry point, clap parser, env-var config, command dispatch
  lib.rs               — Exposes `api` module for integration tests
  api/
    client.rs          — CodebaseClient: HTTP client with Basic auth, XML headers, GET/POST/PUT/DELETE
    models.rs          — All serde data models (Project, Repository, Ticket, Milestone, Event, etc.)
    projects.rs        — Projects API functions (8): list, show, create, update, delete, groups, users, assign
    repositories.rs    — Repositories API functions (4): list, show, create, delete
    tickets.rs         — Tickets API functions (11): list, search, create, notes, add-note, watchers, set-watchers, statuses, priorities, categories, types
    milestones.rs      — Milestones API functions (3): list, create, update
    activity.rs        — Activity Feed API functions (2): account feed, project feed
  commands/
    projects.rs        — `cb project` subcommands → calls api::projects
    repositories.rs    — `cb repo` subcommands → calls api::repositories
    tickets.rs         — `cb ticket` subcommands → calls api::tickets
    milestones.rs      — `cb milestone` subcommands → calls api::milestones
    activity.rs        — `cb activity` subcommands → calls api::activity
tests/
  projects_tests.rs    — 14 mock tests (mockito)
  repositories_tests.rs — 10 mock tests
  tickets_tests.rs     — 12 mock tests
  milestones_tests.rs  — 10 mock tests
  activity_tests.rs    — 9 mock tests
```

## Key Patterns

- **Two-layer design**: `api/` contains pure API logic (returns deserialized structs), `commands/` handles CLI arg parsing and output formatting. Each command module calls into its corresponding API module.
- **XML API**: CodebaseHQ uses XML with kebab-case field names. Deserialization uses `quick-xml` + `serde`. Models use `#[serde(rename = "kebab-name")]` for field mapping.
- **Wrapper types for lists**: XML list responses need a wrapper struct (e.g., `Projects { projects: Vec<Project> }`) because quick-xml deserializes the root element.
- **All fields are `Option<T>`**: API responses may omit fields; models use Options throughout.
- **Auth via env vars**: `CODEBASE_ACCOUNT`, `CODEBASE_USERNAME`, `CODEBASE_API_KEY` (no config file yet).
- **Testing with mockito**: Integration tests use `mockito::Server::new_async()` and `CodebaseClient::with_base_url()` to point at mock servers.

## Key Dependencies

- **clap** (derive) — CLI parsing with subcommands
- **reqwest** (json) — HTTP client
- **quick-xml** (serialize) — XML deserialization
- **serde** / **serde_json** — serialization
- **tokio** (full) — async runtime
- **anyhow** — error handling
- **mockito** (dev) — HTTP mocking for tests
