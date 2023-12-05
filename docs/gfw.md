# Command-Line Help for `gfw`

This document contains the help content for the `gfw` command-line program.

**Command Overview:**

- [`gfw`↴](#gfw)
- [`gfw complete`↴](#gfw-complete)
- [`gfw ip`↴](#gfw-ip)
- [`gfw sub`↴](#gfw-sub)
- [`gfw sub list`↴](#gfw-sub-list)
- [`gfw sub update`↴](#gfw-sub-update)

## `gfw`

**Usage:** `gfw [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `complete` — Generate tab-completion scripts for your shell
- `ip` —
- `sub` —

###### **Options:**

- `-v`, `--verbose` — More output per occurrence
- `-q`, `--quiet` — Less output per occurrence

## `gfw complete`

Generate tab-completion scripts for your shell

**Usage:** `gfw complete <SHELL>`

###### **Arguments:**

- `<SHELL>`

  Possible values: `markdown`, `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `gfw ip`

**Usage:** `gfw ip [OPTIONS]`

###### **Options:**

- `-t`, `--timeout <TIMEOUT>`

  Default value: `3`

## `gfw sub`

**Usage:** `gfw sub <COMMAND>`

###### **Subcommands:**

- `list` —
- `update` —

## `gfw sub list`

**Usage:** `gfw sub list [OPTIONS]`

###### **Options:**

- `--folder <FOLDER>`

  Default value: `the Great Wall`

- `-f`, `--fields <FIELDS>`

  Default values: `name`, `download`, `upload`, `total`, `expire`

  Possible values: `name`, `url`, `download`, `upload`, `total`, `expire`

## `gfw sub update`

**Usage:** `gfw sub update [OPTIONS]`

###### **Options:**

- `--folder <FOLDER>`

  Default value: `the Great Wall`

- `-a`, `--api <API>`

  Default value: `https://gfw.liblaf.me`

- `-c`, `--config <CONFIG>`

  Default value: `/etc/sing-box/config.json`

<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
