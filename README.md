# builder-rust

A simple builder for base16 templates and schemes.

**NOTE**: This currently implements version 0.9.1 of the [base16 spec](https://github.com/tinted-theming/home/blog/main/builder.md), which is very outdated. We are currently working on the builder and it will soon implement the latest version of the spec. If you want to use a builder, use [builder-go] our most up to date builder.

## Basic Usage

```shell
builder-rust sync
builder-rust build path/to/base16-template
```

## Commands

The following is a table of the available subcommands for the CLI tool (Tinty), including the descriptions and any notable arguments.

| Subcommand | Description                          | Arguments            | Example Usage                              |
|------------|--------------------------------------|----------------------|--------------------------------------------|
| `sync`  | Installs and or updates latest schemes. | - | `builder-rust sync` |
| `build`    | Builds the themes of a template. | `template_path`: Path to template directory. | `builder-go build ./path/to/base16-template` |

## Flags

| Flag/Option       | Description                             | Applicable Subcommands | Default Value | Example Usage                             |
|-------------------|-----------------------------------------|------------------------|---------------|-------------------------------------------|
| `--schemes-dir` `-s`   | Path to a custom local schemes directory to use when building. Only necessary if the [latest schemes repository] is not desired. | `build` | - |
| `--data-dir` `-d`   | Specifies a custom path for the data directory. | All | Linux: `$XDG_DATA_HOME/tinted-theming/tinty` or `~/.local/share`. MacOS: `~/Library/Application\ Support/tinted-theming/tinty` | `builder-go sync --data-dir /path/to/custom/data-dir` |
| `--help` `-h`     | Displays help information for the subcommand. | All | - | `tinty --help`, `tinty apply --help`, etc |
| `--version` `-V`  | Shows the version of tinty. | All | - | `tinty --version` |

[latest schemes repository]: https://github.com/tinted-theming/schemes
[home repository]: https://github.com/tinted-theming/home
[builder-go]: https://github.com/tinted-theming/base16-builder-go
