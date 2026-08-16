# mid

`mid` is a terminal database client written in Rust. It manages database and fast viewing of query results.

> `mid` is currently under active development. 

## Features

- PostgreSQL and MySQL connections.
- TUI-based interactive query-result table.
- Query editing through `$EDITOR`.
- Query history and replay.

All commands, options, examples, TUI controls, and detailed feature explanations
are documented in [FEATURES.md](FEATURES.md).

## Status

| Capability | PostgreSQL | MySQL | SQLite |
| --- | --- | --- | --- |
| Connect and run queries | Working | Working | Planned |
| Interactive table output | Working | Working | Planned |
| JSON output | Working | Working | Planned |
| SQL `INSERT` export | Working | Working | Planned |
| List and select tables | Working | Working | Planned |
| Update selected values | Experimental | Experimental | Planned |

## Requirements

- Rust and Cargo.
- A database connection string (PostgreSQL or MySQL).

## Installation

Build the project:

```sh
cargo build --release
```

Install `mid` into Cargo's binary directory:

```sh
cargo install --path .
```

If you're testing a local build, replace `mid` in the documentation examples with
`cargo run --`:

```sh
cargo run -- --help
```

## Roadmap

### Safe mutation workflow

A dedicated `mutate` command is planned but is **not implemented yet**. The
intended workflow is to run mutations inside a transaction, report the affected
row count, and require explicit confirmation before committing:

```sh
# Planned syntax — not currently available
mid mutate 'DELETE FROM sessions WHERE expires_at < NOW()'
```

The goal is to provide guardrails for `UPDATE`, `DELETE`, `TRUNCATE`, and other
potentially destructive operations.

Other planned work includes:

- SQLite support.
- Durable history storage with SQLite.
- Safer and more general selected-cell updates.
- Local/project-specific remotes.

For the full command reference, see [FEATURES.md](FEATURES.md).
