# mid features and command reference

This document explains every user-facing `mid` command, output format, and
interactive table control.

## Command overview

```text
mid --help
mid remote list
mid remote add <CONNECTION_STRING> [--name <NAME>]
mid remote remove <NAME>
mid remote switch <NAME>
mid status
mid list [--table-name <TABLE_NAME>] [--output-format <FORMAT>]
mid query [QUERY] [--output-format <FORMAT>] [--id <ID>]
mid query last [--skip <COUNT>] [--output-format <FORMAT>]
mid history list
mid history last
```

Use `mid <command> --help` to inspect the options supported by the installed
version.

## Quick start

Add a connection and give it a memorable name:

```sh
mid remote add 'postgres://user:password@localhost/app' --name local-app
```

Activate it and confirm the selection:

```sh
mid remote switch local-app
mid status
```

Run a query:

```sh
mid query 'SELECT * FROM users LIMIT 20'
```

## Remote commands

Connections are called "remotes." The active remote is used by `query` and
`list`.

### Add a remote

```sh
mid remote add <CONNECTION_STRING> [--name <NAME>]
```

Examples:

```sh
mid remote add 'postgres://user:password@localhost/app' --name app
mid remote add 'mysql://user:password@localhost/app' --name app-mysql
```

If `--name` is omitted, `mid` generates a name. Adding a remote does not activate
it; use `remote switch` afterward.

### List remotes

```sh
mid remote list
```

Prints the names of all configured remotes.

### Switch the active remote

```sh
mid remote switch app
```

Subsequent query and table commands use this remote.

### Remove a remote

```sh
mid remote remove app-mysql
```

The active remote cannot be removed. Switch to another remote first.

### Show status

```sh
mid status
```

Prints the name of the active remote.

## Configuration

Remote configuration is stored globally in:

```text
~/.midconfig.toml
```

Connection strings are currently stored as plain text. Protect this file with
appropriate filesystem permissions and do not commit or share it.

## Query commands

### Run a query

The default output is the interactive table:

```sh
mid query 'SELECT id, email FROM users'
```

Select another output format with `--output-format`:

```sh
mid query 'SELECT id, email FROM users' --output-format json
mid query 'SELECT id, email FROM users' --output-format sql
```

### Rerun a query by history ID

```sh
mid query --id 42
mid query --id 42 --output-format json
```

The ID can be found with `mid history list`.

### Rerun a recent query

```sh
# Latest query
mid query last

# Previous query
mid query last --skip 1

# Third newest query
mid query last --skip 2

# Replay with JSON output
mid query last --skip 1 --output-format json
```

`--skip 0` means the latest query, `--skip 1` means the previous query, and so
on. `-s` is the short form of `--skip`.

## Output formats

The `query` and `list` commands support these formats:

| Format | Description |
| --- | --- |
| `table` | Interactive terminal table and the default format. |
| `json` | Pretty-printed JSON array. |
| `sql` | Multi-row `INSERT` statement generated from the result set. |

SQL export supports PostgreSQL and MySQL. It expects a query with a recognizable
`FROM <table>` clause and is intended for straightforward table queries rather
than arbitrary joins or derived tables.

## Interactive table

The table UI is used by query results and interactive table listing.

| Key | Action |
| --- | --- |
| `j` / `k` | Select the next or previous row. |
| `h` / `l` | Select the previous or next column. |
| `Home` / `G` | Select the first or last visible row. |
| `g` | Open the go-to-line popup. |
| `f` | Filter rows using the selected column. |
| `Enter` | Expand a value, or select a table in table-list mode. |
| `y` | Copy the selected value. |
| `e` | Expand or collapse the displayed query. |
| `E` | Edit the query using `$EDITOR`. |
| `u` | Prepare an update for the selected value (experimental). |
| `q` | Quit. |

### Go to line

Press `g`, enter a displayed line number, and press `Enter`. Use `Backspace` to
edit the input. `Esc` or `q` closes the popup without moving.

### Filter

Select a column, press `f`, enter text, and press `Enter`. Matching is
case-insensitive and applies only to the selected column. Submit an empty filter
to restore all rows. `Esc` cancels without changing the current table.

### Copy

Press `y` to copy the full selected value. A successful copy briefly highlights
that exact cell in green. Clipboard support depends on the desktop or terminal
environment.

### Expand values

Press `Enter` in query-result mode to expand or collapse the selected value.
Table previews may be shortened for performance; expansion and copying use the
full value.

### Edit a query

Press `E` to open the query in the editor configured by `$EDITOR`. Saving and
closing the editor reruns the edited query.

`$EDITOR` is required, and its executable must be available in `PATH`:

```sh
export EDITOR=nvim
mid query 'SELECT * FROM users'
```

You can also set it for one invocation:

```sh
EDITOR=nvim mid query 'SELECT * FROM users'
```

### Update a selected value

Press `u` to prepare an update for the selected cell. This feature is
experimental and currently depends on the expected identifier-column behavior.
Review the generated query carefully before executing it.

## List command

Open an interactive list of database tables:

```sh
mid list
```

Select a table with `j`/`k` and `Enter` to open its rows.

Query a table directly:

```sh
mid list --table-name users
mid list -t users
```

Choose an output format:

```sh
mid list --table-name users --output-format json
mid list --table-name users --output-format sql
```

## History commands

Executed queries are recorded with an ID, timestamp, SQL text, and remote name.

### List history

```sh
mid history list
```

### Display the latest entry

```sh
mid history last
```

History is currently stored in the operating system's temporary directory as:

```text
.midhistory.toml
```

It should not be treated as permanent storage.

## Help commands

```sh
mid --help
mid remote --help
mid query --help
mid query last --help
mid list --help
mid history --help
```
