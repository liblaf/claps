# Command-Line Help for `gfw`

This document contains the help content for the `gfw` command-line program.

**Command Overview:**

- [`gfw`↴](#gfw)
- [`gfw api`↴](#gfw-api)
- [`gfw api proxy`↴](#gfw-api-proxy)
- [`gfw api proxy get`↴](#gfw-api-proxy-get)
- [`gfw api proxy set`↴](#gfw-api-proxy-set)
- [`gfw complete`↴](#gfw-complete)
- [`gfw ip`↴](#gfw-ip)
- [`gfw sub`↴](#gfw-sub)
- [`gfw sub list`↴](#gfw-sub-list)
- [`gfw sub update`↴](#gfw-sub-update)

## `gfw`

Logging flags to `#[command(flatte)]` into your CLI

**Usage:** `gfw [OPTIONS] <COMMAND>`

###### **Subcommands:**

- `api` —
- `complete` — Generate tab-completion scripts for your shell
- `ip` —
- `sub` —

###### **Options:**

- `-v`, `--verbose` — Increase logging verbosity
- `-q`, `--quiet` — Decrease logging verbosity

## `gfw api`

**Usage:** `gfw api <COMMAND>`

###### **Subcommands:**

- `proxy` —

## `gfw api proxy`

**Usage:** `gfw api proxy <COMMAND>`

###### **Subcommands:**

- `get` —
- `set` —

## `gfw api proxy get`

**Usage:** `gfw api proxy get [OPTIONS]`

###### **Options:**

- `-u`, `--url <URL>`

  Default value: `http://127.0.0.1:9090`

- `-s`, `--secret <SECRET>`

## `gfw api proxy set`

**Usage:** `gfw api proxy set [OPTIONS] [NAME]`

###### **Arguments:**

- `<NAME>`

###### **Options:**

- `-u`, `--url <URL>`

  Default value: `http://127.0.0.1:9090`

- `-s`, `--secret <SECRET>`

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
