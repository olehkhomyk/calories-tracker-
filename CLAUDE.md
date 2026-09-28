# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A minimal REST API boilerplate built on Axum 0.8 + Tokio, using Rust edition 2024 (requires Rust 1.85 or newer). It is at an early stage: a single binary crate with no tests, database, config loading or shared state yet.

## Commands

```sh
cargo run                          # serves on 0.0.0.0:8080
curl localhost:8080/api/v1/users   # smoke-test the only endpoint

cargo check
cargo clippy --all-targets
cargo test                         # all tests
cargo test <name_substring>        # a single test (filter by name)
```

`bacon.toml` is set up for the [bacon](https://dystroy.org/bacon/) watcher:
- `bacon`: runs `cargo check` on every change (the default job)
- `bacon clippy-all`, also bound to the `c` key
- `bacon test -- <filter>` runs one test
- `bacon run-long` restarts the server on every change (`kill_then_restart`)
- `bacon nextest` needs `cargo-nextest` installed

## Architecture

Everything is in `src/main.rs`:
- `main` builds the `Router`, binds the listener, then calls `axum::serve`. Bind and serve failures `unwrap()` and there is no error type yet.
- Every route is prefixed with the `BASER_API_URL` constant (`"/api/v1"`), which is joined through `format!` when the route is registered. Register new routes the same way so they share the prefix.
- Handlers are free `async fn`s that return `axum::Json<T>`. Response models derive `serde::{Serialize, Deserialize}`.
- `get_users` returns hardcoded data. There is no persistence layer or `State` yet.

## Conventions

- The source is indented with **tabs**, and there is no `rustfmt.toml`. Running `cargo fmt` would rewrite every file to 4-space indentation. Either keep editing with tabs, or add a `rustfmt.toml` with `hard_tabs = true` before you run the formatter.
