# coral-cli

`coral-cli` is a command-line tool for interfacing with
Puffer's smart contracts as well as Secure-Signer enclaves

## Configuration

`coral-cli` can be configured via `cli.toml` to reduce the
amount of command line arguments needed.

`coral-cli` will look in the following directories for a `cli.toml` file:
 - `CORAL_CONFIG_DIR` environment variable
 - `~/.config/puffer/coral`

See [cli.toml.md](/coral-cli/docs/cli.toml.md) for more details

See [cli.toml](/coral-cli/config/cli.toml) for example config

## Subcommands

#### `help`
Show help

#### `version`
Prints `coral-cli` version

#### `print-config`
Prints `cli.toml` config.
Use this to make sure `coral-cli` is able to
read and understand the config.

#### `completions [SHELL]`
Output shell code necessary for a given shell to
auto-complete `coral-cli` commands

#### `validator`

See [validator](/coral-cli/docs/validator.md)

#### `guardian`

See [guardian](/coral-cli/docs/guardian.md)
