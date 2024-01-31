# Command-Line Help for `alist-cli`

This document contains the help content for the `alist-cli` command-line program.

**Command Overview:**

- [`alist-cli`↴](#alist-cli)
- [`alist-cli complete`↴](#alist-cli-complete)
- [`alist-cli list`↴](#alist-cli-list)
- [`alist-cli upload`↴](#alist-cli-upload)
- [`alist-cli remove`↴](#alist-cli-remove)

## `alist-cli`

**Usage:** `alist-cli [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `complete` — Generate tab-completion scripts for your shell
- `list` —
- `upload` —
- `remove` —

###### **Options:**

- `--url <URL>`

  Default value: `https://alist.liblaf.me/api`

- `-u`, `--username <USERNAME>`
- `-p`, `--password <PASSWORD>`
- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

## `alist-cli complete`

Generate tab-completion scripts for your shell

**Usage:** `alist-cli complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `alist-cli list`

**Usage:** `alist-cli list [OPTIONS] [PATH]`

###### **Arguments:**

- `<PATH>`

  Default value: `/`

###### **Options:**

- `-d`, `--depth <DEPTH>`

  Default value: `1`

- `-r`, `--refresh`

  Possible values: `true`, `false`

## `alist-cli upload`

**Usage:** `alist-cli upload [OPTIONS] <PATH>`

###### **Arguments:**

- `<PATH>`

###### **Options:**

- `--prefix <PREFIX>`

  Default value: `/public/img`

- `--no-refresh`

  Possible values: `true`, `false`

## `alist-cli remove`

**Usage:** `alist-cli remove [OPTIONS] [PATH]...`

###### **Arguments:**

- `<PATH>`

###### **Options:**

- `--no-refresh`

  Possible values: `true`, `false`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
