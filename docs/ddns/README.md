# Command-Line Help for `ddns`

This document contains the help content for the `ddns` command-line program.

**Command Overview:**

- [`ddns`↴](#ddns)
- [`ddns complete`↴](#ddns-complete)
- [`ddns delete`↴](#ddns-delete)
- [`ddns install`↴](#ddns-install)
- [`ddns list`↴](#ddns-list)
- [`ddns update`↴](#ddns-update)

## `ddns`

**Usage:** `ddns [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `complete` — Generate tab-completion scripts for your shell
- `delete` —
- `install` —
- `list` —
- `update` —

###### **Options:**

- `-v`, `--verbose` — More output per occurrence
- `-q`, `--quiet` — Less output per occurrence

## `ddns complete`

Generate tab-completion scripts for your shell

**Usage:** `ddns complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `ddns delete`

**Usage:** `ddns delete [OPTIONS]`

###### **Options:**

- `-n`, `--name <NAME>`
- `-t`, `--token <TOKEN>`
- `-z`, `--zone <ZONE>`

  Default value: `919b04037636d3b4bbc0af135eaccdfa`

## `ddns install`

**Usage:** `ddns install`

## `ddns list`

**Usage:** `ddns list [OPTIONS]`

###### **Options:**

- `-n`, `--name <NAME>`
- `-t`, `--token <TOKEN>`
- `-z`, `--zone <ZONE>`

  Default value: `919b04037636d3b4bbc0af135eaccdfa`

## `ddns update`

**Usage:** `ddns update [OPTIONS]`

###### **Options:**

- `-n`, `--name <NAME>`
- `-t`, `--token <TOKEN>`
- `-z`, `--zone <ZONE>`

  Default value: `919b04037636d3b4bbc0af135eaccdfa`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
