
This document contains all the things the application needs to do.

# mid

`mid` is a Rust command-line tool for connecting to configured database
remotes and running ad-hoc queries from the terminal.

## Features

- Run ad-hoc database queries from the terminal.
- View query results as an interactive table or formatted JSON.
- Manage saved database connections: add, remove, list, and switch remotes.
- Connect to PostgreSQL databases.
- Recognize MySQL connections for planned adapter support.

## Feature Status

| Feature | Status |
| --- | --- |
| Query table view | Working |
| Query JSON output | Working |
| Remote add | Working |
| Remote remove | Working |
| Remote list | Working |
| Remote switch | Working |

## Database Support

| Database | Status |
| --- | --- |
| PostgreSQL | Working |
| MySQL | Planned |
| SQLite | Planned |

## Requirements

- Rust toolchain with Cargo
- Any supported database server (PostgreSQL, MySQL, SQLite, etc.)

## Install

From the project root:

```sh
cargo build
```

For local development, run commands through Cargo:

```sh
cargo run -- --help
```

To install the binary into your Cargo bin directory:

```sh
cargo install --path .
```

## Remote

Use remote to connect to some database servers. Removes can either be global or local.

Remotes are the actual server, but the user can change the current database with `database` command.

Even this command should return the actual database

```sh
mid remote status
# connected to my_server:my_application_db_dev
```
### Sugestions

We recommend adding `.mid_config.toml` into your global .gitignore file to prevent commiting that files to git.

### Local Remotes

Local remotes are stored into a `.mid_config.toml` file 

### Global Remotes

## Query Command

Query command runs the query on the **Current Connection**. 

Run a query directily from the CLI.

```sh
mid query 'SELECT * FROM users';
```

 Run queries from the STDIN.
 ```sh
cat 'SELECT * FROM users' | mid query
# or
cat my_query.sql | mid query
 ```

## Mutate command

Mutate commands are made for mutating the database, inserts, deletes or updates.

Those commands are inits transations by default, and only commits when the user explicit writes `confirm` on the terminal. (Just like `terraform apply` command).

This add guardrails to users dont delete or update in batch, which is dangerous.

```sh
mid mutate 'DELETE FROM users;'
# output:
# 9421 line afftected, type `yes` to apply
# > 
```

Or with STDIN

 ```sh
cat 'DELETE FROM users;' | mid mutate
# or
cat my_truncate.sql | mid mutate
 ```
 Remote is a subcommands for user manage which server it will consume.
 
 ## Commands
 
 **ADD**
 
 The `add` subcommand will insert a new server connection, on local directory or a global one using the `--global` flag.
 
 Once a remote is add, it will automatically be **active**, the database on the connection string will be used as default. But can be switched using the `database` commands.
 
 - **Usage**
 ```sh
 # --global is optional, local is the default
 mid remote add 'postgres://etc' --global
 ```
 
 When using local options, a `.mid_config.toml` file will be created on current working directory.
 
 That file will contain the server information as the encrypted password.
 
 **LS**
 
 Ls sub command list the available remotes, either global and local ones.
 
 - **Usage**
 ```sh
 mid remote ls
 ```
 
 **STATUS**
