# Ignore It

[![Rust](https://github.com/jewlexx/ignoreit/actions/workflows/rust.yml/badge.svg?branch=trunk)](https://github.com/jewlexx/ignoreit/actions/workflows/rust.yml)

## Installation

[![Packaging status](https://repology.org/badge/vertical-allrepos/ignoreit.svg)](https://repology.org/project/ignoreit/versions)

If you do not have access to these repositories you can install from crates.io:

```shell
cargo install ignoreit
```

## Help

```shell
ignoreit

Quickly load .gitignore templates

USAGE:
   ignoreit [FLAGS] <COMMAND> [ARGUMENTS]

FLAGS:
   -h, --help     Shows the help message
   -V, --version  Show version

COMMANDS:
   help       Shows the help message
   list       List all available templates
   pull       Pull a template from the repository
   purge      Purge gitignore cache

Thank you for using ignoreit by Juliette Cordor
```

## Usage

```shell
# List all possible templates
ignoreit list

# Pull the rust template
ignoreit pull rust

# Pull the rust template and save it to a custom location
ignoreit pull rust rust.ignore
```

**Developed with ❤️ by Juliette**
