# Dotenv plugin for Nushell

Provides the `from dotenv` command, that parse a .env file.

```
open my.env | from dotenv
```

## Install

It is not yet published on cargo, so install it manually:
```sh
cargo install --path . --root ~/.config/nushell/plugins
```

Then register it in the nu config:

```nu
register "~/.config/nushell/plugins/bin/nu_plugin_dotenv"
```
