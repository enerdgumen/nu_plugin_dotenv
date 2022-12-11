# Dotenv plugin for Nushell

Provides the `from dotenv` command, that parse a .env file.

```nu
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

## Alternatives

You can achieve a similar result using [direnv](https://direnv.net):

```nu
direnv dotenv json my.env | from json
```
