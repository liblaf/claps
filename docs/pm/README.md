# Command-Line Help for `pm`

This document contains the help content for the `pm` command-line program.

**Command Overview:**

- [`pm`↴](#pm)
- [`pm search`↴](#pm-search)
- [`pm complete`↴](#pm-complete)

## `pm`

Logging flags to `#[command(flatte)]` into your CLI

**Usage:** `pm [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `search` —
- `complete` — Generate tab-completion scripts for your shell

###### **Options:**

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

## `pm search`

**Usage:** `pm search [SEARCH]`

###### **Arguments:**

- `<SEARCH>`

  Default value: ``

## `pm complete`

Generate tab-completion scripts for your shell

**Usage:** `pm complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
