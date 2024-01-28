# Command-Line Help for `pm`

This document contains the help content for the `pm` command-line program.

**Command Overview:**

- [`pm`↴](#pm)
- [`pm complete`↴](#pm-complete)

## `pm`

**Usage:** `pm [OPTIONS] [SEARCH] [COMMAND]`

###### **Subcommands:**

- `complete` — Generate tab-completion scripts for your shell

###### **Arguments:**

- `<SEARCH>`

###### **Options:**

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

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
