# Dotenv plugin for Nushell

Crate with a simple example of the Plugin trait that needs to be implemented
in order to create a binary that can be registered into nushell declaration list

## Install

    # from local
    cargo install --path . --root ~/.config/nushell/plugins

    # from git
    cargo install --git https://github.com/enerdgumen/nu_plugin_dotenv

    register "nu_plugin_dotenv"

## Uninstall
    
    cargo uninstall nu_plugin_dotenv
