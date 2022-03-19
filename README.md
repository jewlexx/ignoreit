# Ignore It

## Help

```shell
ignoreit --help

    ignoreit 1.0.0
    Quickly download .gitignore templates for nearly any project

    USAGE:
        ignoreit <SUBCOMMAND>

    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information

    SUBCOMMANDS:
        help    Print this message or the help of the given subcommand(s)
        list    List all available templates
        pull    Pull a template from the repository
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
