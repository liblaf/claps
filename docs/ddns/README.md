# NAME

ddns - CLIs written in Rust

# SYNOPSIS

**ddns** \[**-n**\|**\--name**\] \[**-t**\|**\--token**\]
\[**-z**\|**\--zone**\] \[**-v**\|**\--verbose**\]\...
\[**-q**\|**\--quiet**\]\... \[**-h**\|**\--help**\]
\[**-V**\|**\--version**\] \<_subcommands_\>

# DESCRIPTION

CLIs written in Rust

# OPTIONS

**-n**, **\--name**=_NAME_

:

    May also be specified with the **NAME** environment variable.

**-t**, **\--token**=_TOKEN_

:

    May also be specified with the **TOKEN** environment variable.

**-z**, **\--zone**=_ZONE_ \[default: 919b04037636d3b4bbc0af135eaccdfa\]

:

    May also be specified with the **ZONE** environment variable.

**-v**, **\--verbose**

: Increase logging verbosity

**-q**, **\--quiet**

: Decrease logging verbosity

**-h**, **\--help**

: Print help

**-V**, **\--version**

: Print version

# SUBCOMMANDS

ddns-complete(1)

:

ddns-delete(1)

:

ddns-install(1)

:

ddns-list(1)

:

ddns-update(1)

:

ddns-help(1)

: Print this message or the help of the given subcommand(s)

# VERSION

v0.1.0

# AUTHORS

liblaf \<i@liblaf.me\>
